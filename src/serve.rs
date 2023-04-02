
use std::{env::Args, str::FromStr};
use tiny_http::{Response, Request, Header};
use std::fs::File;
use super::model::{Files};
use std::path::Path;



pub fn serve_server(args :&mut Args){
    let dir_path = args.next().expect("Please provide directory path");
    let address = args.next().unwrap_or("127.0.0.1:2128".to_string());

    let files = Files::build(Path::new(&dir_path)).unwrap();

    let server = tiny_http::Server::http(&address).unwrap();


    println!("Listening on : {address}");

    
loop {
    let request = match server.recv() {
        Ok(rq) => rq,
        Err(e) => { println!("error: {}", e); break }
    };
    request_matching(request,&files);
} 
}

fn request_matching(request : Request,files : &Files){
    match request.method() {
        tiny_http::Method::Get => url_get(request),
        tiny_http::Method::Post => url_post(request,files),
        _ => todo!(),
    }
}

fn url_get(request : Request){
    match request.url() {
        "/" => serve_file(request, "./public/index.html", "text/html;"),
        "/public/js" => serve_file(request, "./public/index.js", "text/javascript;"),
        _ => todo!(),
     };
}

fn url_post(request : Request,files : &Files){
    match request.url() {
        "/api/search" => search(request,&files),
        _ => todo!()
    }
}



fn search(mut request :  Request,files : &Files){
    let mut body_buffer= String::new();
    request.as_reader().read_to_string(&mut body_buffer).unwrap();

    let result = files.search(body_buffer).rank(10);
    //TODO Find proper way to send json without using 3rd party library


    let mut json = String::with_capacity(200);
    json.push_str("{\"data\" : [");
    for (file_path,_) in result {
        json.push_str("\"");
        json.push_str(&file_path);
        json.push_str("\"");
        json.push_str(",");
    }

    json.pop();
    json.push_str("]}");

    let header = Header::from_str("Content-Type: application/json").unwrap();
    let response = Response::from_string(json).with_header(header);

    match request.respond(response) {
        Ok(ok) => ok, 
        Err(e) => panic!("Request Invalid {e}"),
    };
}

fn serve_file(request : Request, path : &str, content_type : &str){
    let path = Path::new(&path);
    let file = File::open(path).unwrap();

    let content_type = format!( "Content-Type: {content_type}");
    
    let header = Header::from_str(&content_type).unwrap();
    let response = Response::from_file(file).with_header(header);

    match request.respond(response) {
        Ok(ok) => ok, 
        Err(e) => panic!("Request Invalid {e}"),
    };
}
