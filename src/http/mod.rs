use std::fs;
use std::io::{BufReader, prelude::*};
use std::net::TcpStream;

use crate::http::utils::request_is_http;

mod utils;

pub fn read_http_request(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if request_is_http(http_request.clone()) {
        // maybe put that writing to a file logic into a separate thread?
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

        let response = "HTTP/1.1 200 OK\r\n\r\n";

        stream.write_all(response.as_bytes()).unwrap();
    }
}
