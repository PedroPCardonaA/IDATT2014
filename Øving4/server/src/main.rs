use std::net::UdpSocket;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("127.0.0.1:7878")?;
    loop {
        let mut buf = [0; 1024];
        let (amt, src) = socket.recv_from(&mut buf)?;
        println!("received {} bytes from {}: {}", amt, src, String::from_utf8_lossy(&buf[..amt]));
        println!("{}",calculate(&String::from_utf8_lossy(&buf[..amt])));
        socket.send_to(calculate(&String::from_utf8_lossy(&buf[..amt])).as_bytes(), src)?;
    }
    Ok(())
}


fn calculate(data: &str) -> String{
    let mut v = Vec::new();
    

    for word in data.split_whitespace(){
        v.push(word);
    } 
    if v.len() == 3{
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

fn to_camel_case(text: &str) -> String {
    let mut answer = String::new();
    let mut sub_text = text.to_string();
    let mut end_pointer = 0; 
    for char in text.chars(){
        if char == '-' || char == '_'{
            let first;
            let second;
            let copy = sub_text.clone();
            (first, second) = copy.split_at(end_pointer);
            sub_text = second.to_string();
            answer = format!("{}{}",answer,first);
            println!("{}",answer.clone())
        }
        end_pointer += 1;
    }
    answer = format!("{}{}",answer,sub_text);
    return  answer;
}