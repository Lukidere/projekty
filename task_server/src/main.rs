mod queue;
use std::{error::Error, thread};
mod clients;
use crate::clients::handle_client;


use tokio::{io::{AsyncReadExt,AsyncWrite,AsyncWriteExt},time::{sleep,Duration},task,sync::{Mutex,mpsc},net::{TcpStream,TcpSocket,TcpListener}};
use crate::queue::{Task,TaskQueue};
type TaskResult = Box<dyn FnOnce() + Send + 'static>;
pub const welcome_message:&str = "Hello and welcome to task server!\nChoose your service:\n1.Reverse String\n2.Print out length of the text u provide\n3.Add two numbers";
const addr:&str = "127.0.0.1:8080";
#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>> {
    let task_queue = TaskQueue::new();
    let server = TcpListener::bind(addr).await?;
    loop {
        let task_queue_clone = task_queue.clone();
        let workingthread = task::spawn(async move {
            let task_queue2 = task_queue_clone.clone();
            loop {
                if let Some(zadanie) = task_queue2.get_task().await {

                    match zadanie {
                        Task::ReverseString(val,mut stream) => stream.write_all(val.trim().chars().rev().collect::<String>().to_string().to_string().as_bytes()).await.unwrap(),
                        Task::WordCount(slowo,mut stream) => stream.write_all(slowo.trim().split_whitespace().count().to_string().as_bytes()).await.unwrap()
                    }
                } else {
                    thread::sleep(Duration::from_secs(1))
                }
            }
        });
        let task_queue_append = task_queue.clone();
        let (stream,socket) = server.accept().await?;
        let _= task::spawn(async move {
            let task_queue3 = task_queue_append.clone();
            task_queue3.add_task(handle_client(stream).await.unwrap()).await;

        });

    }
 
    Ok(())
}
