use std::io::Write;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use color_print::cprintln;

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

                    handle_connection(i, stream, socket, Arc::clone(&shutdown));
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

fn handle_connection(
    i: i32,
    mut stream: TcpStream,
    socket: SocketAddr,
    shutdown: Arc<Mutex<bool>>,
) -> JoinHandle<()> {
    let connection_id = format!("conn-{}", i);

    println!("connected to {}", connection_id);

    // check shutdown with pattern matching
    thread::Builder::new()
        .name(connection_id.clone())
        .spawn(move || {
            while !*shutdown.lock().unwrap() {
                if let Err(e) = stream.write(b"Hello, Fahd!") {
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
