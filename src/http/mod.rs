use std::io::{BufReader, prelude::*};
use std::net::TcpStream;

use crate::http::utils::request_is_http;

mod utils;

pub fn read_http_request(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let buf_reader_clone = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    request_is_http(http_request.clone());

    println!("Request: {:#?}", http_request);
}
