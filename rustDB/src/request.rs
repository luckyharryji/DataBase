#[doc = "
	
	Accepted: parameter: 
	/**
	Done:
		PUTLIST
		@Arguments:
			PUTLIST CollectionName
			Attributes1
			Attributes2
			...
		@Purpose: Create a new collection in the database with the given attributes

		DELETELIST
		@Arguments: 
			DELETELIST CollectionName
		@Purpose: Delete an entry from the data store

		GETLIST
		@Arguments:
			GETLIST CollectionName
		@Purpose: Retrieve content of the desired collection

		APPEND
		@Arguments: 
			APPEND CollectionName
			Key Value
			...
		@Purpose: Add an element to an existing list in the data store

		UPDATE
		@Arguments: 
			UPDATE CollectionName
			Key Value;Key Value;...	// parse condition
			Key Value;...	//update value
		@Purpose: Update existing item in the databse

	On Going:


	To Do:

		GET
		Arguments: Key
		Purpose: Retrieve a stored value from the data store

		APPEND
		Arguments: Key, Value
		Purpose: Add an element to an existing list in the data store
	**/
"]

use std::net::TcpStream;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;
use std::fs::OpenOptions;
use std::sync::{Arc,Mutex};
use std::convert::AsRef;

use rustc_serialize::json;

use std::collections::{HashMap, BTreeSet};
type Set<K> = BTreeSet<K>;

use response::Response;
use lib::{get_file_content,write_into_file};

// defind request structure
pub struct Request{
	url: String,
	stream: TcpStream,
	command: String,
	request_info: String,
	request_collection: String,
	request_parameter: Vec<String>,
	not_find_page: String,
}


impl Request{
	pub fn new(mut stream:TcpStream)->Self{
		let mut http_reader = BufReader::new(stream);
		let mut log_request_info = String::new();
		
		let mut header = String::new();
		let mut http_info = Vec::<&str>::new();
		
		// first get the request file from first line of the stream
		match http_reader.read_line(&mut header).unwrap()>0{
			true=> {
				print!("info is: {}",header);
				http_info= header.split_whitespace().collect();
			},
			false =>{
				println!("Request Error");
			},
		}
		log_request_info.push_str(&header);


		let mut parameter = Vec::new();
		let mut read_stream_info = String::new();		
		while http_reader.read_line(&mut read_stream_info).unwrap()>0{
			if read_stream_info == "\r\n".to_owned(){   // since TcpStream is a long connection, have to jump out when 
				break;									// read to the last line \r\n , or it will stall the network connection
			}
			let record = read_stream_info.to_owned();
			log_request_info.push_str(&record);

			// remove \r, \n
			parameter.push(read_stream_info.clone().trim().to_owned());
			read_stream_info.clear();
		}


		let command = http_info[0].to_owned();
		let col_name = http_info[1].to_owned();

		let file_source = http_info[0];					// source of the request file
		stream = http_reader.into_inner();
		let mut file_addr = String::from("./");
		file_addr.push_str(file_source);

		Request{
			url: file_addr,
			stream: stream,
			command: command,
			request_info: log_request_info,
			request_collection: col_name,
			request_parameter: parameter,
			not_find_page: ".//404.html".to_owned(),
		}
	}

	/**exposed public function**/
	// record request time and all the request info into log
	pub fn record_log(&mut self,time:&str, write_log_file: &Arc<Mutex<OpenOptions>>){
		let format_log = "Request Time: ".to_owned()+time+"\r\n"+&self.request_info+"\r\n";
		match write_into_file(&format_log,write_log_file){
			Err(_)=>println!("Failed to record request logs"),
			Ok(_) => println!("Request Log Recorded"),
		}
	}


	// API to call for create response
	pub fn get_response(&mut self) -> Response{
		self.process_url()
	}

	pub fn get_parameters(&self) -> Set<String>{
		let parameter_set: Set<String> = self.request_parameter.iter().cloned().collect();
		parameter_set
	}

