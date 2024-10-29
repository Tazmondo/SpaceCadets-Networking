use std::io::{stdin, Read, Write};
use std::net::TcpStream;

fn write_message(stream: &mut TcpStream, message: &str) {
    let data_length = message.len() as u32;
    let data_length_bytes: [u8; 4] = data_length.to_be_bytes();

    stream.write_all(&data_length_bytes).unwrap();
    stream.write_all(message.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn wait_for_input() -> String {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("TODO: panic message");

    input
}

pub fn start() {
    println!("Connecting to host...");
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    println!("Connected!");
    stream.set_nodelay(true).unwrap();

    loop {
        let str_to_write = wait_for_input();
        write_message(&mut stream, &str_to_write);
    }
}
