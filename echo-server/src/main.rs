use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, thread};
use std::env::args;
use std::time::Duration;
const S_ser_add:&str = "127.0.0.1:8000";
fn main() {
    let listener = TcpListener::bind(S_ser_add).unwrap();
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        handle_connection(_stream)

    }
}
fn handle_connection(mut stream:TcpStream,) {
    let mut buffer:[u8;1024] = [0;1024];
    let len = stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer[..len]);
    println!("received {message}");
    let _ = stream.write_all(message.as_bytes());
    println!("sent: {}",message);

}