	// get object and desire for update
	pub fn get_object_desired(&self) -> (HashMap<String, String>, HashMap<String, String>){
		let obj_attributes:Vec<&str> = self.request_parameter[0].trim().split(";").collect();
		let desired_attributes:Vec<&str> = self.request_parameter[1].trim().split(";").collect();
		let mut obj_pair = HashMap::<String, String>::new();
		for pair in obj_attributes.iter().clone(){
			if pair.is_empty(){
				break;
			}
			let key_value:Vec<&str> = pair.split_whitespace().collect();
			obj_pair.insert(key_value[0].to_owned(), key_value[1].to_owned());			
		}

		let mut desire_pair = HashMap::<String, String>::new();
		for pair in desired_attributes.iter().clone(){
			if pair.is_empty(){
				break;
			}
			let key_value:Vec<&str> = pair.split_whitespace().collect();
			desire_pair.insert(key_value[0].to_owned(), key_value[1].to_owned());			
		}
		(obj_pair, desire_pair)
	}

	pub fn get_collection(&self) -> String{
		self.request_collection.clone()
	}

	pub fn get_command(&self) -> String{
		self.command.clone()
	}

	pub fn get_attributes(&self) -> HashMap<String, String>{
		let mut key_value_pair = HashMap::<String, String>::new();
		for pair in self.request_parameter.iter().clone(){
			let key_value:Vec<&str> = pair.split_whitespace().collect();
			key_value_pair.insert(key_value[0].to_owned(), key_value[1].to_owned());
		}
		key_value_pair
	}

	
	/**private function**/
	pub fn is_valid(&self) -> bool{
		match self.command.as_ref(){
		    "PUTLIST" => self.request_parameter.len() == 1,
		    "POST" => self.request_parameter.len() == 1,
		    _ =>false,
		}
	}

	// parse url in the reqeust
	// end with / means it could request for content inside a folder
	fn process_url(&mut self)->Response{
		match self.url.ends_with("/"){
			true => return self.parse_dir(),
			false => return self.parse_file(),
		}
	}

	// process if request for folder
	fn parse_dir(&mut self)->Response{
		let file_name = vec!["index.html", "index.shtml", "index.txt"];
		let origin_url = self.url.clone();
		for file in &file_name{
			let mut source_addr = origin_url.to_owned();
			source_addr.push_str(file);

			if let Ok(s) = get_file_content(&Path::new(&source_addr)){
				self.url = source_addr;
				return self.form_response(200, Some(s));
			}
		}
		match get_file_content(&Path::new(&self.not_find_page)){
			Err(_)=>return self.form_response(404, None),
			Ok(s)=>return self.form_response(404,Some(s)),
		}
	}

	// process when request for a file
	fn parse_file(&self)->Response{
		match get_file_content(&Path::new(&self.url)){
			Err(meg) => {
				match meg.kind(){
					ErrorKind::NotFound => {
						match get_file_content(&Path::new(&self.not_find_page)){
							Err(_)=>self.form_response(404, None),
							Ok(s)=>self.form_response(404,Some(s)),
						}
					},
					ErrorKind::PermissionDenied => self.form_response(403, None),
					_ => self.form_response(400, None),
				}
			},
			Ok(s)=>self.form_response(200, Some(s)),
		}
	}

	// create a response from here 
	// Only have conten-type when get a file with code 200
	fn form_response(&self, code:usize,content:Option<String>)->Response{
		let response_file_type = match code{
			200=>{
				match self.url.ends_with(".html"){
					true => Some("html".to_owned()),
					false => Some("plain".to_owned()),
				}
			},
			404=>Some("html".to_owned()),
			_ => None,
		};
		match content{
			Some(content) => {
				let length_of_content = content.len();
				return Response::new(code, Some(length_of_content), Some(content), response_file_type, &self.stream); // xiangyu: rewrite to decide type
			},
			None => Response::new(code, None, None, response_file_type, &self.stream),
		}
	}
}