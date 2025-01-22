mod web;

use std::net::TcpListener;
use web::handle_client;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Could not bind");
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}
