#[doc=" This is simple http web server.
        Created by Zhiyi Liu, Yuanhui Yang.
        Netid: zlj234,  yyd198.

Assumptions: 
       This web_server only supports single command of HTTP 0.9.
       And it only supports 'GET' method.
       Please use this shape: GET /path/to/file HTTP.
       If the request has less than 3 tokens or the first token isn't 'GET' or the 3rd token doesn't contain 'HTTP',
       then the response is 400 Bad Request.
       If it has more than 3 tokens then only the first three tokens is valid.
       If the request is valid but we can't find the file according to the request path. Then the response is 404 Not Found.
       If the request path points to a directory. Then it is interpreted as pointing to one of these files: index.html, index.shtml, and index.txt.
       The first file found is served assuming it is accessible. Otherwise the path triggers a 404-message. Thus, the request GET /src HTTP will show us
       the content of index.txt since index.txt is in this dir.
       We record the request and the current time to a log file named log.txt.
"]

extern crate time;
use std::io::{ErrorKind, Write, Read};
use std::net::{TcpListener, TcpStream};
use std::fs::{File};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::env;

mod httpserver;

pub struct Request{
	method: String,
	path: String,
	protocol: String,
	error: String,
}

pub struct Response{
     protocol: String,
     method: String,
     status: String,
     content_type: String,
     content_length: usize,
}

fn main(){
	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	println!("Listening started, ready to accept");

    let log_file = Arc::new(Mutex::new(File::create("log.txt").unwrap()));

	for stream in listener.incoming() {
		let log_file = log_file.clone();
		let mut stream = stream.unwrap();
		thread::spawn(move || handle_client(&mut stream, &log_file));
	}
	drop(listener);
}

fn handle_client(stream: &mut TcpStream, log_file: &Arc<Mutex<File>>) {
	println!("New connection.");
    stream.write(b"If you are using 'telnet'. Please use this shape: GET /path/to/file HTTP\n");
	
	let mut input: String = httpserver::send_request(stream);
	let mut request: Vec<&str> = input.split_whitespace().collect();

	let mut request_after: Request = httpserver::form_request(request);
	httpserver::record_req_to_log(&request_after, &log_file);

	if request_after.error == "No"{
	  if Path::new(&request_after.path).has_root() {
		let mut path: String = String::new();
		path = env::current_dir().unwrap().to_string_lossy().to_owned().to_string();
		path.push_str(&request_after.path);
		stream.write("\n".clone().to_owned().as_bytes()).unwrap();
		//
		if Path::new(&path.clone()).exists() {
			if Path::new(&path.clone()).is_dir() {
				if Path::new(&path.clone()).join("index.html").is_file() {
					path = Path::new(&path.clone()).join("index.html").to_string_lossy().to_owned().to_string();
				}
				else if Path::new(&path.clone()).join("index.shtml").is_file() {
					path = Path::new(&path.clone()).join("index.shtml").to_string_lossy().to_owned().to_string();
				}								
				else if Path::new(&path.clone()).join("index.txt").is_file() {
					path = Path::new(&path.clone()).join("index.txt").to_string_lossy().to_owned().to_string();
				}
				else {
					stream.write("\nHTTP/1.0 404 Not Found\nzlj234-yyd198-web-server/0.1\n\n".clone().to_owned().as_bytes()).unwrap();
				}
			}
			if Path::new(&path.clone()).is_file() {
				let mut response:Response;
                let mut f = File::open(path.clone()).unwrap();
                let mut file_content = String::new();
                match f.read_to_string(&mut file_content) {
		           Ok(_) => {
			          if Path::new(&path.clone().to_owned()).extension().unwrap() == "html" {
			            	response = httpserver::get_response(&request_after, "text/html".to_string(), file_content.len(), "200".to_string());
				            httpserver::handle_response(response,stream);
				            
			          }
			          else if Path::new(&path.clone().to_owned()).extension().unwrap() != "html" {
			            	response = httpserver::get_response(&request_after, "text/plain".to_string(), file_content.len(), "200".to_string());
			    	        httpserver::handle_response(response,stream);
			          }							
	 	              stream.write(file_content.clone().to_owned().as_bytes()).unwrap();
	                  stream.write("\n".clone().to_owned().as_bytes()).unwrap();								
		           },
	               Err(e) => {
	    	          if e.kind() == ErrorKind::InvalidInput {
                            response = httpserver::get_response(&request_after, "".to_string(), "".len(), "400".to_string());
                            httpserver::handle_response(response,stream);
	 	              }
	 	              else if e.kind() == ErrorKind::PermissionDenied {
	 		             	response = httpserver::get_response(&request_after, "".to_string(), "".len(), "403".to_string());
	 	            		httpserver::handle_response(response,stream);
	 	        	  }
	 	        	  else if e.kind() == ErrorKind::NotFound {
     	        	    	response = httpserver::get_response(&request_after, "".to_string(), "".len(), "404".to_string());
     	        	    	httpserver::handle_response(response,stream);
	 	              }
		           },
	            }							
		    }
	    }
	    else if !Path::new(&path.clone()).exists() {
			stream.write("\nHTTP/1.0 404 Not Found\nzlj234-yyd198-web-server/0.1\n\n".clone().to_owned().as_bytes()).unwrap();
	    }
	  }
	  else if !Path::new(&request_after.path).has_root() {
			stream.write("\nHTTP/1.0 400 Bad Request\nzlj234-yyd198-web-server/0.1\n\n".clone().to_owned().as_bytes()).unwrap();
	  }		
	}
	if request_after.error == "Yes"{
        stream.write("\nHTTP/1.0 400 Bad Request\nzlj234-yyd198-web-server/0.1\n\n".clone().to_owned().as_bytes()).unwrap();
	}
	
}

