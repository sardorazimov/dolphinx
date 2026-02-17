use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};

use std::sync::Arc;
use std::sync::atomic::Ordering;

use crate::stats::Stats;

pub async fn connect_worker(
    target: String,
    stats: Arc<Stats>,
    hold: bool,
) {

    match TcpStream::connect(&target).await {

        Ok(_) => {

            stats.success.fetch_add(1, Ordering::Relaxed);

            if hold {
                sleep(Duration::from_secs(60)).await;
            }

        }

        Err(_) => {

            stats.failed.fetch_add(1, Ordering::Relaxed);

        }

    }

}
