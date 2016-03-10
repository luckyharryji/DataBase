extern crate rustc_serialize;
use rustc_serialize::json;
use std::net::{TcpListener,TcpStream};
use std::thread;
use std::sync::{Arc,Mutex};
use std::fs::OpenOptions;
use std::convert::AsRef;


extern crate time;  // import for record time for log

mod vec_dbcollection;
mod db_module;
use db_module::RustDB;
mod response;

mod request;
use request::Request;
pub mod lib;

use lib::{read_db, store_in_disk};

fn main() {
    initial_bind_server(8080);
}


fn handle_stream(stream:TcpStream,write_log_file: Arc<Mutex<OpenOptions>>, database_obj:&mut Arc<Mutex<RustDB>>){
    let request_time = time::now().ctime().to_string();    // record time when request come
    let mut request = Request::new(stream);                // parse the request, extract url and all requet info

    let mut on_database = database_obj.lock().unwrap();
    let mut respone_info = String::new();

    match request.get_command().as_ref(){
        "PUTLIST" => {
            match on_database.create_table(&request.get_collection(), &request.get_parameters()){
                Ok(_) => respone_info = json::encode(&"Success").unwrap(),
                Err(e) => respone_info = json::encode(&e.to_owned()).unwrap(),
            }
        },
        "DELETELIST" => {
            match on_database.delete_cl(&request.get_collection()){
                Ok(s) => respone_info = json::encode(&s.to_owned()).unwrap(),
                Err(err) => respone_info = json::encode(&err.to_owned()).unwrap(),
            }
        },
        "GETLIST" => {
            match on_database.find_cl(&request.get_collection()){
                Ok(s) => {
                    // create response here
                    let json_result: String = json::encode(s).unwrap();
                    println!("result of GETLIST is : {}", json_result);
                    respone_info = json_result;
                },
                Err(err) => respone_info = json::encode(&err.to_owned()).unwrap(),
            }
        },
        "APPEND" => {
            match on_database.find_cl(&request.get_collection()){
                Ok(s) => {
                    match s.insert(&request.get_attributes()){
                        Ok(s) => respone_info = json::encode(&s.to_owned()).unwrap(),
                        Err(err) => respone_info = json::encode(&err.to_owned()).unwrap(),
                    }
                },
                Err(err) => respone_info = json::encode(&err.to_owned()).unwrap(),
            }
        },
        "UPDATE" => {
            match on_database.find_cl(&request.get_collection()){
                Ok(s) => {
                    let (object, desired) = request.get_object_desired();
                    match s.update(&object, &desired){
                        None => respone_info = json::encode(&"Error".to_owned()).unwrap(),
                        Some(num) => {
                            println!("{} number of items are updated", &num);
                            respone_info = json::encode(&"Success".to_owned()).unwrap();
                        },
                    }
                },
                Err(err) => respone_info = json::encode(&err.to_owned()).unwrap(),
            }
        },
        "GET" => {
            match on_database.find_cl(&request.get_collection()){
                Ok(s) => {
                    match s.find(&request.get_attributes()){
                        Some(items) => {
                            let json_data: String = json::encode(&items).unwrap();
                            println!("the items find are: {}", &json_data);
                            respone_info = json_data;
                        },
                        None => {
                            println!("Illeagel query");
                            respone_info = json::encode(&"Error".to_owned()).unwrap();
                        },
                    }
                },
                Err(err) => respone_info = err.to_owned(),
            }
        },
        "DELETE" => {
            match on_database.find_cl(&request.get_collection()){
                Ok(s) => {
                    match s.delete(&request.get_attributes()){
                        Some(number) => {
                            println!("there are {} number of data deleted", &number);
                            respone_info = json::encode(&"Success".to_owned()).unwrap();
                        },
                        None => {
                            println!("Illeagel collection attribute");
                            respone_info = json::encode(&"Error".to_owned()).unwrap();
                        }
                    }
                },
                Err(err) => respone_info = err.to_owned(),
            }
        },
        "SHOWDB" => {
            on_database.show_db();
        },
        _ => {
            println!("Receive an illegel query method");
            respone_info = json::encode(&"Unsupport query type".to_owned()).unwrap();
        }
    }

    // in-disk storage for database content
    let json_for_storage: String = json::encode(&*on_database).unwrap();
    match store_in_disk(&json_for_storage) {
        Ok(_) => println!("Query result store successful"),
        _ => println!("Failed to store in disk"),
    }

    request.record_log(&request_time, &write_log_file);      // write request info into log

    let mut response = request.form_response(Some(respone_info));            // create response structure from request information
    response.write_response();           // send back response to the client
    let response_time = time::now().ctime().to_string();   // record time when send out response
    response.record_log(&response_time, &write_log_file);     // write request info into log
}


fn initial_bind_server(port:usize){
    // bing server to the localhost
    let bind_addr:&str = &("127.0.0.1:".to_owned()+&port.to_string());
    let listener = TcpListener::bind(bind_addr).unwrap();
    println!("Server Started");

    // new database object initial here 
    // read data from in-disk
    let mut database = Arc::new(Mutex::new(RustDB::new()));
    let file_for_log = Arc::new(Mutex::new(OpenOptions::new()));

    if let Ok(storage_string) = read_db(){
        if storage_string.is_empty() == false{
            let rust_db : RustDB = json::decode(&storage_string).unwrap();
            database = Arc::new(Mutex::new(rust_db));
        }
    }

    for stream in listener.incoming() {
        let log_file_for_write = file_for_log.clone();
        let mut database_obj = database.clone();
        match stream{
            Ok(stream)=>{               
                thread::spawn(move || {  // spawn a thread for each request 
                    handle_stream(stream,log_file_for_write,&mut database_obj);
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
