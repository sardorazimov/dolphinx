mod config;
mod stats;
mod worker;
mod modes;

use config::Config;
use stats::Stats;

use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {

    let config = Config::from_args();

    println!("==============================");
    println!("Multi-Stresser Started");
    println!("Target      : {}", config.target);
    println!("Connections : {}", config.connections);
    println!("Concurrency : {}", config.concurrency);
    println!("Hold mode   : {}", config.hold);
    println!("Infinite    : {}", config.infinite);
    println!("HTTP mode   : {}", config.http);
    println!("Rate limit  : {:?}", config.rate);
    println!("==============================");

    let semaphore = Arc::new(Semaphore::new(config.concurrency));

    let stats = Stats::new();

    Stats::start_printer(stats.clone());

    run_with_rate(config, semaphore, stats).await;

}

async fn run_with_rate(
    config: Config,
    semaphore: Arc<Semaphore>,
    stats: Arc<Stats>,
) {

    let delay = match config.rate {

        Some(rate) => Duration::from_secs_f64(1.0 / rate as f64),

        None => Duration::from_secs(0),

    };

    if config.infinite {

        loop {

            spawn_connection(&config, semaphore.clone(), stats.clone()).await;

            if delay.as_nanos() > 0 {
                sleep(delay).await;
            }

        }

    } else {

        for _ in 0..config.connections {

            spawn_connection(&config, semaphore.clone(), stats.clone()).await;

            if delay.as_nanos() > 0 {
                sleep(delay).await;
            }

        }

    }

}

async fn spawn_connection(
    config: &Config,
    semaphore: Arc<Semaphore>,
    stats: Arc<Stats>,
) {

    let permit = semaphore.clone().acquire_owned().await.unwrap();

    let target = config.target.clone();
    let stats_clone = stats.clone();

    if config.http {

        tokio::spawn(async move {

            modes::request::send_http_request(target, stats_clone).await;

            drop(permit);

        });

    } else {

        let hold = config.hold;

        tokio::spawn(async move {

            worker::connect_worker(target, stats_clone, hold).await;

            drop(permit);

        });

    }

}
