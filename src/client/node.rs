pub mod node {
    use uuid::Uuid;
    pub struct Node {
        ip: String,
        guid: Uuid,
    }

    impl Node {
        pub fn new(ip: String) -> Node {
            Node {
                ip,
                guid: Uuid::new_v4(),
            }
        }
        pub fn guid(&self) -> String {
            let bytes = self.guid.simple().to_string();
            return bytes;
        }
        pub fn ip(&self) -> &str {
            self.ip.as_ref()
        }
    }
}
