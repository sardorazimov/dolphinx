use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

use futures::stream::{FuturesUnordered, StreamExt};


pub async fn scan_target(
    target: &str,
    start_port: u16,
    end_port: u16,
    concurrency: usize,
) {

    println!(
        "Scanning {} ports {}-{} with {} workers",
        target,
        start_port,
        end_port,
        concurrency
    );

    let semaphore =
        std::sync::Arc::new(
            tokio::sync::Semaphore::new(concurrency)
        );

    let mut tasks =
        FuturesUnordered::new();

    for port in start_port..=end_port {

        let permit =
            semaphore.clone()
            .acquire_owned()
            .await
            .unwrap();

        let target =
            target.to_string();

        tasks.push(tokio::spawn(async move {

            let addr =
                format!("{}:{}", target, port);

            let result =
                timeout(
                    Duration::from_millis(800),
                    TcpStream::connect(&addr)
                ).await;

            drop(permit);

            match result {

                Ok(Ok(_)) => {

                    println!(
                        "[OPEN] {}",
                        addr
                    );

                }

                _ => {}

            }

        }));

    }

    while tasks.next().await.is_some() {}

    println!("Scan complete.");

}
