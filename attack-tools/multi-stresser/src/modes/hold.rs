use std::sync::Arc;
use tokio::sync::Semaphore;

use crate::stats::Stats;
use crate::worker::connect_worker;

pub async fn run(
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

                connect_worker(target, stats, true).await;

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

                connect_worker(target, stats, true).await;

                drop(permit);

            });

            handles.push(handle);

        }

        for h in handles {
            let _ = h.await;
        }

    }

}
