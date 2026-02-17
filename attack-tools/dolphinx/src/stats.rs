use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use tokio::time::{sleep, Duration};

use colored::*;

use crate::metrics::Metrics;


pub struct Stats {

    pub success: AtomicU64,
    pub failed: AtomicU64,
    pub metrics: Metrics,

}


impl Stats {

    pub fn new() -> Arc<Self> {

        Arc::new(Self {

            success: AtomicU64::new(0),
            failed: AtomicU64::new(0),
            metrics: Metrics::new(),

        })

    }


    pub fn inc_success(&self) {

        self.success.fetch_add(1, Ordering::Relaxed);

    }


    pub fn inc_failed(&self) {

        self.failed.fetch_add(1, Ordering::Relaxed);

    }


    pub fn start_printer(stats: Arc<Self>) {

        tokio::spawn(async move {

            let mut last_success = 0;

            loop {

                sleep(Duration::from_secs(1)).await;

                let success =
                    stats.success.load(Ordering::Relaxed);

                let failed =
                    stats.failed.load(Ordering::Relaxed);

                let speed =
                    success.saturating_sub(last_success);

                last_success = success;

                // DOĞRU kullanım
                stats.metrics.update(speed);

                let peak =
                    stats.metrics.peak();

                let avg =
                    stats.metrics.avg_speed();

                println!(
                    "{} ✓ {} ✗ {} ⚡ {} peak={} avg={}",
                    "[STATS]".cyan(),
                    success.to_string().green(),
                    failed.to_string().red(),
                    speed.to_string().yellow(),
                    peak.to_string().magenta(),
                    avg.to_string().blue()
                );

            }

        });

    }

}
