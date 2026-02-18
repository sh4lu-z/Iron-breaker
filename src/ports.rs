// src/ports.rs
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use std::net::IpAddr;

pub async fn scan_full_range(ip: IpAddr, concurrency: usize) {
    println!("\n[+] Starting Full Range Port Scan (1-65535) on {}...", ip);
    println!("    (This might take a while. Press Ctrl+C to stop if needed)\n");

    let mut open_ports = Vec::new();
    let mut tasks = Vec::new();


    for port in 1..65535 {
        let ip_clone = ip;
        
   (Batching)
        if tasks.len() >= concurrency {
            for task in tasks.drain(..) {
                if let Ok(Some(p)) = task.await {
                    open_ports.push(p);
                }
            }
        }

        tasks.push(tokio::spawn(async move {
            let address = format!("{}:{}", ip_clone, port);
            if let Ok(Ok(_)) = timeout(Duration::from_millis(500), TcpStream::connect(&address)).await {
                println!("    ✅ Port {} is OPEN!", port);
                return Some(port);
            }
            None
        }));
    }


    for task in tasks {
        if let Ok(Some(p)) = task.await {
            open_ports.push(p);
        }
    }

    println!("\n[=] Scan Complete. Found {} open ports.", open_ports.len());

}
