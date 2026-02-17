use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering}
};

use tokio::time::{sleep, Duration};

pub struct Stats {
    pub success: AtomicUsize,
    pub failed: AtomicUsize,
}

impl Stats {

    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            success: AtomicUsize::new(0),
            failed: AtomicUsize::new(0),
        })
    }

    pub fn start_printer(stats: Arc<Self>) {

        tokio::spawn(async move {

            loop {

                let success = stats.success.load(Ordering::Relaxed);
                let failed = stats.failed.load(Ordering::Relaxed);

                println!("Success: {} | Failed: {}", success, failed);

                sleep(Duration::from_secs(1)).await;

            }

        });

    }

}
