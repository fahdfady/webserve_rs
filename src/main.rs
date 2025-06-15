use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread::{self, JoinHandle};
use std::time::Duration;

const PORT: i32 = 3000;

fn main() {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", PORT)).expect("could not bind listener");
    // let (mut stream, _socket) = listener.accept().unwrap();

    // check if the server has connections for the last minute
    let mut has_connected_since_minute: bool = false;

    let mut connections = 0;

    thread::Builder::new()
        .name("please-connect".to_string())
        .spawn({
            let has_connected_since_minute = has_connected_since_minute.clone();

            move || {
                if has_connected_since_minute == true {
                    loop {
                        println!("Hey my guy, this server is awaiting for connections!");
                        thread::sleep(Duration::from_secs(90));
                    }
                } else {
                    println!("we're in has_connected_since_minute else");
                }
            }
        })
        .unwrap();
    for stream in listener.incoming() {
        connections += 1;
        match stream {
            Ok(stream) => handle_connection(connections, stream, has_connected_since_minute),
            Err(err) => return,
        };
    }
}

fn handle_connection(
    i: i32,
    mut stream: TcpStream,
    has_connected_since_minute: bool,
) -> JoinHandle<()> {
    let join_handle = thread::Builder::new()
        .name(format!("conn-{}", i))
        .spawn(|| {})
        .expect("failed to connect to a thread");

    stream.write(b"Hello Fahd!").expect("failed to write");

    println!("connected to conn-{}", i);

    join_handle
}
