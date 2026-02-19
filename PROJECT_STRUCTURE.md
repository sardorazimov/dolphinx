# dolphinx/
│
├──  Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── SECURITY.md
├── CONTRIBUTING.md
├── PROJECT_STRUCTURE.md
│
├── .gitignore
├── .github/
│   └── workflows/
│       └── rust.yaml
│
├── src/
│   │
│   ├── main.rs
│   │
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── parser.rs
│   │   └── commands.rs
│   │
│   ├── core/
│   │   ├── mod.rs
│   │   ├── engine.rs
│   │   ├── worker.rs
│   │   ├── network.rs
│   │   ├── socket.rs
│   │   └── runtime.rs
│   │
│   ├── metrics/
│   │   ├── mod.rs
│   │   ├── stats.rs
│   │   ├── monitor.rs
│   │   └── benchmark.rs
│   │
│   ├── attack/
│   │   ├── mod.rs
│   │   │
│   │   ├── scan/
│   │   │   ├── mod.rs
│   │   │   ├── port/
│   │   │   │   ├── mod.rs
│   │   │   │   └── scanner.rs
│   │   │   │
│   │   │   └── service/
│   │   │       ├── mod.rs
│   │   │       └── fingerprint.rs
│   │   │
│   │   ├── stress/
│   │   │   ├── mod.rs
│   │   │   ├── tcp.rs
│   │   │   ├── http.rs
│   │   │   └── slowloris.rs
│   │   │
│   │   ├── fuzz/
│   │   │   ├── mod.rs
│   │   │   └── fuzzer.rs
│   │   │
│   │   └── exploit/
│   │       ├── mod.rs
│   │       └── templates.rs
│   │
│   ├── defense/
│   │   ├── mod.rs
│   │   ├── analyzer.rs
│   │   ├── firewall.rs
│   │   ├── detector.rs
│   │   └── blacklist.rs
│   │
│   └── utils/
│       ├── mod.rs
│       ├── logger.rs
│       ├── banner.rs
│       └── config.rs
│
│
├── docs/
│   ├── architecture.md
│   ├── modules.md
│   ├── cli.md
│   └── roadmap.md
│
├── logs/
│   ├── attack.log
│   ├── benchmark.json
│   └── defense.log
│
└── target/
