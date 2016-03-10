#[doc = "
    
    Accepted: parameter: 
    /**
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
            Key Value;Key Value;...;    // parse condition
            Key Value;...;  //update value
        @Purpose: Update existing item in the databse

        GET
        @Arguments: 
            GET CollectionName
            Key Value
            ...
        Purpose: Retrieve stored value that has the queried key-value

        DELETE
        @Arguments: 
            GET CollectionName
            Key Value
            ...
        Purpose: Deltte stored value that has the queried key-value
    **/
"]

use std::net::TcpStream;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::sync::{Arc,Mutex};
use std::collections::{HashMap, BTreeSet};
type Set<K> = BTreeSet<K>;

use response::Response;
use lib::write_into_file;

// defind request structure
pub struct Request{
    stream: TcpStream,
    command: String,
    request_info: String,
    request_collection: String,
    request_parameter: Vec<String>,
}


impl Request{
    pub fn new(mut stream:TcpStream)->Self{
        let mut http_reader = BufReader::new(stream);
        let mut log_request_info = String::new();
        
        let mut header = String::new();
        let mut http_info = Vec::<&str>::new();
        // parse query type and objective function from first line
        match http_reader.read_line(&mut header).unwrap()>0{
            true=> {
                http_info= header.split_whitespace().collect();
            },
            false =>{
                println!("Request Error");
            },
        }

        log_request_info.push_str(&header);   // record info for log

        let mut parameter = Vec::new();
        let mut read_stream_info = String::new();       
        while http_reader.read_line(&mut read_stream_info).unwrap()>0{
            if read_stream_info == "\r\n".to_owned(){   // since TcpStream is a long connection, have to jump out when 
                break;                                  // read to the last line \r\n , or it will stall the network connection
            }
            let record = read_stream_info.to_owned();
            log_request_info.push_str(&record);
            // remove \r, \n with trim()
            parameter.push(read_stream_info.clone().trim().to_owned());
            read_stream_info.clear();
        }

        let command = http_info[0].to_owned();
        let col_name = http_info[1].to_owned();

        stream = http_reader.into_inner();

        Request{
            stream: stream,
            command: command,
            request_info: log_request_info,
            request_collection: col_name,
            request_parameter: parameter,
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

    // create a response from here 
    pub fn form_response(&self, content:Option<String>)->Response{
        Response::new(content, &self.stream)
    }
}