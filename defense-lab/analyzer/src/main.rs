use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;

use chrono::{Utc, Datelike};




const ATTACK_THRESHOLD: usize = 100;


#[tokio::main]
async fn main() {

    let listener = TcpListener::bind("0.0.0.0:8081")
        .await
        .expect("Failed to bind");

    println!("Analyzer listening on port 8081");

    let connections: Arc<Mutex<HashMap<String, usize>>> =
        Arc::new(Mutex::new(HashMap::new()));

    loop {

        let (mut socket, addr) = listener.accept().await.unwrap();

        let ip = addr.ip().to_string();

        let connections = connections.clone();

        tokio::spawn(async move {

            let mut buffer = [0; 1024];

            let _ = socket.read(&mut buffer).await;

            let mut map = connections.lock().unwrap();

            let count = map.entry(ip.clone()).or_insert(0);

            *count += 1;

            println!("{} -> {}", ip, count);

            if *count == ATTACK_THRESHOLD {

                println!("ATTACK DETECTED: {}", ip);

                log_attack(&ip, *count);

            }

        });

    }

}


// NEW: production log rotation system
fn log_attack(ip: &str, connections: usize) {

    // create logs folder
    create_dir_all("logs").ok();

    let now = Utc::now();

    let filename = format!(
        "logs/attack-{:04}-{:02}-{:02}.json",
        now.year(),
        now.month(),
        now.day()
    );

    let timestamp = now.to_rfc3339();

    let threat = if connections > 1000 {
        "CRITICAL"
    } else if connections > 500 {
        "HIGH"
    } else {
        "MEDIUM"
    };

    let log_entry = format!(
        "{{\"ip\":\"{}\",\"connections\":{},\"threat\":\"{}\",\"timestamp\":\"{}\"}}\n",
        ip,
        connections,
        threat,
        timestamp
    );

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
    {
        let _ = file.write_all(log_entry.as_bytes());
    }

}
