use std::io::Read;
use std::net::TcpListener;

pub fn start() {
    println!("Creating listener...");
    let listener = TcpListener::bind(("0.0.0.0", 8080)).expect("Could not bind to address");

    println!("Waiting for connection...");
    let (mut stream, socket) = listener.accept().expect("Could not accept connection");

    println!("Received connection from {socket}");

    loop {
        println!("Reading stream...");
        let mut length_buffer = [0u8; 4];
        stream.read_exact(&mut length_buffer).unwrap();

        let length = u32::from_be_bytes(length_buffer);

        println!("Expecting message of length {length}");

        let mut contents_buffer: std::vec::Vec<u8> = vec![0; length as usize];
        stream.read_exact(contents_buffer.as_mut_slice()).unwrap();

        let received_string = String::from_utf8(contents_buffer).unwrap();

        println!("Received string: {received_string}");
    }
}
