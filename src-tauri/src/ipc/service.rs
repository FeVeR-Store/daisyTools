use super::error::{IpcError, Result};
use super::message::{Message, MessageType};
use super::{ADDRESS, PROT};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use uuid::Uuid;

type SharedTcpStream = Arc<Mutex<TcpStream>>;
type ClientMap = Arc<Mutex<HashMap<Uuid, SharedTcpStream>>>;

pub fn setup_tcp_server() {
    tokio::spawn(async {
        let mut server = TcpServer::new().await?;
        server.set_handler(|msg| {
            log::info!("处理: {}", msg.content);
            Message::new(MessageType::Response, format!("处理: {}", msg.content))
        });
        server.start().await?;
        Ok::<(), IpcError>(())
    });
}

pub struct TcpServer {
    listener: TcpListener,
    handler: Arc<Mutex<Box<dyn Fn(Message) -> Message + Send + Sync>>>,
    clients: ClientMap,
}

impl TcpServer {
    pub async fn new() -> Result<Self> {
        let listener = TcpListener::bind(ADDRESS.to_string() + ":" + PROT).await?;
        let handler: Box<dyn Fn(Message) -> Message + Send + Sync> = Box::new(|msg| {
            Message::new(
                MessageType::Response,
                format!("Default response to: {}", msg.content),
            )
        });

        Ok(Self {
            listener,
            handler: Arc::new(Mutex::new(handler)),
            clients: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub fn set_handler<F>(&mut self, handler: F)
    where
        F: Fn(Message) -> Message + Send + Sync + 'static,
    {
        self.handler = Arc::new(Mutex::new(Box::new(handler)));
    }

    // 新增: 向所有客户端推送消息
    pub async fn broadcast(&self, content: String) -> Result<()> {
        let message = Message::new(MessageType::Push, content);
        let mut clients = self.clients.lock().await;
        
        let mut disconnected_clients = Vec::new();
        
        for (id, stream) in clients.iter() {
            let mut stream = stream.lock().await;
            if let Err(_) = message.clone().write_to(&mut *stream).await {
                disconnected_clients.push(*id);
            }
        }
        
        // 清理断开连接的客户端
        for id in disconnected_clients {
            clients.remove(&id);
        }
        
        Ok(())
    }

    // 新增: 向特定客户端推送消息
    pub async fn push_to_client(&self, client_id: Uuid, content: String) -> Result<()> {
        let message = Message::new(MessageType::Push, content);
        let clients = self.clients.lock().await;
        
        if let Some(stream) = clients.get(&client_id) {
            let mut stream = stream.lock().await;
            message.write_to(&mut *stream).await?;
            Ok(())
        } else {
            Err(IpcError::custom_error("Client not found"))
        }
    }

    pub async fn start(&self) -> Result<()> {
        println!("Server started at {}", self.listener.local_addr()?);

        loop {
            let (socket, addr) = self.listener.accept().await?;
            println!("New connection from: {}", addr);

            let client_id = Uuid::new_v4();
            let handler = Arc::clone(&self.handler);
            let clients = Arc::clone(&self.clients);
            let shared_socket = Arc::new(Mutex::new(socket));
            
            // 存储客户端连接
            {
                let mut clients_map = clients.lock().await;
                clients_map.insert(client_id, Arc::clone(&shared_socket));
            }

            tokio::spawn(async move {
                if let Err(e) = handle_connection(shared_socket, handler, client_id, clients).await {
                    eprintln!("Connection error: {}", e);
                }
            });
        }
    }
}

async fn handle_connection(
    socket: SharedTcpStream,
    handler: Arc<Mutex<Box<dyn Fn(Message) -> Message + Send + Sync>>>,
    client_id: Uuid,
    clients: ClientMap,
) -> Result<()> {
    loop {
        let mut stream = socket.lock().await;
        if let Some(message) = Message::read_from(&mut *stream).await? {
            match message.message_type {
                MessageType::Request => {
                    let response = handler.lock().await(message);
                    response.write_to(&mut *stream).await?;
                }
                MessageType::Heartbeat => {
                    let heartbeat_response = Message::new(MessageType::Heartbeat, "pong".to_string());
                    heartbeat_response.write_to(&mut *stream).await?;
                }
                _ => {}
            }
        } else {
            println!("Client {} disconnected", client_id);
            let mut clients_map = clients.lock().await;
            clients_map.remove(&client_id);
            break;
        }
    }
    Ok(())
}