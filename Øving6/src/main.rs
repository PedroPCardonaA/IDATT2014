use std::thread;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use base64::Engine;
use sha1::{Sha1, Digest};


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
    

    let mut buffer = [0; 1028];
    stream.read(&mut buffer).unwrap();
    let text = String::from_utf8_lossy(&buffer[..]);
    
    println!("{}",text);
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
    let clone = stream.try_clone();
    let mes = read_message(clone.unwrap());
    println!("{}", mes);
    send_message(stream, &mes);
}

fn send_message(mut stream: TcpStream, message: &str) {
    // Construct the WebSocket message header and payload
    let payload = message.as_bytes();
    let payload_len = payload.len();
    let mut header = [0u8; 10];
    header[0] = 0x81; // FIN bit set, opcode is text
    if payload_len <= 125 {
        header[1] = payload_len as u8;
    } else if payload_len <= 65535 {
        header[1] = 126;
        header[2..4].copy_from_slice(&(payload_len as u16).to_be_bytes());
    } else {
        header[1] = 127;
        header[2..10].copy_from_slice(&(payload_len as u64).to_be_bytes());
    }
    // Send the WebSocket message to the server
    stream.write_all(&header).unwrap();
    stream.write_all(payload).unwrap();
    stream.flush().unwrap();
}
fn read_message(mut stream: TcpStream) -> String {
    // Read the WebSocket message header
    let mut header = [0u8; 14];
    stream.read_exact(&mut header[..2]).unwrap();
    let fin = header[0] & 0x80 != 0;
    let opcode = header[0] & 0x0f;
    let masked = header[1] & 0x80 != 0;
    let payload_len = match header[1] & 0x7f {
        126 => u16::from_be_bytes(header[2..4].try_into().unwrap()),
        127 => u64::from_be_bytes(header[2..10].try_into().unwrap()) as u16,
        n => n as u16,
    };
    let mask = if masked {
        let mut mask = [0u8; 4];
        stream.read_exact(&mut mask).unwrap();
        Some(mask)
    } else {
        None
    };
    // Read the WebSocket message payload
    let mut payload = vec![0u8; payload_len as usize];
    stream.read_exact(&mut payload).unwrap();
    if let Some(mask) = mask {
        for i in 0..payload_len as usize {
            payload[i] ^= mask[i % 4];
        }
    }
    String::from_utf8_lossy(&payload).to_string()
}

fn find_hash(key: &str) -> String {
    println!("key = {}", key);
    let mut hasher = Sha1::new();
    let mut input = key.to_string();
    input.push_str("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    //println!("Concated = {}", input);
    hasher.update(&input);
    let sha_result = hasher.finalize();
    let mut answer = String::new();
    //println!(" Sha = {:?}",sha_result);
    answer = base64::engine::general_purpose::STANDARD.encode(sha_result);
    //println!("Base64 = {}", answer);
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
          let ws = new WebSocket('ws://localhost:3001', 'chat');
          ws.onmessage = event => alert('Message from server: ' + event.data);
          ws.onopen = function(e) {
            console.log('[open] Connection established');
            console.log('Sending to server');
            console.log('Hei send to the server!')
            ws.send('Hei, server');
          };
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