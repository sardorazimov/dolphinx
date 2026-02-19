use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::time::Duration;

pub async fn probe_http(host: &str, port: u16) -> Option<String> {

    let addr = format!("{}:{}", host, port);

    let mut stream = match tokio::time::timeout(
        Duration::from_secs(3),
        TcpStream::connect(&addr)
    ).await {

        Ok(Ok(s)) => s,
        _ => return None
    };

    let request = format!(
        "GET / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        host
    );

    if stream.write_all(request.as_bytes()).await.is_err() {
        return None;
    }

    let mut buffer = vec![0; 4096];

    match tokio::time::timeout(
        Duration::from_secs(3),
        stream.read(&mut buffer)
    ).await {

        Ok(Ok(n)) if n > 0 => {

            Some(String::from_utf8_lossy(&buffer[..n]).to_string())

        }

        _ => None
    }
}