#[cfg(test)]
mod form_request_test {
	use httpserver::{form_request};
	#[test]
    fn error_test1(){
        let mut input: String = "GET 111".to_string();
        let mut request: Vec<&str> = input.split_whitespace().collect();
        let error_request = form_request(request);
        assert_eq!(error_request.error,"Yes".to_string());
        assert_eq!(error_request.method,"".to_string());
    }
    #[test]
    fn error_test2(){
    	let mut input: String = "GET /src HTTP".to_string();
        let mut request: Vec<&str> = input.split_whitespace().collect();
        let error_request = form_request(request);
        assert_eq!(error_request.error,"No".to_string());
        assert_eq!(error_request.method,"GET".to_string());
    }
    #[test]
    fn error_test3(){
    	let mut input: String = "111".to_string();
        let mut request: Vec<&str> = input.split_whitespace().collect();
        let error_request = form_request(request);
        assert_eq!(error_request.error,"Yes".to_string());
        assert_eq!(error_request.method,"".to_string());
    }
}

mod path_test{
	use httpserver::{form_request};
	use std::fs::{File};
	use std::path::Path;
	use std::env;
	#[test]
    fn path_test1(){
    	let mut input: String = "GET /src HTTP".to_string();
        let mut request: Vec<&str> = input.split_whitespace().collect();
        let path_request = form_request(request);
        assert_eq!(path_request.path,"/src".to_string());
        assert_eq!(Path::new(&path_request.path).has_root(),true);
    }
    #[test]
    fn path_test2(){
    	let mut input: String = "GET /src HTTP".to_string();
        let mut request: Vec<&str> = input.split_whitespace().collect();
        let path_request = form_request(request);
        assert_eq!(path_request.path,"/src".to_string());
        let mut path: String = String::new();
		path = env::current_dir().unwrap().to_string_lossy().to_owned().to_string();
        assert_eq!(path,"/Users/zhiyiliu1993/Downloads/EECS495-38-HW4");
        path.push_str(&path_request.path);
        assert_eq!(path,"/Users/zhiyiliu1993/Downloads/EECS495-38-HW4/src");
        assert_eq!(Path::new(&path.clone()).exists(),true);
        assert_eq!(Path::new(&path.clone()).is_dir(),true);
        path.push_str(&"/index.txt");
        assert_eq!(path,"/Users/zhiyiliu1993/Downloads/EECS495-38-HW4/src/index.txt");
        assert_eq!(Path::new(&path.clone()).is_file(),true);
    }
}

mod get_response_test{
	use httpserver::{get_response};
	use httpserver::{form_request};
	#[test]
    fn response_test(){
    	let mut input: String = "GET /src/main.rs HTTP".to_string();
        let mut request: Vec<&str> = input.split_whitespace().collect();
        let request_after = form_request(request);
        let mut response = get_response(&request_after,"text/plain".to_string(),"hello world".len(),"200".to_string());
        assert_eq!(response.protocol,"HTTP");
        assert_eq!(response.content_length,11);
    }
}




