use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

use std::sync::Arc;
use std::sync::atomic::Ordering;

use crate::stats::Stats;

pub async fn send_http_request(
    target: String,
    stats: Arc<Stats>,
) {

    let (host, port, path) = parse_target(&target);

    let addr = format!("{}:{}", host, port);

    match TcpStream::connect(&addr).await {

        Ok(mut stream) => {

            // Browser-like GET request
            let request = format!(
                "GET {} HTTP/1.1\r\n\
                 Host: {}\r\n\
                 User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64)\r\n\
                 Accept: */*\r\n\
                 Accept-Language: en-US,en;q=0.9\r\n\
                 Connection: keep-alive\r\n\
                 Cache-Control: no-cache\r\n\
                 \r\n",
                path,
                host
            );

            if stream.write_all(request.as_bytes()).await.is_ok() {

                stats.success.fetch_add(1, Ordering::Relaxed);

            } else {

                stats.failed.fetch_add(1, Ordering::Relaxed);

            }

        }

        Err(_) => {

            stats.failed.fetch_add(1, Ordering::Relaxed);

        }

    }

}


// POST request support
pub async fn send_post_request(
    target: String,
    stats: Arc<Stats>,
) {

    let (host, port, path) = parse_target(&target);

    let addr = format!("{}:{}", host, port);

    match TcpStream::connect(&addr).await {

        Ok(mut stream) => {

            let body = "data=test";

            let request = format!(
                "POST {} HTTP/1.1\r\n\
                 Host: {}\r\n\
                 User-Agent: multi-stresser\r\n\
                 Content-Type: application/x-www-form-urlencoded\r\n\
                 Content-Length: {}\r\n\
                 Connection: keep-alive\r\n\
                 \r\n\
                 {}",
                path,
                host,
                body.len(),
                body
            );

            if stream.write_all(request.as_bytes()).await.is_ok() {

                stats.success.fetch_add(1, Ordering::Relaxed);

            } else {

                stats.failed.fetch_add(1, Ordering::Relaxed);

            }

        }

        Err(_) => {

            stats.failed.fetch_add(1, Ordering::Relaxed);

        }

    }

}


// Parse URL safely
fn parse_target(target: &str) -> (String, u16, String) {

    let cleaned = target
        .replace("http://", "")
        .replace("https://", "");

    let mut parts = cleaned.split('/');

    let host_port = parts.next().unwrap();

    let path = format!("/{}", parts.collect::<Vec<&str>>().join("/"));

    let mut hp = host_port.split(':');

    let host = hp.next().unwrap().to_string();

    let port = hp
        .next()
        .unwrap_or("80")
        .parse::<u16>()
        .unwrap_or(80);

    let final_path = if path == "/" { "/".to_string() } else { path };

    (host, port, final_path)

}
