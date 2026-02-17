use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

use std::sync::Arc;
use std::sync::atomic::Ordering;

use crate::stats::Stats;

use rand::rngs::StdRng;
use rand::{SeedableRng, Rng};
use rand::seq::SliceRandom;

use std::cell::RefCell;


// Thread-local RNG (Send-safe, fast)
thread_local! {
    static RNG: RefCell<StdRng> =
        RefCell::new(StdRng::from_entropy());
}


// User agents
const USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 13_0)",
    "Mozilla/5.0 (X11; Linux x86_64)",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 16_0)",
    "curl/7.81.0",
    "dolphinx/0.1",

];

// Paths
const PATHS: &[&str] = &[
    "/",
    "/api",
    "/login",
    "/dashboard",
    "/index.html",
    "/home",
    "/health",
];


pub async fn send_http_request(
    target: String,
    stats: Arc<Stats>,
) {

    let (host, port, base_path) = parse_target(&target);

    let addr = format!("{}:{}", host, port);

    match TcpStream::connect(&addr).await {

        Ok(mut stream) => {

            // Use thread-local RNG safely
            let (path, user_agent, fake_ip) = RNG.with(|rng| {

                let mut rng = rng.borrow_mut();

                let random_path = PATHS.choose(&mut *rng).unwrap();

                let ua = USER_AGENTS.choose(&mut *rng).unwrap();

                let ip = format!(
                    "{}.{}.{}.{}",
                    rng.gen_range(1..255),
                    rng.gen_range(1..255),
                    rng.gen_range(1..255),
                    rng.gen_range(1..255)
                );

                (random_path.to_string(), ua.to_string(), ip)

            });

            let final_path = if base_path == "/" {
                path
            } else {
                format!("{}/{}", base_path, path.trim_start_matches('/'))
            };

            let request = format!(
                "GET {} HTTP/1.1\r\n\
                 Host: {}\r\n\
                 User-Agent: {}\r\n\
                 Accept: */*\r\n\
                 X-Forwarded-For: {}\r\n\
                 Connection: keep-alive\r\n\
                 \r\n",
                final_path,
                host,
                user_agent,
                fake_ip
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


// Parse target safely
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
