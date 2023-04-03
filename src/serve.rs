use super::model::Files;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::path::Path;
use std::{env::Args, str::FromStr};
use tiny_http::{Header, Request, Response};


//TODO rename
#[derive(Serialize, Deserialize)]
struct QueryFromClient {
    query: String,
    quantity: usize,
}

enum ContentType {
    HTML,
    JS,
    JSON
}

fn create_content_type(content_type : ContentType) -> Header{
    let header = match content_type {
        ContentType::HTML => "Content-Type: text/html",
        ContentType::JS => "Content-Type: text/javascript",
        ContentType::JSON => "Content-Type: application/json",
    };

    Header::from_str(header).unwrap()
}


pub fn serve_server(args: &mut Args) {
    let dir_path = args.next().expect("Please provide directory path");
    let address = args.next().unwrap_or("127.0.0.1:2128".to_string());

    let files = Files::build(Path::new(&dir_path)).unwrap();

    let server = tiny_http::Server::http(&address).unwrap();

    println!("Listening on : {address}");

    loop {
        let request = match server.recv() {
            Ok(rq) => rq,
            Err(e) => {
                println!("error: {}", e);
                break;
            }
        };
        request_matching(request, &files);
    }
}

fn request_matching(request: Request, files: &Files) {
    match request.method() {
        tiny_http::Method::Get => url_get(request),
        tiny_http::Method::Post => url_post(request, files),
        _ => todo!(),
    }
}

fn url_get(request: Request) {
    match request.url() {
        "/" => serve_file(request, "./public/index.html", "text/html;"),
        "/public/js" => serve_file(request, "./public/index.js", "text/javascript;"),
        _ => todo!(),
    };
}

fn url_post(request: Request, files: &Files) {
    match request.url() {
        "/api/search" => search(request, &files),
        _ => todo!(),
    }
}

fn search(mut request: Request, files: &Files) {
    let mut body_buffer = String::new();
    request
        .as_reader()
        .read_to_string(&mut body_buffer)
        .unwrap();
    let query: QueryFromClient = match serde_json::from_str(&body_buffer) {
        Ok(v) => v,
        Err(error) => {
            let response = Response::from_string(format!(
                "{{\"status_code\" : 400, \"reason\": \"{error}\"}}"
            ))
            .with_header(create_content_type(ContentType::JSON)).with_status_code(400); //TODO refactor this ugly code
            request
                .respond(
                    response,
                )
                .unwrap();
            return;
        }
    };

    let result = files.search(query.query).rank(query.quantity);

    let json = json!({ "data": result }).to_string();

    let header = create_content_type(ContentType::JSON);
    let response = Response::from_string(json).with_header(header);

    match request.respond(response) {
        Ok(ok) => ok,
        Err(e) => panic!("Request Invalid {e}"),
    };
}

fn serve_file(request: Request, path: &str, content_type: &str) {
    let path = Path::new(&path);
    let file = File::open(path).unwrap();

    let content_type = format!("Content-Type: {content_type}");

    let header = Header::from_str(&content_type).unwrap();
    let response = Response::from_file(file).with_header(header);

    match request.respond(response) {
        Ok(ok) => ok,
        Err(e) => panic!("Request Invalid {e}"),
    };
}
