use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;

use chrono::{Utc, Datelike};

pub fn log_attack(ip: &str, connections: usize) {

    // ensure logs directory exists
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
