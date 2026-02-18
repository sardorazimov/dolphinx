use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::time::{timeout, Duration};

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::error::Error;


const USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 13_0)",
    "Mozilla/5.0 (X11; Linux x86_64)",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 16_0)",
    "curl/7.81.0",
    "dolphinx/2.1.3",
];

const PATHS: &[&str] = &[
    "/",
    "/index.html",
    "/login",
    "/dashboard",
    "/api",
    "/health",
];


pub async fn send_http_request(
    target: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {

    let (host, port, _) = parse_target(target)?;

    let addr = format!("{}:{}", host, port);

    let mut stream =
        timeout(
            Duration::from_secs(5),
            TcpStream::connect(&addr)
        ).await??;

    let mut rng = thread_rng();

    let path =
        PATHS.choose(&mut rng)
        .unwrap_or(&"/");

    let ua =
        USER_AGENTS.choose(&mut rng)
        .unwrap_or(&"dolphinx");

    let request = format!(
        "GET {} HTTP/1.1\r\n\
         Host: {}\r\n\
         User-Agent: {}\r\n\
         Connection: close\r\n\
         Accept: */*\r\n\
         \r\n",
        path,
        host,
        ua
    );

    stream.write_all(request.as_bytes()).await?;

    Ok(())

}


fn parse_target(
    target: &str
) -> Result<(String, u16, String), Box<dyn Error + Send + Sync>> {

    if target.starts_with("http://") {

        let host = target
            .trim_start_matches("http://")
            .to_string();

        return Ok((host, 80, "/".into()));
    }

    if target.starts_with("https://") {

        let host = target
            .trim_start_matches("https://")
            .to_string();

        return Ok((host, 443, "/".into()));
    }

    if target.contains(':') {

        let parts: Vec<&str> =
            target.split(':').collect();

        if parts.len() == 2 {

            let host = parts[0].to_string();

            let port = parts[1]
                .parse::<u16>()?;

            return Ok((host, port, "/".into()));
        }

    }

    Ok((target.to_string(), 80, "/".into()))

}
