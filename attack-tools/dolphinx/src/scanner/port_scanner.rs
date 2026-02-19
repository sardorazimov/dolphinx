use tokio::net::TcpStream;
use std::time::Duration;

pub async fn scan_ports(host: &str) -> Vec<u16> {

    let common_ports = vec![
        21, 22, 23, 25,
        53, 80, 110, 143,
        443, 445, 3306, 3389, 8080
    ];

    let mut open_ports = Vec::new();

    for port in common_ports {

        let addr = format!("{}:{}", host, port);

        if tokio::time::timeout(
            Duration::from_secs(1),
            TcpStream::connect(&addr)
        ).await.is_ok() {

            open_ports.push(port);
        }
    }

    open_ports
}
