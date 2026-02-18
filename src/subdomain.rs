use tokio::net::lookup_host;

pub async fn find_subdomains(domain: &str) -> Vec<(String, Vec<String>)> {
    let mut discovered = Vec::new();
   
    let sub_list = vec![
    // Infrastructure & Core
    "www", "mail", "remote", "blog", "webmail", "server", "ns1", "ns2", "smtp", "secure",
    "vpn", "api", "dev", "staging", "test", "portal", "support", "login", "app", "shop",
    "static", "img", "cpanel", "whm", "pop3", "imap", "cloud", "dns", "m", "mobile",

    // Development & Environments
    "admin", "beta", "alpha", "help", "news", "docs", "video", "drive", "db", "sql",
    "mysql", "postgres", "git", "gitlab", "internal", "local", "monitor", "dashboard", "stats", "billing",

    // Services & Apps
    "members", "forum", "store", "download", "updates", "files", "ftp", "ssh", "vps", "media",
    "search", "maps", "devops", "code", "demo", "sandbox", "jenkins", "jira", "confluence", "slack",

    // Management & Security
    "matrix", "chat", "backup", "storage", "archive", "firewall", "proxy", "gateway", "tracker", "auth",
    "identity", "oauth", "sso", "client", "customer", "feedback", "careers", "hr", "wiki", "intranet",

    // Enterprise & Others
    "office", "owa", "exchange", "lab", "registry", "docker", "k8s", "cluster", "prod", "production"
];

    println!("\n[i] 🔍 Phase 1: Subdomain Discovery on '{}'...", domain);

    for sub in sub_list {
        let target = format!("{}.{}", sub, domain);
        let host_port = format!("{}:80", target);

        println!("    🔍 Checking: {}", target); 

        if let Ok(addrs) = lookup_host(&host_port).await {
           
            let mut ips = Vec::new();
            for addr in addrs {
                ips.push(addr.ip().to_string());
            }

            if !ips.is_empty() {
                println!("    ✅ Found: {} -> IPs: {:?}", target, ips); 
                discovered.push((target.clone(), ips));
            }
        }
    }
    discovered
}