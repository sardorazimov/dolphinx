use std::process::Command;
use std::collections::{ HashMap, HashSet };
use std::sync::{ Arc, Mutex };
use tokio::time::{ sleep, Duration };
use serde::{ Serialize }; // JSON formatı için
use std::fs::File;
use std::io::Write;
use chrono::Local; // İstersen ekle: cargo add chrono

#[derive(Serialize)]
struct AttackLog {
    ip: String,
    timestamp: String,
    peak_rps: u32,
    action: String,
}

#[tokio::main]
async fn main() {
    File::create("BASLADI_MI.txt").unwrap();
    let traffic_stats = Arc::new(Mutex::new(HashMap::<String, u32>::new()));
    let banned_ips = Arc::new(Mutex::new(HashSet::<String>::new()));
    let attack_logs = Arc::new(Mutex::new(Vec::<AttackLog>::new()));

    let stats_clone = Arc::clone(&traffic_stats);
    let banned_clone = Arc::clone(&banned_ips);
    let logs_clone = Arc::clone(&attack_logs);
    // ... önceki kodlar
    let json_data = serde_json::to_string_pretty(&*logs_clone.lock().unwrap()).unwrap();
    let path = std::env::current_dir().unwrap().join("attack_reports.json"); // Tam yolu al

    let mut file = File::create(&path).unwrap();
    file.write_all(json_data.as_bytes()).unwrap();

    println!("\x1b[32m[+] RAPOR KAYDEDİLDİ: {:?}\x1b[0m", path); // Yolu ekrana bas
    // ...

    println!("\x1b[36m[SYSTEM] ADS & LOGGING UNIT STARTED...\x1b[0m");

    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(1)).await;
            let mut stats = stats_clone.lock().unwrap();
            let mut blist = banned_clone.lock().unwrap();
            let mut logs = logs_clone.lock().unwrap();

            for (ip, rps) in stats.iter() {
                if *rps > 300 && !blist.contains(ip) {
                    // 1. BANLAMA (A Şıkkı)
                    Command::new("sudo")
                        .args(["iptables", "-A", "INPUT", "-s", ip, "-j", "DROP"])
                        .output()
                        .ok();

                    // 2. RAPORLAMA (B Şıkkı)
                    let new_log = AttackLog {
                        ip: ip.clone(),
                        timestamp: format!("{:?}", Local::now()), // Veya basitçe bir String
                        peak_rps: *rps,
                        action: "BANNED_BY_ADS".to_string(),
                    };
                    logs.push(new_log);

                    // JSON Dosyasına Yaz
                    let json_data = serde_json::to_string_pretty(&*logs).unwrap();
                    let mut file = File::create("attack_reports.json").unwrap();
                    file.write_all(json_data.as_bytes()).unwrap();

                    println!("\x1b[31m[!] ALERT: {} banned. Log updated in attack_reports.json\x1b[0m", ip);
                    blist.insert(ip.clone());
                }
            }
            stats.clear();
        }
    });

    // Simülasyon
    loop {
        {
            let mut stats = traffic_stats.lock().unwrap();
            *stats.entry("127.0.0.1".to_string()).or_insert(0) += 1;
        }
        sleep(Duration::from_micros(200)).await;
    }
}
