use std::{sync::Arc,collections::VecDeque};
use tokio::{net::TcpStream,sync::Mutex};
pub enum Task {
    ReverseString(String,TcpStream),
    WordCount(String,TcpStream),
}
#[derive(Clone)]
pub struct TaskQueue {
    pub queue: Arc<Mutex<VecDeque<Task>>>
}

impl TaskQueue {
   pub fn new() -> TaskQueue {
        TaskQueue {
        queue:Arc::new(Mutex::new(VecDeque::new()))
        }
    }
   pub async fn add_task(&self,task:Task) {
       self.queue.lock().await.push_back(task);
      
    }
   pub async fn get_task(&self) -> Option<Task> {
       self.queue.lock().await.pop_front()
   }
}
