use sha2::{Sha256, Digest};
use base64;
use uuid::Uuid;

pub fn calculate_hash(data:&[u8]) -> String {
    let data_string = std::str::from_utf8(&data).unwrap();
    let data_hash = Sha256::digest(data_string);
    return base64::encode(&data_hash);
}

pub fn str_to_buf(data:String) -> [u8;256] {
    let data_bytes = data.as_bytes();
    let mut packet = [0 as u8; 256];
    for i in 0..data_bytes.len() {
        packet[i] = data_bytes[i];
    }
    return packet;
}

pub fn guid() -> String{
    return Uuid::new_v4().simple().to_string().to_owned();
}