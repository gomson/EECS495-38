use super::{Request,Response};

extern crate time;

use std::io::{ErrorKind, Write, Read};
use std::net::{TcpListener, TcpStream};
use std::fs::{File};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::env;

pub fn send_request(stream: &mut TcpStream) -> String{
    const BUF_SIZE: usize = 64;
    let mut buf = [0; BUF_SIZE];
    let mut get_request = String::new();
    while let Ok(n) = stream.read(&mut buf[..]){
    	get_request.push_str(&String::from_utf8(buf.to_owned()).unwrap());
    	if n < BUF_SIZE{
    		break;
    	}
    }
    return get_request;
}

pub fn form_request(request: Vec<&str>) -> Request{
	if request.len() < 3 || request[0] != "GET" || !request[2].contains("HTTP") {
		return Request{
        method: "".to_string(),
	      path: "".to_string(),
	      protocol: "".to_string(), 
	      error: "Yes".to_string(),
		};
	}
	Request{
    method: request[0].to_string(),
    path: request[1].to_string(),
	  protocol: request[2].to_string(),
	  error: "No".to_string(),
  }
}

pub fn get_response(request: &Request, content_type: String, content_length: usize, status: String) -> Response{
    Response{
       protocol: request.protocol.clone(),
       method: request.method.clone(),
       status: status,
       content_type: content_type,
       content_length: content_length,
    }
}

pub fn handle_response(response: Response, stream: &mut TcpStream){
	if response.status == "200"{
        stream.write("HTTP/1.0 200 OK\nzlj234-yyd198-web-server/0.1\n".clone().to_owned().as_bytes()).unwrap();
        stream.write("Content-type: ".clone().to_owned().as_bytes()).unwrap();
        stream.write(response.content_type.clone().to_owned().as_bytes()).unwrap();
        stream.write("\nContent-length: ".clone().to_owned().as_bytes()).unwrap();
	      stream.write(response.content_length.to_owned().to_string().as_bytes()).unwrap();
        stream.write("\n\n".clone().to_owned().as_bytes()).unwrap();
	}
	else if response.status == "400"{
        stream.write("\nHTTP/1.0 400 Bad Request\nzlj234-yyd198-web-server/0.1\n\n".clone().to_owned().as_bytes()).unwrap();
	}
	else if response.status == "403"{
        stream.write("\nHTTP/1.0 403 Forbidden\nzlj234-yyd198-web-server/0.1\n\n".clone().to_owned().as_bytes()).unwrap();
	}
	else if response.status == "404"{
        stream.write("\nHTTP/1.0 404 Not Found\nzlj234-yyd198-web-server/0.1\n\n".clone().to_owned().as_bytes()).unwrap();
	}
}

pub fn record_req_to_log(request: &Request, log_file: &Arc<Mutex<File>>){
    let mut guard = log_file.lock().unwrap();
    let current_time = time::now().ctime().to_string();
    let mut request_to_log1 = String::new();
    let mut request_to_log = String::new();
  
    if request.error == "No"{
        request_to_log = request_to_log + &"Request method: " + &request.method + &"\n";
        request_to_log = request_to_log + &"Request Path: " + &request.path + &"\n";
        request_to_log = request_to_log + &"Request time: " + &current_time.to_string() + &"\n\n";
        guard.write(request_to_log.clone().to_owned().as_bytes()).unwrap();
    }
    else{
        request_to_log1 = request_to_log1 + &"Invalid Request" + &"\n";
        request_to_log1 = request_to_log1 + &"Request time: " + &current_time.to_string() + &"\n";
        guard.write(request_to_log1.clone().to_owned().as_bytes()).unwrap();
    }
    
}



