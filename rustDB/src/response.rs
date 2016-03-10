use std::net::TcpStream;
use std::io::prelude::*;
use std::sync::{Arc,Mutex};
use std::fs::OpenOptions;
use lib::write_into_file;

// define response structure to send back to client
pub struct Response<'a>{
    content: Option<String>,
    stream: &'a TcpStream,      // since repsonse and request share same TcpStream, lifetime should be set here
}

impl <'a>Response<'a>{
    pub fn new(content:Option<String>, stream:&'a TcpStream)->Self{    
        Response{
            content: content,
            stream: stream,
        }
    }

    /**exposed public function**/
    // send response info through TcpStream
    pub fn write_response(&mut self){   
        let response_content = match self.content{
            Some(ref content) => format!("\r\n{}\r\n",content),
            None => "".to_owned(),
        };
        self.write_to_stream(&response_content);
    }

    // write reponse status and time into log
    pub fn record_log(&mut self, time: &str, write_log_file: &Arc<Mutex<OpenOptions>>){
        let mut format_log = "Response Time: ".to_owned()+time+"\r\n";
        if let Some(ref cont) = self.content{
            format_log = format_log.to_owned() + cont + "\r\n\r\n";
        }
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
