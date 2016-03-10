use std::net::TcpStream;
use std::io::prelude::*;
use std::sync::{Arc,Mutex};
use std::fs::OpenOptions;

use lib::{get_status_info,write_into_file};

// define response structure to send back to client
pub struct Response<'a>{
    status_code: usize,
    content_length: Option<usize>,
    content: Option<String>,
    file_type: Option<String>,
    stream: &'a TcpStream,      // since repsonse and request share same TcpStream, lifetime should be set here
}

impl <'a>Response<'a>{
    pub fn new(code:usize, content_length: Option<usize>, content:Option<String>, file_type:Option<String>,stream:&'a TcpStream)->Self{
        Response{
            status_code: code,
            content_length: content_length,
            content:content,
            file_type: file_type,
            stream: stream,
        }
    }

    /**exposed public function**/
    // send response info through TcpStream
    pub fn write_response(&mut self)->usize{
        let status_info = get_status_info(self.status_code).to_owned();
        let header = format!("HTTP/1.0 {} {}\r\n", self.status_code, status_info);
        let server_name = "Xiangyu and Nianzu: Rust-Server/0.1\r\n";

        let response_type_info = match self.file_type{
            Some(ref file_type)=>format!("Content-type: text/{}\r\n",file_type.to_owned()),
            None => "".to_owned(),
        };

        let response_content_length_info = match self.content_length{
            Some(length) => format!("Content-length: {}\r\n",length),
            None => "".to_owned(),
        };

        let response_content = match self.content{
            Some(ref content) => format!("\r\n{}\r\n",content),
            None => "".to_owned(),
        };

        let result = header+server_name+&response_type_info+&response_content_length_info+&response_content;
        self.write_to_stream(&result);
        self.status_code
    }

    // write reponse status and time into log
    pub fn record_log(&mut self,time:&str,status_code:usize,write_log_file: &Arc<Mutex<OpenOptions>>){
        let format_log = "Response Time: ".to_owned()+time+"\r\n"+"Response Status Code: "+&status_code.to_string()+"\r\n\r\n";
        match write_into_file(&format_log,write_log_file){
            Err(_)=>println!("Failed to record response logs"),
            Ok(_) => println!("Response Log Recorded"),
        }
    }

    /**private function**/
    // write reponse to TcpStream
    fn write_to_stream(&mut self, content:&str){
        let response_write_content = content.to_owned();
        self.stream.write(response_write_content.as_bytes()).unwrap();
    }
}
