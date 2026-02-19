use futures::future::join_all;

use crate::scanner::port_scanner::scan_ports;
use crate::scanner::service_detector::{
    detect_service,
    ServiceInfo,
};




#[derive(Debug, Clone)]
pub struct ScanResult {

    pub host: String,
    pub services: Vec<ServiceInfo>,
   
}

//
// MAIN ENTRY POINT
//
pub async fn scan_target(host: &str) -> ScanResult {

    // 1️⃣ Port scan
    let open_ports = scan_ports(host).await;

    // Eğer port yoksa boş result dön
    if open_ports.is_empty() {

        return ScanResult {

            host: host.to_string(),
            services: Vec::new(),
        };
    }

    // 2️⃣ Service detection parallel
    let services = detect_services_parallel(host, open_ports).await;

    ScanResult {

        host: host.to_string(),

        services,
    }
}

//
// PARALLEL SERVICE DETECTION
//
async fn detect_services_parallel(
    host: &str,
    ports: Vec<u16>,
) -> Vec<ServiceInfo> {

    let tasks = ports
        .into_iter()
        .map(|port| {

            let host = host.to_string();

            tokio::spawn(async move {

                detect_service(&host, port).await

            })
        });

    let results = join_all(tasks).await;

    let mut services = Vec::new();

    for result in results {

        if let Ok(service) = result {

            services.push(service);
        }
    }

    services
}

//
// OPTIONAL: CUSTOM PORT RANGE SCAN
//
pub async fn scan_target_with_ports(
    host: &str,
    ports: Vec<u16>,
) -> ScanResult {

    let services = detect_services_parallel(host, ports).await;

    ScanResult {

        host: host.to_string(),

        services,
    }
}

//
// OPTIONAL: FULL PORT SCAN (1–65535)
//
pub async fn scan_target_full(host: &str) -> ScanResult {

    let ports: Vec<u16> = (1..=65535).collect();

    scan_target_with_ports(host, ports).await
}
