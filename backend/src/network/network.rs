use crate::blockchain::blockchain::Blockchain;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub async fn handle_client(mut socket: TcpStream, blockchain: &mut Blockchain) {
    let mut buffer = vec![0; 1024];
    loop {
        let n = socket.read(&mut buffer).await.unwrap();
        if n == 0 {
            return;
        }
        // Handle incoming data and potentially update blockchain
        socket.write_all(&buffer[0..n]).await.unwrap();
    }
}
