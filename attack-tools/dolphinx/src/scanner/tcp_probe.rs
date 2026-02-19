use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::time::Duration;

pub async fn probe_tcp(host: &str, port: u16) -> Option<String> {

    let addr = format!("{}:{}", host, port);

    let mut stream = match tokio::time::timeout(
        Duration::from_secs(3),
        TcpStream::connect(&addr)
    ).await {

        Ok(Ok(s)) => s,
        _ => return None
    };

    // Generic probes
    let probes = vec![
        b"\r\n".as_ref(),
        b"HELP\r\n".as_ref(),
        b"QUIT\r\n".as_ref(),
        b"GET / HTTP/1.0\r\n\r\n".as_ref(),
    ];

    for probe in probes {

        if stream.write_all(probe).await.is_err() {
            continue;
        }

        let mut buffer = [0u8; 2048];

        if let Ok(Ok(n)) = tokio::time::timeout(
            Duration::from_secs(2),
            stream.read(&mut buffer)
        ).await {

            if n > 0 {

                return Some(
                    String::from_utf8_lossy(&buffer[..n]).to_string()
                );
            }
        }
    }

    None
}
