use std::thread;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use base64::Engine;
//use base64::Engine;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
//use base64::engine::general_purpose;


fn main(){

    let mut threads = Vec::new();

    let listener_http = TcpListener::bind("127.0.0.1:3000").unwrap();
    let listener_websocket = TcpListener::bind("127.0.0.1:3001").unwrap();

    threads.push(thread::spawn(move||{
        for stream in listener_http.incoming(){
        
            let stream = stream.unwrap();
            thread::spawn(move||{
                handle_connection_http(stream);
            });
    
        }
    }));

    threads.push(thread::spawn(move||{
        for stream in listener_websocket.incoming(){
        
            let stream = stream.unwrap();
            thread::spawn(move||{
                handle_connection_websocket(stream);
            });
    
        }
    }));

    for t in threads{
        let _ = t.join();
    }

}

fn handle_connection_websocket(mut stream: TcpStream){
    /*find_hash("lol".to_string());
    let mut buffer = [0;1028];
    stream.read(&mut buffer).unwrap();
    let text = String::from_utf8_lossy(&buffer[..]);
    let mut fields = text.lines();
    let key = fields
    .find(|line| line.starts_with("Sec-WebSocket-Key"))
    .map(|line| line.split(": ").nth(1).unwrap())
    .unwrap();
    let accept_key = find_hash(key.to_string());
    let response = format!(
        "HTTP/1.1 101 Switching Protocols\r\n\
        Upgrade: websocket\r\n\
        Connection: Upgrade\r\n\
        Sec-WebSocket-Accept: {}\r\n\
        Sec-WebSocket-Protocol: chat\r\n\r\n",
        accept_key
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();*/
    let mut buffer = [0; 1028];
    stream.read(&mut buffer).unwrap();
    let text = String::from_utf8_lossy(&buffer[..]);
    let mut fields = text.lines();
    let key = fields
        .find(|line| line.starts_with("Sec-WebSocket-Key"))
        .map(|line| line.split(": ").nth(1).unwrap())
        .unwrap();
    let accept_key = find_hash(key);
    let response = format!(
        "HTTP/1.1 101 Switching Protocols\r\n\
         Upgrade: websocket\r\n\
         Connection: Upgrade\r\n\
         Sec-WebSocket-Accept: {}\r\n\
         Sec-WebSocket-Protocol: chat\r\n\r\n",
        accept_key
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

/*fn find_hash(key: String)->String{
    println!("{}", key);
    let mut hasher = Sha1::new();
    let mut input = key.to_string();
    input.push_str("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    hasher.input_str(&input);
    let sha_result = hasher.result_str();
    println!("Hash = {}", sha_result);
    let base64_result = base64::encode(sha_result);
    println!("key = {}", base64_result);
    base64_result
}*/

fn find_hash(key: &str) -> String {
    println!("key = {}", key);
    let mut hasher = Sha1::new();
    let mut input = key.to_string();
    input.push_str("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    hasher.input_str(&input);
    let sha_result = hasher.result_str();
    println!("Sha = {}", sha_result);
    let mut answer = String::new();
    answer = base64::engine::general_purpose::STANDARD.encode(sha_result.as_bytes());
    println!("Base64 = {}", answer);
    return answer;
}

fn handle_connection_http(mut stream: TcpStream){
    let content = "<!DOCTYPE html>
    <html>
      <head>
        <meta charset=\"UTF-8\" />
      </head>
      <body>
        WebSocket test page
        <script>
          let ws = new WebSocket('ws://localhost:3001');
          ws.onmessage = event => alert('Message from server: ' + event.data);
          ws.onopen = () => ws.send('hello');
        </script>
      </body>
    </html>
    ";

    let response = format!(
        "HTTP/1.1 200 0k\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}