use tokio::{io::{AsyncReadExt,AsyncWrite,AsyncWriteExt}, net::{TcpListener, TcpSocket, TcpStream}, sync::{mpsc, Mutex}, task, time::{sleep,Duration}};
use crate::queue::{Task,TaskQueue};
use crate::welcome_message;
use std::{error::Error};
pub async fn handle_client(mut stream: tokio::net::TcpStream) -> Option<Task> {
    println!("handluje clienta...");
    let mut buf:[u8;128] = [0;128];
    let _ = stream.write_all((welcome_message.to_string()+ "\n").as_bytes()).await;
    let ilosc = stream.read(&mut buf).await.unwrap();
    let wiadomosc = String::from_utf8_lossy(&buf[0..ilosc]).to_string();
    println!("{:#?}",wiadomosc);
    let task = match wiadomosc.trim().parse() {
        Ok(val) => {
            buf.fill(0);
            match val {
                1 => {
                    let _ = stream.write_all("napisz wiadomosc do odwrocenia \n".as_bytes()).await;
                    let ilosc = stream.read(&mut buf).await.unwrap();
                    let msg = String::from_utf8_lossy(&buf[0..ilosc]).to_string();

                    stream.flush().await;
                    Some(Task::ReverseString(msg,stream))
                }
                2 => {
                    let _ = stream.write_all("napisz wiadomosc do zbadania dlugosci \n".as_bytes()).await;
                    let ilosc = stream.read(&mut buf).await.unwrap();
                    let msg = String::from_utf8_lossy(&buf[0..ilosc]).to_string();

                    stream.flush().await;
                    Some(Task::WordCount(msg,stream))
                }
                _ => {
                    let _ = stream.write_all("Bledna opcja opuszczanie...".as_bytes()).await;
                    None
                }
                
            }
        }
        _ => {
            println!("what");
            None
        }
        Err(err) => {
            println!("{err}");
            None
        }
    };
    println!("Koniec handlowania wysylam do wolnego tasku...");
    task
}

