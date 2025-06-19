use std::fs;
use std::io::{BufReader, Error, prelude::*};
use std::net::TcpStream;

use crate::http::utils::request_is_http;

mod utils;

type Result<T> = std::result::Result<T, Error>;

enum HttpStatus {
    Ok,
    NotFound,
}

impl HttpStatus {
    fn as_str(&self) -> &'static str {
        match self {
            HttpStatus::Ok => "HTTP/1.1 200 OK",
            HttpStatus::NotFound => "HTTP/1.1 404 NOT FOUND",
        }
    }
}

struct HttpResponse {
    status: HttpStatus,
    content_length: usize,
    content_type: String,
    body: String,
}

impl HttpResponse {
    fn new(status: HttpStatus, contents: String) -> Self {
        Self {
            status: status,
            content_length: contents.len(),
            content_type: String::from("text/html"), // temporary defaulted to `text/html` .. todo: make it to be dynamic in another iterations
            body: contents,
        }
    }

    fn as_str(&self) -> String {
        format!(
            "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
            self.status.as_str(),
            self.content_length,
            self.content_type,
            self.body
        )
    }
}

pub fn read_http_request(mut stream: TcpStream) -> crate::http::Result<()> {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if request_is_http(&http_request) {
        send_http_response(&http_request.clone()[0], http_request, stream);
    }
    Ok(())
}

fn send_http_response(request_line: &str, http_request: Vec<String>, mut stream: TcpStream) {
    // todo: maybe put that writing to a file logic into a separate thread?
    std::fs::create_dir("tmp").expect_err("couldn't create dir tmp");

    let already_existing_requests =
        std::fs::read_to_string("tmp/requests.txt").unwrap_or("".to_string());

    std::fs::write(
        "tmp/requests.txt",
        format!(
            "{}\n Request: {:#?}",
            already_existing_requests, http_request
        ),
    )
    .expect("could not write requests to tmp/requests.txt");

    let response: String = match request_line {
        "GET / HTTP/1.1" => {
            let contents =
                fs::read_to_string("assets/index.html").expect("Couldn't Load the HTML File");
            HttpResponse::new(HttpStatus::Ok, contents)
                .as_str()
                .to_owned()
        }
        "GET /workout HTTP/1.1" => {
            let contents =
                fs::read_to_string("assets/workout.html").expect("Couldn't Load the HTML File");
            HttpResponse::new(HttpStatus::Ok, contents)
                .as_str()
                .to_owned()
        }
        _ => {
            let contents =
                fs::read_to_string("assets/404.html").expect("Couldn't Load the HTML File");
            HttpResponse::new(HttpStatus::NotFound, contents)
                .as_str()
                .to_owned()
        }
    };

    println!(
        "Sending response:\n{}",
        response.lines().take(10).collect::<Vec<_>>().join("\n")
    );

    stream.write_all(response.as_bytes()).unwrap();
}
