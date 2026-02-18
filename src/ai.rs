use reqwest::Client;
use serde_json::json;
use std::error::Error;

// මේක Allow dead code දැම්මේ warning එන එක නවත්වන්න
#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct AttackTarget {
    pub path: String,
    pub payload: String,
}

const SYSTEM_PROMPT: &str = "You are a Red Team Exploit Developer. \
Your goal is to identify specific attack vectors for the detected server version. \
Provide a list of 3 specific 'Attack Payloads' to test for identified CVEs. \
IMPORTANT: At the very end, strictly output a JSON array of objects with 'path' and 'payload' keys. \
Label it 'JSON_ATTACKS:'.";

// 1. පේලෝඩ් ඉල්ලන ෆන්ක්ෂන් එක
pub async fn ask_groq(api_key: &str, banner: &str) -> Result<Vec<AttackTarget>, Box<dyn Error + Send + Sync>> {
    let client = Client::new();
    
    let payload = json!({
        "model": "llama-3.3-70b-versatile", // අලුත් මොඩල් එක
        "messages": [
            { "role": "system", "content": SYSTEM_PROMPT },
            { "role": "user", "content": format!("Target Banner: '{}'. Give me specific exploit test payloads.", banner) }
        ],
        "temperature": 0.5,
        "max_tokens": 1024
    });

    println!("    [?] 🧠 Consulting AI for Vulnerabilities...");

    let res = client.post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    if res.status().is_success() {
        let body: serde_json::Value = res.json().await?;
        if let Some(content) = body["choices"][0]["message"]["content"].as_str() {
            // JSON කොටස කඩා ගැනීම
            if let Some(start_index) = content.find("JSON_ATTACKS: ") {
                let json_str = &content[start_index + 14..].trim();
                if let Ok(attacks) = serde_json::from_str::<Vec<AttackTarget>>(json_str) {
                    return Ok(attacks);
                }
            }
        }
    }
    Ok(Vec::new())
}

// 2. ඇටෑක් ප්ලෑන් එක හදන ෆන්ක්ෂන් එක
pub async fn suggest_exploit(api_key: &str, target_info: &str) -> String {
    let client = Client::new();
    
    let prompt = format!(
        "You are an Elite Pentesting Assistant. Target Analysis: '{}'. 
        1. Identify potential CVEs.
        2. Suggest 3 advanced tools or specific commands (like metasploit modules, hydra, sqlmap arguments) to test this.
        Keep it technical and concise.", 
        target_info
    );

    let res = client.post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "llama-3.3-70b-versatile",
            "messages": [{"role": "user", "content": prompt}]
        }))
        .send().await;

    match res {
        Ok(response) => {
            let status = response.status();
            let body: serde_json::Value = response.json().await.unwrap_or(json!({}));
            
            if !status.is_success() {
                return format!("AI Error ({}): {}", status, body["error"]["message"]);
            }
            
            body["choices"][0]["message"]["content"].as_str().unwrap_or("No suggestions.").to_string()
        },
        Err(e) => format!("AI Connection Failed: {}", e)
    }
}
