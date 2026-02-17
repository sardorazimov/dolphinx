mod config;
mod stats;
mod worker;
mod modes;

use config::Config;
use stats::Stats;

use std::sync::Arc;
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {

    // Load config
    let config = Config::from_args();

    println!("==============================");
    println!("Multi-Stresser Started");
    println!("Target      : {}", config.target);
    println!("Connections : {}", config.connections);
    println!("Concurrency : {}", config.concurrency);
    println!("Hold mode   : {}", config.hold);
    println!("Infinite    : {}", config.infinite);
    println!("HTTP mode   : {}", config.http);
    println!("==============================");

    // Semaphore for concurrency control
    let semaphore = Arc::new(Semaphore::new(config.concurrency));

    // Stats
    let stats = Stats::new();

    // Start live stats printer
    Stats::start_printer(stats.clone());

    // Mode selection
    if config.http {

        run_http_mode(
            config.target,
            config.connections,
            semaphore,
            stats,
            config.infinite,
        ).await;

    }
    else if config.hold {

        modes::hold::run(
            config.target,
            config.connections,
            semaphore,
            stats,
            config.infinite,
        ).await;

    }
    else {

        modes::connect::run(
            config.target,
            config.connections,
            semaphore,
            stats,
        ).await;

    }

}

async fn run_http_mode(
    target: String,
    connections: usize,
    semaphore: Arc<Semaphore>,
    stats: Arc<Stats>,
    infinite: bool,
) {

    if infinite {

        loop {

            let permit = semaphore.clone().acquire_owned().await.unwrap();

            let target = target.clone();
            let stats = stats.clone();

            tokio::spawn(async move {

                modes::request::send_http_request(target, stats).await;

                drop(permit);

            });

        }

    } else {

        let mut handles = Vec::new();

        for _ in 0..connections {

            let permit = semaphore.clone().acquire_owned().await.unwrap();

            let target = target.clone();
            let stats = stats.clone();

            let handle = tokio::spawn(async move {

                modes::request::send_http_request(target, stats).await;

                drop(permit);

            });

            handles.push(handle);

        }

        for h in handles {
            let _ = h.await;
        }

    }

}
