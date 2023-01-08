use std::{
    net::TcpStream,
    io::prelude::*
};

use shared::response::StatusCode;

fn main() {

    let mut stream = TcpStream::connect("127.0.0.1:6969").unwrap();
    let mut buffer = [0u8; 1024];

    stream.read(&mut buffer).unwrap();

    let code: u16 = u16::from_be_bytes([buffer[0], buffer[1]]);
    let body = String::from_utf8(buffer[2 .. buffer.len() - 1].to_vec()).unwrap();


    if code != StatusCode::OK {
        println!("Connection failed! Code: {}, Body: {:?}", code, body)
    } else {
        println!("Connection successful! Body: {}", body);
    }
}
