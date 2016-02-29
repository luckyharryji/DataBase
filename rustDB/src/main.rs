use std::net::{TcpListener,TcpStream};
use std::thread;
use std::sync::{Arc,Mutex};
use std::fs::OpenOptions;

extern crate time;  // import for record time for log

pub mod lib;

mod response;

mod request;
use request::Request;


fn main() {
	initial_bind_server(8080);
}


fn handle_stream(stream:TcpStream,write_log_file: &Arc<Mutex<OpenOptions>>){
	let request_time = time::now().ctime().to_string();    // record time when request come
	let mut request = Request::new(stream);				   // parse the request, extract url and all requet info
	request.record_log(&request_time,write_log_file);					   // write request info into log

	// let mut response = request.get_response();			   // create response structure from request information
	// let reponse_code = response.write_response();		   // send back response to the client
	// let response_time = time::now().ctime().to_string();   // record time when send out response
	// response.record_log(&response_time, reponse_code,write_log_file);     // write request info into log
}


fn initial_bind_server(port:usize){
	// bing server to the localhost
	let bind_addr:&str = &("127.0.0.1:".to_owned()+&port.to_string());
    let listener = TcpListener::bind(bind_addr).unwrap();
    println!("Server Started");

    let file_for_log = Arc::new(Mutex::new(OpenOptions::new()));
    for stream in listener.incoming() {
    	let log_file_for_write = file_for_log.clone();
		match stream{
			Ok(stream)=>{  				
				thread::spawn(move || {  // spawn a thread for each request 
					handle_stream(stream,&log_file_for_write);
				});
			},
			Err(_)=>{
				println!("Reques Stream Error");
			}
		}
    }
    // close server
    drop(listener);
}
