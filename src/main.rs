use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

const PORT: i32 = 3000;

fn main() {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", PORT)).expect("could not bind listener");
    // let (mut stream, _socket) = listener.accept().unwrap();

    let mut shutdown = Arc::new(Mutex::new(false));

    // check if the server has connections for the last minute
    // let mut : bool = false;

    let mut connections = 0;
    thread::Builder::new()
        .name("please-connect".to_string())
        .spawn({
            move || {
                loop {
                    println!("Hey my guy, this server is awaiting for connections!");
                    thread::sleep(Duration::from_secs(5));
                }
            }
        })
        .unwrap();

    let server_handler = thread::Builder::new()
        .name(format!("server"))
        .spawn({
            println!("Starting server ..");
            println!("━━(￣ー￣*|||━━");
            let shutdown = shutdown.clone();
            let mut i = 0;
            move || {
                while !*shutdown.lock().unwrap() {
                    i += 1;

                    let (stream, socket) = listener.accept().unwrap();
                    println!("received connection from conn-{}", i)
                }
            }
        })
        .expect("failed to connect to a thread");

    let sream = server_handler.join().unwrap();
}

// fn handle_connection(i: i32, mut stream: TcpStream) -> JoinHandle<()> {
//     let join_handle = stream.write(b"Hello Fahd!").expect("failed to write");

//     println!("connected to conn-{}", i);

//     join_handle
// }
