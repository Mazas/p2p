pub mod node {
    pub struct Node {
        ip: String,
        guid: String,
    }

    impl Node {
        pub fn new(ip: String, guid: String) -> Node {
            Node { 
                ip,
                guid,
            }
        }

        pub fn ip(&self) -> &str {
            self.ip.as_ref()
        }
    }
}
