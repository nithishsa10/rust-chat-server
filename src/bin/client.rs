use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| {"127.0.0.1:8080".to_string()});
    
    println!("Connecting to server at {}", server_addr);
    let stream = TcpStream::connect(server_addr).await?;
    println!("Connected to the server.");

    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    let mut read_task = tokio::spawn(async move {
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => {
                    println!("\nConnection closed by server.");
                    break;
                },
                Ok(_) => {
                    print!("\nServer: {}", line);
                    io::stdout().flush().unwrap();
                },
                Err(_) => {
                    eprintln!("\nError reading from server.");
                    break;
                }
            }
        }
    });
    let mut write_task = tokio::spawn(async move {
        let stdin = io::stdin();
        let mut input_line = String::new();

        loop {
            input_line.clear();
            if stdin.read_line(&mut input_line).is_err() {
                eprint!("\nError reading from stdin.");
                break;
            }

            if input_line.trim().is_empty() {
                continue;
            }
            
            if input_line.trim().eq_ignore_ascii_case("/quit") {
                println!("Disconnecting from chat.");
                break;
            }

            if writer.write_all(input_line.as_bytes()).await.is_err() {
                eprint!("\nError sending message to server.");
                break;
            }
        }
    });

    tokio::select! {
        _ = (&mut read_task) => write_task.abort(),
        _ = (&mut write_task) => read_task.abort()
    }
    Ok(())
}