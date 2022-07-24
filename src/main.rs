use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use crate::client::client::Client;

pub mod client;
pub mod node;
pub mod crypto;

const BUFFER_SIZE: usize = 256;

fn handle_client(mut stream: TcpStream){
    let mut data = [0 as u8; BUFFER_SIZE];
    match stream.read_exact(&mut data) {
        Ok(_) => {
            let received_data = &data[0..BUFFER_SIZE];
            let hash_string = crypto::calculate_hash(&received_data);
            println!("Received: {}", hash_string);
            // echo everything!
            stream.write(&crypto::str_to_buf(hash_string)).unwrap();
            println!("Closing connection...");
            stream.shutdown(Shutdown::Both).expect("Failed to shut down gracefully...");
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
        }
    }
}

fn start_server() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}

fn main() {
    start_server();
    let mut client = Client::new();
    client.locate_references();
    let message = "Hello!".to_owned();
    let data = crypto::str_to_buf(message);
    client.send(&data)
}