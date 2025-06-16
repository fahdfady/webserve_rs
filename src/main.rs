use std::io::{BufReader, prelude::*};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::{fs, vec};

use color_print::cprintln;

const PORT: i32 = 3000;

fn main() {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", PORT)).expect("could not bind listener");
    // let (mut stream, _socket) = listener.accept().unwrap();

    let shutdown = Arc::new(Mutex::new(false));

    // check if the server has connections for the last minute
    // let mut : bool = false;

    let connections = Arc::new(Mutex::new(0));

    let server_handler = thread::Builder::new()
        .name(format!("server"))
        .spawn({
            println!("Starting server ..");
            println!("━━(￣ー￣*|||━━");
            let shutdown = Arc::clone(&shutdown);
            let connections = Arc::clone(&connections);
            // todo: debug the number of connections here.

            let mut i = 0;
            move || {
                while !*shutdown.lock().unwrap() {
                    i += 1;
                    *connections.lock().unwrap() += 1;
                    // todo: debug the number of connections here.

                    let (stream, socket) = listener.accept().unwrap();

                    handle_connection(
                        i,
                        stream,
                        socket,
                        Arc::clone(&shutdown),
                        Arc::clone(&connections),
                    );
                }
            }
        })
        .expect("failed to connect to a thread");

    let counter_handler = thread::Builder::new()
        .name("please-connect".to_string())
        .spawn({
            let shutdown = Arc::clone(&shutdown);
            let connections = Arc::clone(&connections);

            // todo: debug the number of connections here.
            move || {
                while !*shutdown.lock().unwrap() {
                    // while *connections.lock().unwrap() > 0 {
                    println!("Hey my guy, this server is awaiting for connections!");
                    thread::sleep(Duration::from_secs(5));
                }
                // }
            }
        })
        .unwrap();

    let stream = server_handler.join().unwrap();

    {
        // lock aqcuired
        let mut guard = shutdown.lock().unwrap();
        *guard = true;
        // lock dropped
    }
    println!("Shutdown initiated");

    counter_handler.join().unwrap();
    println!("Shutdown completed");
}

fn handle_connection(
    i: i32,
    mut stream: TcpStream,
    socket: SocketAddr,
    shutdown: Arc<Mutex<bool>>,
    connections: Arc<Mutex<i32>>,
) -> JoinHandle<()> {
    // todo: debug the number of connections here.
    // todo: minus one connection for the disconnected connections
    let connection_id = format!("conn-{}", i);

    println!("connected to {}", connection_id);

    // check shutdown with pattern matching
    thread::Builder::new()
        .name(connection_id.clone())
        .spawn(move || {
            while !*shutdown.lock().unwrap() {
                //  FIX: this write to stream does not work anymore
                //      because of the read_request function.

                read_http_request(stream.try_clone().expect("cloning stream failed"));

                if let Err(e) = { stream.write(b"Hello, Fahd!") } {
                    println!("Error writing to stream: {}, closing connection", e);
                    break;
                }
                if let Err(e) = stream.flush() {
                    println!("could not flush, closing connections")
                }
                thread::sleep(Duration::from_secs(3));
            }
            cprintln!("<r>connection <bold>{}</bold> closed</>", connection_id);
        })
        .unwrap()
}

fn read_http_request(mut stream: TcpStream) {
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

fn request_is_http(mut request: Vec<String>) -> bool {
    let http_request_start_line_rules = vec![
        "GET", "POST", "TRACE", "PUT", "HEAD", "CONNECT", "DELETE", "PATCH",
    ];

    for rule in http_request_start_line_rules {
        if request[0].contains(&rule.to_owned()) {
            println!("hahahaha that is an HTTP request congrats !!! ╰(*°▽°*)╯");
            continue;
        };
    }

    true
}
