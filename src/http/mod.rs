use std::fs;
use std::io::{BufReader, Error, prelude::*};
use std::net::TcpStream;

use crate::http::utils::request_is_http;

mod utils;

type Result<T> = std::result::Result<T, Error>;

pub fn read_http_request(mut stream: TcpStream) -> crate::http::Result<()> {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if request_is_http(http_request.clone()) {
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

        // todo: in shutdown, delete the requests.txt

        let status_line = "HTTP/1.1 200 OK";

        let contents = fs::read_to_string("assets/index.html")
            .expect("Error 404 couldn't find the index HTML file");
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\nContent-Type: text/html\r\n\r\n{contents}"
        );

        println!(
            "Sending response:\n{}",
            response.lines().take(10).collect::<Vec<_>>().join("\n")
        );

        stream.write_all(response.as_bytes()).unwrap();
    }
    Ok(())
}
