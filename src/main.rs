use std::io::Write;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

const PORT: i32 = 3000;

fn main() {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", PORT)).expect("could not bind listener");
    // let (mut stream, _socket) = listener.accept().unwrap();

    let shutdown = Arc::new(Mutex::new(false));

    // check if the server has connections for the last minute
    // let mut : bool = false;

    let mut connections = 0;

    let server_handler = thread::Builder::new()
        .name(format!("server"))
        .spawn({
            println!("Starting server ..");
            println!("━━(￣ー￣*|||━━");
            let shutdown = Arc::clone(&shutdown);
            let mut i = 0;
            move || {
                while !*shutdown.lock().unwrap() {
                    i += 1;

                    let (stream, socket) = listener.accept().unwrap();

                    handle_connection(i, stream, socket);
                }
            }
        })
        .expect("failed to connect to a thread");

    let counter_handler = thread::Builder::new()
        .name("please-connect".to_string())
        .spawn({
            let shutdown = Arc::clone(&shutdown);
            move || {
                while !*shutdown.lock().unwrap() {
                    println!("Hey my guy, this server is awaiting for connections!");
                    thread::sleep(Duration::from_secs(5));
                }
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

fn handle_connection(i: i32, mut stream: TcpStream, socket: SocketAddr) -> JoinHandle<()> {
    println!("connected to conn-{}", i);

    thread::Builder::new()
        .name(format!("conn-{}", i))
        .spawn(move || {
            loop {
                stream
                    .write(b"Hello, Fahd!")
                    .expect("could not write to stream");
                thread::sleep(Duration::from_secs(2));
            }
        })
        .unwrap()
}
