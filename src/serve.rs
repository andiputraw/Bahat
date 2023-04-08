use crate::utils::{read_config_file, AppConfig};

use super::model::Files;
use super::utils;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;
use std::{env::Args, str::FromStr};
use tiny_http::{Header, Request, Response, StatusCode};

//TODO rename
#[derive(Serialize, Deserialize)]
struct QueryFromClient {
    query: String,
    quantity: usize,
}

#[derive(Serialize, Deserialize)]
struct OpenFromClient {
    path: String,
}

enum ContentType {
    HTML,
    JS,
    JSON,
}

fn create_content_type(content_type: ContentType) -> Header {
    let header = match content_type {
        ContentType::HTML => "Content-Type: text/html",
        ContentType::JS => "Content-Type: text/javascript",
        ContentType::JSON => "Content-Type: application/json",
    };

    Header::from_str(header).unwrap()
}

pub fn serve_server(mut args: Args) {
    let dir_path = args.next().expect("Please provide directory path");
    let port = args.next().unwrap_or("2128".to_string());
    let address = format!("127.0.0.1:{port}");

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
        "/" => serve_file(request, "./public/index.html", ContentType::HTML),
        "/public/js" => serve_file(request, "./public/index.js", ContentType::JS),
        _ => serve_file(request, "./public/404.html", ContentType::HTML),
    };
}

fn url_post(request: Request, files: &Files) {
    match request.url() {
        "/api/search" => search(request, &files),
        "/api/open" => open_in_file(request),
        _ => serve_file(request, "./public/404.html", ContentType::HTML),
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
            let response = response_error(error.to_string());
            request.respond(response).unwrap();
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

fn open_in_file(mut request: Request) {
    let query: OpenFromClient = match get_json_body(&mut request) {
        Ok(v) => v,
        Err(response) => {
            request.respond(response).unwrap();
            return;
        }
    };

    if let Err(()) = utils::open_file(query.path) {
        println!("Error opening file");
        panic!();
    }

    let response = response_success(
        "{{\"status_code\" : 200, \"success\": true}}",
        StatusCode(200),
    );

    match request.respond(response) {
        Ok(ok) => ok,
        Err(e) => panic!("Request Invalid {e}"),
    };
}

fn serve_file(request: Request, path: &str, content_type: ContentType) {
    let path = Path::new(&path);
    let file = File::open(path).unwrap();

    let header = create_content_type(content_type);
    let response = Response::from_file(file).with_header(header);

    match request.respond(response) {
        Ok(ok) => ok,
        Err(e) => panic!("Request Invalid {e}"),
    };
}

fn response_error(error: String) -> Response<std::io::Cursor<Vec<u8>>> {
    let response = Response::from_string(format!(
        "{{\"status_code\" : 400,\"success\" : false ,\"reason\": \"{error}\"}}"
    ))
    .with_header(create_content_type(ContentType::JSON))
    .with_status_code(400);
    return response;
}

fn response_success(
    response_body: &str,
    status_code: StatusCode,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let response = Response::from_string(response_body)
        .with_header(create_content_type(ContentType::JSON))
        .with_status_code(status_code);

    return response;
}

fn get_json_body<T>(request: &mut Request) -> Result<T, Response<Cursor<Vec<u8>>>>
where
    T: DeserializeOwned,
{
    let mut body_buffer = String::new();
    request
        .as_reader()
        .read_to_string(&mut body_buffer)
        .unwrap();

    let json: T = match serde_json::from_str(&body_buffer) {
        Ok(v) => v,
        Err(error) => {
            let response = response_error(error.to_string());
            return Err(response);
        }
    };

    Ok(json)
}
