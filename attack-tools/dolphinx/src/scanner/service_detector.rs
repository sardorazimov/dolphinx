use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::time::Duration;
use crate::scanner::version_detector::detect_version;
use crate::scanner::http_probe::probe_http;
use crate::scanner::tcp_probe::probe_tcp;
use serde::Serialize;




#[derive(Debug, Clone, Serialize)]
pub struct ServiceInfo {
    pub host: String,
    pub port: u16,
    pub service: String,
    pub product: Option<String>,
    pub version: Option<String>,

    pub banner: Option<String>,
}

pub async fn detect_service(host: &str, port: u16) -> ServiceInfo {

    let mut banner = grab_banner(host, port).await;

    // HTTP probe
    if banner.is_none() && (port == 80 || port == 8080 || port == 8000) {

        banner = probe_http(host, port).await;
    }

    // TCP fingerprint probe
    if banner.is_none() {

        banner = probe_tcp(host, port).await;
    }

    let service = identify_service(port, &banner);

    let version_info = detect_version(&banner);

    ServiceInfo {

        host: host.into(),
        port,

        service,

        product: version_info.product,
        version: version_info.version,

        banner,
    }
}


async fn grab_banner(host: &str, port: u16) -> Option<String> {

    let addr = format!("{}:{}", host, port);

    let mut stream = match tokio::time::timeout(
        Duration::from_secs(3),
        TcpStream::connect(&addr)
    ).await {

        Ok(Ok(s)) => s,
        _ => return None
    };

    let mut buffer = [0u8; 1024];

    match tokio::time::timeout(
        Duration::from_secs(2),
        stream.read(&mut buffer)
    ).await {

        Ok(Ok(n)) if n > 0 => {

            Some(String::from_utf8_lossy(&buffer[..n]).to_string())

        }

        _ => None
    }
}
fn identify_service(port: u16, banner: &Option<String>) -> String {

    if let Some(b) = banner {

        let b = b.to_uppercase();

        if b.contains("SSH") {
            return "ssh".into();
        }

        if b.contains("HTTP") {
            return "http".into();
        }

        if b.contains("FTP") {
            return "ftp".into();
        }
    }

    match port {

        22 => "ssh".into(),
        80 => "http".into(),
        443 => "https".into(),

        _ => "unknown".into()
    }
}
