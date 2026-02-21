mod logger;

use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;

use chrono::Utc;




const ATTACK_THRESHOLD: usize = 100;


#[tokio::main]
async fn main() {

    let listener = TcpListener::bind("0.0.0.0:8081")
        .await
        .expect("Failed to bind");

    println!("Analyzer listening on port 8081");

    let connections: Arc<Mutex<HashMap<String, usize>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let blocked_ips: Arc<Mutex<HashSet<String>>> =
        Arc::new(Mutex::new(HashSet::new()));

    loop {

        let (mut socket, addr) = listener.accept().await.unwrap();

        let ip = addr.ip().to_string();

        // Drop connection immediately if IP is blocked
        {
            let blocked = blocked_ips.lock().unwrap();
            if blocked.contains(&ip) {
                println!("BLOCKED: {} (connection dropped)", ip);
                drop(socket);
                continue;
            }
        }

        let connections = connections.clone();
        let blocked_ips = blocked_ips.clone();

        tokio::spawn(async move {

            let mut buffer = [0; 1024];

            let _ = socket.read(&mut buffer).await;

            let mut map = connections.lock().unwrap();

            let count = map.entry(ip.clone()).or_insert(0);

            *count += 1;

            println!("{} -> {}", ip, count);

            if *count == ATTACK_THRESHOLD {

                println!("ATTACK DETECTED: {} — blocking IP", ip);

                logger::log_attack(&ip, *count);

                save_attack_info(&ip, *count);

                // Block the IP (move ip, no more uses after this)
                blocked_ips.lock().unwrap().insert(ip);

            }

        });

    }

}


fn save_attack_info(ip: &str, connections: usize) {

    create_dir_all("logs").ok();

    let timestamp = Utc::now().to_rfc3339();

    let threat = if connections > 1000 {
        "CRITICAL"
    } else if connections > 500 {
        "HIGH"
    } else {
        "MEDIUM"
    };

    let entry = format!(
        "{{\"ip\":\"{}\",\"connections\":{},\"threat\":\"{}\",\"timestamp\":\"{}\",\"action\":\"blocked\"}}\n",
        ip,
        connections,
        threat,
        timestamp
    );

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/attack-info.json")
    {
        let _ = file.write_all(entry.as_bytes());
    }

}
