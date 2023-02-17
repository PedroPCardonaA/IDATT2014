use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    let data = std::str::from_utf8(&buf).unwrap();
    let data_copy = data.clone();
    println!("Received: {}", data);
    println!("{}",calculate(data_copy));
    stream.write(calculate(data).as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on {:?}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Accepted connection from {:?}", stream.peer_addr().unwrap());
        thread::spawn(|| handle_client(stream));
    }
}


fn calculate(data: &str) -> String{
    let mut v = Vec::new();
    

    for word in data.split_whitespace(){
        v.push(word);
    } 
    if v.len() == 4{
        let mut text  = "Place holder";
        let v_copy = v.clone();
        if !check_values(v_copy){
            text = "Value entered is not valid!"
        }
        return do_operation(v);
    }
    else {
        return "The number of given arguments is not the correct!".to_string();   
    }
}

fn check_values(data: Vec<&str>) -> bool{
    if !is_string_numeric(data[0].to_string()){
        return false;
    }
    if !is_string_numeric(data[2].to_string()){
        return false;
    }
    if data[1] != "+" && data[1] != "-"{
        return false;
    }
    return true;
    
}

fn do_operation(data: Vec<&str>) -> String{
    
    let num1:i32 = data[0].parse().unwrap();
    let num2:i32 = data[2].parse().unwrap();
    if data[1] == "+"{
        return (num1 + num2).to_string();
    }
    if data[1] == "-"{
        return (num1-num2).to_string();
    }
    return "Not valid operation symbol!".to_string()
}

fn is_string_numeric(str: String) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}
