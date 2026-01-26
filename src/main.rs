use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use std::net::SocketAddr;
use tokio::net::TcpStream;
use std::collections::HashMap;

type Tx = mpsc::UnboundedSender<String>;
type Rx = mpsc::UnboundedReceiver<String>;

struct ChatServer {
    clients: Arc<Mutex<HashMap<SocketAddr, Tx>>>,
}

impl ChatServer {
    fn new() -> Self {
        ChatServer {
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn broadcast(&self, msg: String, sender: SocketAddr) {
        let clients = self.clients.lock().await;
        for (addr, tx) in clients.iter() {
            if *addr != sender {
                let _ = tx.send(msg.clone());
            }
        }
    }

    async fn handle_client(
        self: Arc<Self>,
        stream: TcpStream,
        addr: SocketAddr,
    ) {
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);

        let (tx, mut rx) : (Tx, Rx) = mpsc::unbounded_channel();
        self.clients.lock().await.insert(addr, tx);

        let welcome = format!("Welcome to the chat, {}!\n", addr);
        let _ = writer.write_all(welcome.as_bytes()).await;

        let join_msg = format!("{} has joined the chat\n", addr);
        self.broadcast(join_msg, addr).await;

        let mut write_task = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if writer.write_all(msg.as_bytes()).await.is_err() {
                    break;
                }
            }
        });

        let server = self.clone();
        
        let mut reader_task = tokio::spawn(async move {
            let mut line = String::new();
            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => break,
                    Ok(_) => {
                        let msg = format!("{} : {}", addr, line);
                        println!("{}", msg.trim());
                        server.broadcast(msg, addr).await;
                    }
                    Err(_) => break,
                }
            }
        });

        tokio::select! {
            _ = (&mut write_task) => reader_task.abort(),
            _ = (&mut reader_task) => write_task.abort(),
        }

        self.clients.lock().await.remove(&addr);
        let leave_msg = format!("{} has left the chat\n", addr);
        self.broadcast(leave_msg, addr).await;
        println!("Client {} disconnected", addr);
    }

    async fn run(self: Arc<Self>, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        println!("Chat server running on {}", addr);

        loop {
            let (stream, addr) = listener.accept().await?;
            println!("New client connected: {}", addr);
            
            let server = self.clone();
            tokio::spawn(async move {
                server.handle_client(stream, addr).await;
            });
        }
    }

}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Arc::new(ChatServer::new());
    server.run("127.0.0.1:8080").await?;
    Ok(())
}