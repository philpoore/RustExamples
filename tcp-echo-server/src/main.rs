use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::env;
use std::error::Error;
use log::info;
use simple_logger::SimpleLogger;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let server = TcpListener::bind(addr).await?;

    info!("TCP Echo Server Listening on : {}", server.local_addr().unwrap());
    loop {
        let (mut socket, addr) = server.accept().await?;

        info!("Client connected : {:}", addr);
        tokio::spawn(async move {
            let mut buff = [0; 1024];

            loop {
                let n = socket.read(&mut buff).await.expect("Error : Failed to read from client");
                
                if n ==0 { break }

                socket.write_all(&buff[0..n])
                    .await
                    .expect("Error: Failed to write to client");
                info!("Handled message : {:} bytes = {}", addr, n);
            }

            info!("Client disconnected");
        });
    }
}