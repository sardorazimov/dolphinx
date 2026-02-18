mod config;
mod stats;
mod worker;
mod modes;
mod metrics;
mod benchmark;
mod attack;



use config::Config;
use stats::Stats;

use std::sync::Arc;
use std::env;

use tokio::sync::Semaphore;
use tokio::time::{ sleep, Duration };

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_banner() {
    println!(r#"
██████╗  ██████╗ ██╗     ██████╗ ██╗  ██╗██╗███╗   ██╗██╗  ██╗
██╔══██╗██╔═══██╗██║     ██╔══██╗██║  ██║██║████╗  ██║╚██╗██╔╝
██║  ██║██║   ██║██║     ██████╔╝███████║██║██╔██╗ ██║ ╚███╔╝
██║  ██║██║   ██║██║     ██╔═══╝ ██╔══██║██║██║╚██╗██║ ██╔██╗
██████╔╝╚██████╔╝███████╗██║     ██║  ██║██║██║ ╚████║██╔╝ ██╗
╚═════╝  ╚═════╝ ╚══════╝╚═╝     ╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝

DOLPHINX v{}
"#, VERSION);
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
        attack::scan::port::scanner::scan_target(
        "127.0.0.1",
        1,
        100,
        50
    ).await;


    // VERSION
    if args.contains(&"--version".to_string()) || args.contains(&"-v".to_string()) {
        println!("dolphinx {}", VERSION);
        return;
    }
    // SCAN MODE
    if args.len() >= 3 && args[1] == "scan" {
        let target = args[2].clone();

        println!("Starting scan on {}", target);

        attack::scan::port::scanner::scan_target(&target, 1, 1024, 500).await;

        return;
    }

    // HELP
    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        print_banner();
        println!("Usage:");
        println!("dolphinx <target> <connections> <concurrency>");
        println!("dolphinx --benchmark <target>");
        return;
    }

    // BENCHMARK MODE
    if args.contains(&"--benchmark".to_string()) {
        if args.len() < 3 {
            println!("Usage:");
            println!("dolphinx --benchmark <target>");
            return;
        }

        print_banner();

        let target = args[2].clone();

        let report = if args.contains(&"--report".to_string()) {
            let index = args
                .iter()
                .position(|x| x == "--report")
                .unwrap();

            Some(args[index + 1].clone())
        } else {
            None
        };

        benchmark::run(target, report).await;

        return;
    }

    // NORMAL MODE
    print_banner();

    let config = Config::from_args();

    println!("Target      : {}", config.target);
    println!("Connections : {}", config.connections);
    println!("Concurrency : {}", config.concurrency);
    println!();

    let semaphore = Arc::new(Semaphore::new(config.concurrency));

    let stats = Stats::new();

    Stats::start_printer(stats.clone());

    run(config, semaphore, stats).await;
}

async fn run(config: Config, semaphore: Arc<Semaphore>, stats: Arc<Stats>) {
    let delay = match config.rate {
        Some(rate) => Duration::from_secs_f64(1.0 / (rate as f64)),

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

async fn spawn_connection(config: &Config, semaphore: Arc<Semaphore>, stats: Arc<Stats>) {
    let target = config.target.clone();

    let hold = config.hold; // ✅ COPY değer

    let stats_clone = stats.clone();

    tokio::spawn(async move {
        let permit = semaphore.acquire().await.unwrap();

        worker::connect_worker(
            target,
            stats_clone,
            hold // ✅ artık safe
        ).await;

        drop(permit);
    });
}
