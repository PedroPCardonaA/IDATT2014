use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {

    loop{
        let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
        let mut data = String::new();
        println!("Enter operation: ");
        std::io::stdin().read_line(&mut data).unwrap();
        stream.write_all(data.as_bytes()).unwrap();
        let mut buf = [0; 1024];
        stream.read(&mut buf).unwrap();
        let data = std::str::from_utf8(&buf).unwrap();
        println!("Received result: {}", data.to_string());
    }
}