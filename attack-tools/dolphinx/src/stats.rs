use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use tokio::time::{sleep, Duration};

use colored::*;


pub struct Stats {
    pub success: AtomicU64,
    pub failed: AtomicU64,
}


impl Stats {

    pub fn new() -> Arc<Self> {

        Arc::new(Self {
            success: AtomicU64::new(0),
            failed: AtomicU64::new(0),
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
            let mut peak_speed = 0;
            let mut total_speed = 0;
            let mut seconds = 0;

            loop {

                sleep(Duration::from_secs(1)).await;

                let success =
                    stats.success.load(Ordering::Relaxed);

                let failed =
                    stats.failed.load(Ordering::Relaxed);

                let speed =
                    success.saturating_sub(last_success);

                last_success = success;

                seconds += 1;
                total_speed += speed;

                let avg_speed =
                    if seconds > 0 {
                        total_speed / seconds
                    } else {
                        0
                    };

                if speed > peak_speed {
                    peak_speed = speed;
                }

                let total = success + failed;

                let efficiency =
                    if total > 0 {
                        (success as f64 / total as f64) * 100.0
                    } else {
                        100.0
                    };

                // terminal title
                print!(
                    "\x1b]0;dolphinx | {} conn/sec\x07",
                    speed
                );

                println!(
                    "{} ✓ {} ✗ {} ⚡ {} peak={} avg={} eff={:.2}%",
                    "[STATS]".cyan(),
                    success.to_string().green(),
                    failed.to_string().red(),
                    speed.to_string().yellow(),
                    peak_speed.to_string().magenta(),
                    avg_speed.to_string().blue(),
                    efficiency.to_string().green()
                );

            }

        });

    }

}
