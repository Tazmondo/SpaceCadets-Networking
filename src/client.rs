use std::io::Write;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

pub fn start() {
    println!("Connecting to host...");
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    println!("Connected!");
    stream.set_nodelay(true).unwrap();

    for i in 0..5 {
        let str_to_write = "Hello World!";
        let data_length = str_to_write.len() as u32;
        let data_length_bytes: [u8; 4] = data_length.to_be_bytes();

        stream.write_all(&data_length_bytes).unwrap();
        println!("Wrote data length: {:#?}", data_length_bytes);
        stream.write_all(str_to_write.as_bytes()).unwrap();
        stream.flush().unwrap();
        println!("Sent data...");
        sleep(Duration::from_secs(1));
    }
}
