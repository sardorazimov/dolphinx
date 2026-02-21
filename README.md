```
██████╗  ██████╗ ██╗     ██████╗ ██╗  ██╗██╗███╗   ██╗██╗  ██╗
██╔══██╗██╔═══██╗██║     ██╔══██╗██║  ██║██║████╗  ██║╚██╗██╔╝
██║  ██║██║   ██║██║     ██████╔╝███████║██║██╔██╗ ██║ ╚███╔╝
██║  ██║██║   ██║██║     ██╔═══╝ ██╔══██║██║██║╚██╗██║ ██╔██╗
██████╔╝╚██████╔╝███████╗██║     ██║  ██║██║██║ ╚████║██╔╝ ██╗
╚═════╝  ╚═════╝ ╚══════╝╚═╝     ╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝

                     DOLPHINX  v2.1.3
         Network Attack & Defense Lab — Rust powered
```

---

## Table of Contents

- [About](#about)
- [Installation](#installation)
  - [Kali Linux / Debian / Ubuntu](#kali-linux--debian--ubuntu)
  - [macOS](#macos)
  - [Windows](#windows)
  - [Build from Source](#build-from-source)
- [Usage](#usage)
  - [Help](#help)
  - [Stress / Attack Mode](#stress--attack-mode)
  - [Port Scan Mode](#port-scan-mode)
  - [Recon Mode](#recon-mode)
  - [Benchmark Mode](#benchmark-mode)
- [Defense Lab](#defense-lab)
- [License](#license)

---

## About

**dolphinx** is a Rust-based network security lab for simulating and analyzing network attacks and defenses in a controlled environment.

> ⚠️ For **authorized testing only**. Do not use against systems you do not own.

---

## Installation

### Kali Linux / Debian / Ubuntu

```bash
# 1. Update and install dependencies
sudo apt update && sudo apt install -y curl build-essential git

# 2. Install Rust (via rustup)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# 3. Clone the repository
git clone https://github.com/sardorazimov/dolphinx.git
cd dolphinx

# 4. Build the tool
cargo build --release -p dolphinx

# 5. (Optional) Install globally
sudo cp target/release/dolphinx /usr/local/bin/dolphinx
```

### macOS

```bash
# 1. Install Homebrew (if not installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 2. Install Rust
brew install rust

# 3. Clone the repository
git clone https://github.com/sardorazimov/dolphinx.git
cd dolphinx

# 4. Build the tool
cargo build --release -p dolphinx

# 5. (Optional) Install globally
sudo cp target/release/dolphinx /usr/local/bin/dolphinx
```

### Windows

```powershell
# 1. Install Rust from https://rustup.rs
# Run the installer, then open a new terminal

# 2. Clone the repository
git clone https://github.com/sardorazimov/dolphinx.git
cd dolphinx

# 3. Build the tool
cargo build --release -p dolphinx

# Binary will be at: target\release\dolphinx.exe
```

### Build from Source

```bash
git clone https://github.com/sardorazimov/dolphinx.git
cd dolphinx
cargo build --release -p dolphinx
./target/release/dolphinx --help
```

---

## Usage

### Help

```bash
dolphinx --help
dolphinx -h
```

### Version

```bash
dolphinx --version
dolphinx -v
```

---

### Stress / Attack Mode

Send a flood of connections to a target (for load testing your own servers).

```bash
# dolphinx <target> <connections> <concurrency>
dolphinx 127.0.0.1:8081 10000 1000
```

**Hold mode** — keep connections open:

```bash
dolphinx 127.0.0.1:8081 10000 1000 hold
```

**Infinite mode** — loop forever:

```bash
dolphinx 127.0.0.1:8081 10000 1000 infinite
```

**Infinite hold mode** — infinite + keep connections open:

```bash
dolphinx 127.0.0.1:8081 10000 1000 hold infinite
```

| Argument       | Description                                      |
|----------------|--------------------------------------------------|
| `<target>`     | `host:port` of the target server                 |
| `<connections>`| Total number of connections to make              |
| `<concurrency>`| Maximum simultaneous connections                 |
| `hold`         | Keep each connection open instead of closing it  |
| `infinite`     | Repeat the attack loop indefinitely              |

---

### Port Scan Mode

Fast TCP port scan against a target host.

```bash
dolphinx scan <target>
```

Example:

```bash
dolphinx scan 192.168.1.1
```

Scans ports 1–1024 with 500 concurrent workers.

---

### Recon Mode

Service detection and reconnaissance. Saves results to `defense-lab/recon.json`.

```bash
dolphinx recon <target>
```

Example:

```bash
dolphinx recon 192.168.1.1
```

Output is saved automatically:

```
defense-lab/recon.json
```

---

### Benchmark Mode

Measure connection throughput against a target.

```bash
dolphinx --benchmark <target>
```

Save benchmark report to a file:

```bash
dolphinx --benchmark <target> --report report.json
```

---

## Defense Lab

The `defense-lab/analyzer` module monitors for suspicious connection floods and logs attack data.

```bash
cd defense-lab/analyzer
cargo run --release
```

- Detects connection floods
- Tracks IP addresses
- Logs attack data to JSON (`logs/attack-YYYY-MM-DD.json`)

---

## License

See [license](license) for details.
