use std::{
    net::{ TcpListener, TcpStream },
    io::prelude::*,
};

use shared::response::StatusCode;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Incoming connection!");
        handle_connection(stream);
    }

}

fn handle_connection(mut stream: TcpStream) {
    // stream.read(&mut buffer).unwrap();

    let mut buffer: Vec<u8> = StatusCode::OK.to_be_bytes().to_vec();
    buffer.append("Hey".as_bytes().to_vec().as_mut());


    stream.write(&buffer).unwrap();
    stream.flush().unwrap();
}