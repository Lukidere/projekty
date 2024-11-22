use std::io::prelude::*;
use std::net::TcpStream;
const echo_sever_address: &str = "localhost:8080";
fn main() {
    let mut stream = match TcpStream::connect(echo_sever_address) {
        Ok(val) => {
            println!(
                "Polaczanie udane! {},{}",
                val.local_addr().unwrap().ip(),
                val.local_addr().unwrap().port()
            );
            val
        }
        Err(err) => panic!("{err}"),
    };
    let wiadomosc = "elo";
    let _ = stream.write(wiadomosc.as_bytes());
    let mut buffer: [u8; 1024] = [0; 1024];
    let len = stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer);
    println!("Uwaga wiadomosc: {message}");
    stream.flush();
}
