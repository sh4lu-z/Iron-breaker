use reqwest::Client;
use regex::Regex;
use std::collections::HashSet;

pub async fn crawl(client: &Client, url: &str) {
    println!("\n[🕷️] Starting Web Spider on {}...", url);
    println!("    (Hunting for Emails, Links, and Secrets...)\n");

    match client.get(url).send().await {
        Ok(resp) => {
            let body = resp.text().await.unwrap_or_default();
            
            //  (Email Extraction)
           
            let email_regex = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
            let mut emails = HashSet::new();

            for cap in email_regex.captures_iter(&body) {
                emails.insert(cap[0].to_string());
            }

            if !emails.is_empty() {
                println!("    📧 EMAILS FOUND:");
                for email in emails {
                    println!("       - {}", email);
                }
            } else {
                println!("    🚫 No emails found on the homepage.");
            }

            // 2. (Link Extraction)
            let link_regex = Regex::new(r#"href="(https?://[^"]+)""#).unwrap();
            let mut links = HashSet::new();

            println!("\n    🔗 EXTERNAL LINKS FOUND:");
            for cap in link_regex.captures_iter(&body) {
                let link = cap[1].to_string();
            
                if links.insert(link.clone()) {
                    println!("       - {}", link);
                }
            }

            // 3.(Developer Comments / API Keys)
            if body.contains("TODO") || body.contains("FIXME") || body.contains("API_KEY") {
                println!("\n    ⚠️  SUSPICIOUS COMMENTS DETECTED:");
                println!("       - The source code contains 'TODO' or 'API_KEY'. Check manually!");
            }

        }
        Err(e) => println!("❌ Failed to connect: {}", e),
    }
    
    println!("\n✅ Spider Scan Complete.");
}