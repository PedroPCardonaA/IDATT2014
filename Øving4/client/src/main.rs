use std::net::UdpSocket;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    loop{
        let mut data = String::new();
        println!("Enter operation: ");
        std::io::stdin().read_line(&mut data).unwrap();
        socket.send_to(data.as_bytes(), "127.0.0.1:7878")?;
    
        let mut buf = [0; 1024];
        let (amt, _src) = socket.recv_from(&mut buf)?;
        println!("received {} bytes: The result is {}", amt, String::from_utf8_lossy(&buf[..amt]));
    }

    Ok(())
}