use crate::blockchain::Blockchain;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn handle_client(mut socket: TcpStream, _blockchain: &mut Blockchain) {
    let mut buffer = [0; 1024];
    loop {
        let n = socket.read(&mut buffer).await.unwrap();
        if n == 0 {
            return;
        }
        socket.write_all(&buffer[0..n]).await.unwrap();
    }
}
