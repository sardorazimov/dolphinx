use std::env;

#[derive(Clone)]
pub struct Config {

    pub target: String,
    pub connections: usize,
    pub concurrency: usize,

    pub hold: bool,
    pub infinite: bool,
    pub http: bool,

    pub rate: Option<u64>, // connections per second

}

impl Config {

    pub fn from_args() -> Self {

        let args: Vec<String> = env::args().collect();

        if args.len() < 4 {

            println!("Usage:");
            println!("multi-stresser <target> <connections> <concurrency> [hold] [infinite] [http] [rate N]");
            std::process::exit(1);

        }

        let rate = parse_rate(&args);

        Self {

            target: args[1].clone(),
            connections: args[2].parse().unwrap(),
            concurrency: args[3].parse().unwrap(),

            hold: args.contains(&"hold".to_string()),
            infinite: args.contains(&"infinite".to_string()),
            http: args.contains(&"http".to_string()),

            rate,

        }

    }

}

fn parse_rate(args: &[String]) -> Option<u64> {

    for i in 0..args.len() {

        if args[i] == "rate" {

            if i + 1 < args.len() {

                return args[i + 1].parse().ok();

            }

        }

    }

    None

}
