mod config;
mod stats;
mod worker;
mod modes;

use config::Config;
use stats::Stats;

use std::sync::Arc;
use std::env;
use std::sync::atomic::Ordering;

use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

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


fn print_help() {

    println!("Usage:");
    println!("  dolphinx <target> <connections> <concurrency> [options]");
    println!();
    println!("Options:");
    println!("  hold           Keep connections open");
    println!("  infinite       Run forever");
    println!("  http           Use HTTP mode");
    println!("  rate N         Connections per second");
    println!();
    println!("Examples:");
    println!("  dolphinx 127.0.0.1:8081 1000 100");
    println!("  dolphinx http://127.0.0.1:8081 10000 200 http");
    println!("  dolphinx 127.0.0.1:8081 0 500 hold infinite rate 1000");
    println!();
    println!("Flags:");
    println!("  --help, -h     Show help");
    println!("  --version, -v  Show version");

}


#[tokio::main]
async fn main() {

    let args: Vec<String> = env::args().collect();

    // version
    if args.contains(&"--version".to_string()) ||
       args.contains(&"-v".to_string())
    {
        println!("dolphinx {}", VERSION);
        return;
    }

    // help
    if args.contains(&"--help".to_string()) ||
       args.contains(&"-h".to_string())
    {
        print_banner();
        print_help();
        return;
    }

    print_banner();

    let config = Config::from_args();

    println!("Target      : {}", config.target);
    println!("Connections : {}", config.connections);
    println!("Concurrency : {}", config.concurrency);
    println!("HTTP mode   : {}", config.http);
    println!("Hold mode   : {}", config.hold);
    println!("Infinite    : {}", config.infinite);
    println!("Rate limit  : {:?}", config.rate);
    println!();

    let semaphore =
        Arc::new(Semaphore::new(config.concurrency));

    let stats = Stats::new();

    Stats::start_printer(stats.clone());

    run(config, semaphore, stats).await;

}


async fn run(
    config: Config,
    semaphore: Arc<Semaphore>,
    stats: Arc<Stats>,
) {

    let mut rate = config.rate.unwrap_or(100);
    let mut last_failed = 0;

    loop {

        spawn_connection(
            &config,
            semaphore.clone(),
            stats.clone()
        ).await;

        sleep(Duration::from_secs_f64(
            1.0 / rate as f64
        )).await;

        if config.adaptive {

            let failed =
                stats.failed.load(Ordering::Relaxed);

            if failed > last_failed {

                rate = (rate as f64 * 0.8) as u64;

            } else {

                rate = (rate as f64 * 1.1) as u64;

            }

            rate = rate.clamp(10, 100000);

            last_failed = failed;

        }

        if !config.infinite &&
           stats.success.load(Ordering::Relaxed)
           >= config.connections as u64
        {
            break;
        }

    }

}



async fn spawn_connection(
    config: &Config,
    semaphore: Arc<Semaphore>,
    stats: Arc<Stats>,
) {

    let permit =
        semaphore.clone().acquire_owned()
        .await
        .unwrap();

    let target = config.target.clone();
    let stats_clone = stats.clone();

    if config.http {

        tokio::spawn(async move {

            modes::request::send_http_request(
                target,
                stats_clone
            ).await;

            drop(permit);

        });

    }
    else {

        let hold = config.hold;

        tokio::spawn(async move {

            worker::connect_worker(
                target,
                stats_clone,
                hold
            ).await;

            drop(permit);

        });

    }

}
