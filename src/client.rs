pub mod client {
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::str::from_utf8;

    use self::node::node::Node;
    pub mod node;
    pub struct Client {
        node: Node,
        references: Vec<Node>,
    }
    impl Client {
        pub fn new() -> Client {
            return Client {
                node: node::node::Node::new("localhost".to_string()),
                references: vec![],
            };
        }
        pub fn locate_references(&mut self) {
            self.references = vec![node::node::Node::new(self.node.ip().to_string())];
        }
        pub fn send(&self, &data: &[u8; 256]) {
            let address = format!("{}:3333", self.references[0].ip());
            match TcpStream::connect(address) {
                Ok(mut stream) => {
                    println!("Successfully connected to server.");
                    stream.write(&data).unwrap();
                    println!("Sent, awaiting reply...");
                    let mut response = [0 as u8; 256]; // using 6 byte buffer
                    match stream.read_exact(&mut response) {
                        Ok(_) => {
                            if &response == &data {
                                println!("{:}", from_utf8(&response).unwrap())
                            } else {
                                let text = from_utf8(&response).unwrap();
                                println!("Unexpected reply: {}", text);
                            }
                        }
                        Err(e) => {
                            println!("Failed to receive data: {}", e);
                        }
                    }
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
    let message = b"Hello!";
    let mut packet = [0 as u8; 256];
    for i in 0..message.len() {
        packet[i] = message[i];
    }
    client.send(&packet)

}
