use std::net::TcpStream;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;
use std::fs::OpenOptions;
use std::sync::{Arc,Mutex};

use response::Response;
use lib::{get_file_content,write_into_file};

// defind request structure
pub struct Request{
	url: String,
	stream:TcpStream,
	request_info:String,
	not_find_page:String,
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

		let mut read_stream_info = String::new();
		// get remaining request info
		// while http_reader.read_line(&mut read_stream_info).unwrap()>0{
		// 	if read_stream_info == "\r\n".to_owned(){   // since TcpStream is a long connection, have to jump out when 
		// 		break;									// read to the last line \r\n , or it will stall the network connection
		// 	}
		// 	let record = read_stream_info.to_owned();
		// 	log_request_info.push_str(&record);
		// 	read_stream_info.clear();
		// }

		/*
		parse the database manipulation type with the command type
			PUT: update data item
			POST: insert new value
			GET: retrive the data item
			DELETE: delete one item in database
		*/
		let command_type = http_info[0];
		match command_type {
		    "PUT"=>println!("Call Database Update Function"),
		    "POST"=>println!("Call insert function"),
		    "GET"=>println!("Call find function"),
		    "DELETE"=>println!("Call delete function"),
		    _=>println!("Unsupported manipulation"),
		}
		let command_data = http_info[1];

		let file_source = http_info[1];					// source of the request file
		stream = http_reader.into_inner();
		let mut file_addr = String::from("./");
		file_addr.push_str(file_source);

		Request{
			url: file_addr,
			stream:stream,
			request_info:log_request_info,
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
	pub fn get_response(&mut self)->Response{
		self.process_url()
	}

	
	/**private function**/
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