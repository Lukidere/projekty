use futures::{executor::*, lock::Mutex};
use std::prelude::*;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
const addr: &str = "localhost:1234";
use std::net::TcpListener;
#[tokio::main]
async fn main() {
    let mut stream = match TcpStream::connect(addr).await {
        Ok(val) => {
            println!(
                "połączono {},{}",
                val.local_addr().unwrap().ip(),
                val.local_addr().unwrap().port()
            );
            val
        }
        Err(err) => panic!("{err}"),
    };
    let wiadomosc = "dziala";
    let _ = stream.write(wiadomosc.as_bytes()).await;
    let mut buffer: [u8; 1024] = [0; 1024];
    let _ = stream.read(&mut buffer).await.unwrap();
    let callback = String::from_utf8_lossy(&buffer);
    println!("zwrotna: {callback}")
}
