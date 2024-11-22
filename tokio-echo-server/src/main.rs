use std::io::{Read, Write};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
const addr: &str = "127.0.0.1:8001";
const addr2: &str = "127.0.0.1:8000";
#[tokio::main()]
async fn main() {
    let sluchacz = TcpListener::bind(addr).await;
    let sluchacz = match sluchacz {
        Ok(val) => {
            println!("pomyslne stworzono: {}", val.local_addr().unwrap().ip());
            val
        }
        Err(_) => panic!("pomyslnie nie utworzono"),
    };
    loop {
        let (stream, _) = sluchacz.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}
async fn handle_connection(mut stream: TcpStream) {
    let mut buf: [u8; 1024] = [0; 1024];

    let _ = stream.read(&mut buf).await.unwrap();
    let message = String::from_utf8_lossy(&buf);
    println!("received:{message}");
    let wiadomosc_handle = handler(message.to_string()).await;
    let output = format!("handle says {wiadomosc_handle}");
    let _ = stream.write(output.as_bytes()).await;
    println!("sent {output}");
}

async fn handler(msg: String) -> String {
    let _stream = TcpStream::connect(addr2).await;
    let mut stream = match _stream {
        Ok(val) => val,
        Err(err) => panic!("{err}"),
    };
    let _ = stream.write_all(msg.as_bytes()).await;
    let _ = stream.flush();
    let mut buffer: [u8; 1024] = [0; 1024];
    let _ = stream.read(&mut buffer).await.unwrap();
    let message = String::from_utf8_lossy(&buffer);
    message.to_string()
}
