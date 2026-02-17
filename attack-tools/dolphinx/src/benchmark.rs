use std::sync::atomic::Ordering;

use tokio::time::{sleep, Duration};

use crate::stats::Stats;
use crate::worker;

use std::fs::File;
use std::io::Write;

use chrono::Utc;


pub async fn run(
    target: String,
    report_file: Option<String>,
) {

    println!("Benchmark started...\n");

    let mut rate = 100;
    let mut max_stable = 0;
    let mut peak_tested = 0;
    let mut final_efficiency = 0.0;

    loop {

        println!("Testing {} conn/sec...", rate);

        let stats = Stats::new();

        let duration = 5;

        for _ in 0..(rate * duration) {

            let stats_clone = stats.clone();
            let target_clone = target.clone();

            tokio::spawn(async move {

                worker::connect_worker(
                    target_clone,
                    stats_clone,
                    false
                ).await;

            });

            sleep(Duration::from_secs_f64(
                1.0 / rate as f64
            )).await;

        }

        sleep(Duration::from_secs(duration)).await;

        let success =
            stats.success.load(Ordering::Relaxed);

        let failed =
            stats.failed.load(Ordering::Relaxed);

        let total = success + failed;

        let efficiency =
            if total > 0 {
                success as f64 / total as f64
            } else {
                1.0
            };

        peak_tested = rate;
        final_efficiency = efficiency * 100.0;

        if efficiency > 0.99 {

            max_stable = rate;
            rate *= 2;

        } else {

            break;

        }

        if rate > 100000 {
            break;
        }

    }

    println!("\nRESULT");
    println!("------");

    println!(
        "Max stable speed: {} conn/sec",
        max_stable
    );

    println!(
        "Peak tested: {} conn/sec",
        peak_tested
    );

    println!(
        "Efficiency: {:.2}%",
        final_efficiency
    );

    // WRITE REPORT
    if let Some(path) = report_file {

        let timestamp = Utc::now().to_rfc3339();

        let json = format!(
            r#"{{
  "target": "{}",
  "max_stable_conn_per_sec": {},
  "peak_tested_conn_per_sec": {},
  "efficiency": {:.2},
  "timestamp": "{}"
}}"#,
            target,
            max_stable,
            peak_tested,
            final_efficiency,
            timestamp
        );

        let mut file =
            File::create(path).unwrap();

        file.write_all(json.as_bytes()).unwrap();

        println!("\nReport saved.");

    }

}
