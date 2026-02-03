use super::error::Result;
use super::message::{Message, MessageType};
use super::{ADDRESS, PROT};
use common::tokio::net::TcpStream;
use common::tokio::time::{self, Duration};
use common::tokio::sync::Mutex;
use std::sync::Arc;

pub struct TcpClient {
    stream: Arc<Mutex<TcpStream>>,
    message_handler: Option<Arc<Box<dyn Fn(Message) + Send + Sync>>>,
}

impl TcpClient {
    pub async fn connect() -> Result<Self> {
        let stream = TcpStream::connect(ADDRESS.to_string() + ":" + PROT).await?;
        Ok(Self { 
            stream: Arc::new(Mutex::new(stream)),
            message_handler: None,
        })
    }

    // 设置消息处理器
    pub fn set_message_handler<F>(&mut self, handler: F)
    where
        F: Fn(Message) + Send + Sync + 'static,
    {
        self.message_handler = Some(Arc::new(Box::new(handler)));
    }

    pub async fn send_message(&mut self, content: String) -> Result<Option<Message>> {
        let mut stream = self.stream.lock().await;
        let message = Message::new(MessageType::Request, content);
        message.write_to(&mut *stream).await?;
        Message::read_from(&mut *stream).await
    }

    pub async fn start_listen(self) {
        let stream = Arc::clone(&self.stream);
        let handler = self.message_handler.clone();
        
        // 启动心跳
        let heartbeat_stream = Arc::clone(&stream);
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;
                let mut stream = heartbeat_stream.lock().await;
                let heartbeat = Message::new(MessageType::Heartbeat, "ping".to_string());
                if let Err(e) = heartbeat.write_to(&mut *stream).await {
                    eprintln!("Heartbeat error: {}", e);
                    break;
                }
            }
        });

        // 监听消息
        loop {
            let mut stream = stream.lock().await;
            match Message::read_from(&mut *stream).await {
                Ok(Some(message)) => {
                    if let Some(handler) = &handler {
                        handler(message);
                    }
                }
                Ok(None) => {
                    println!("Server disconnected");
                    break;
                }
                Err(e) => {
                    eprintln!("Error reading message: {}", e);
                    break;
                }
            }
        }
    }
}