pub mod crypto;
pub mod client {
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::str::from_utf8;
    use crate::crypto;
    use self::node::node::Node;
    pub mod node;

    pub struct Client {
        node: Node,
        references: Vec<Node>,
    }
    impl Client {
        pub fn new() -> Client {
            return Client {
                node: Node::new("localhost".to_string()),
                references: vec![],
            };
        }
        pub fn locate_references(&mut self) {
            self.references = vec![Node::new(self.node.ip().to_string())];
        }


        pub fn send(&self, &data: &[u8; 256]) {
            let hash_string: String = crypto::calculate_hash(&data);
            let address = format!("{}:3333", self.references[0].ip());
            match TcpStream::connect(address) {
                Ok(mut stream) => {
                    println!("Successfully connected to server.");
                    stream.write(&data).unwrap();
                    println!("Sent, awaiting reply...");
                    let mut response = [0 as u8; 256]; // using 6 byte buffer
                    match stream.read_exact(&mut response) {
                        Ok(_) => {
                            let expected_response = crypto::str_to_buf(hash_string);
                            let text = from_utf8(&response).unwrap();
                            if &response == &expected_response {
                                println!("Success: {}", text);
                            } else {
                                println!("Unexpected reply: {}", text);
                            }
                        }
                        Err(e) => {
                            println!("Failed to receive data: {}", e);
                        }
                    }
                    stream.shutdown(std::net::Shutdown::Both).expect("Shutdown failed");
                }
                Err(e) => {
                    println!("Failed to connect: {}", e);
                }
            }
            println!("Terminated.");
        }
    }
}

fn main() {
    let mut client = client::Client::new();
    client.locate_references();
    let message = "Hello!".to_owned();
    let data = crypto::str_to_buf(message);
    client.send(&data)

}
