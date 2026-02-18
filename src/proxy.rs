use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;

pub async fn start_proxy(listen_port: u16, target_addr: String) -> Result<(), Box<dyn Error>> {
    println!("\n[🎭] Ghost Proxy Started!");
    println!("    🎧 Listening on: 127.0.0.1:{}", listen_port);
    println!("    🎯 Forwarding to: {}\n", target_addr);
    println!("    (Set your browser proxy to 127.0.0.1:{} to capture traffic)\n", listen_port);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", listen_port)).await?;

    loop {
        // අලුත් කෙනෙක් කනෙක්ට් වුණාම
        let (mut client_socket, addr) = listener.accept().await?;
        println!("    [+] Connection intercepted from: {}", addr);
        let target = target_addr.clone();

        tokio::spawn(async move {
            // ටාගට් සර්වර් එකට කනෙක්ට් වෙනවා
            match TcpStream::connect(&target).await {
                Ok(mut server_socket) => {
                    // Bidirectional Copy
                    let (mut client_read, mut client_write) = client_socket.split();
                    let (mut server_read, mut server_write) = server_socket.split();

                    // Client -> Server 
                    let client_to_server = async {
                        let mut buffer = [0; 4096];
                        loop {
                            match client_read.read(&mut buffer).await {
                                Ok(0) => break, // Connection Closed
                                Ok(n) => {
                                    // 🕵️‍♂️ SNIFFING HAPPENS HERE!
                                    let data = String::from_utf8_lossy(&buffer[..n]);
                                    println!("\n    📤 [REQUEST] Captured Data:\n    ----------------------------------------\n    {}\n    ----------------------------------------", data.trim());
                                    
                                    
                                    if server_write.write_all(&buffer[..n]).await.is_err() { break; }
                                }
                                Err(_) => break,
                            }
                        }
                    };

                    // Server -> Client එන දත්ත 
                    let server_to_client = async {
                        let mut buffer = [0; 4096];
                        loop {
                            match server_read.read(&mut buffer).await {
                                Ok(0) => break,
                                Ok(n) => {
                                    if client_write.write_all(&buffer[..n]).await.is_err() { break; }
                                }
                                Err(_) => break,
                            }
                        }
                    };

                    
                    tokio::join!(client_to_server, server_to_client);
                }
                Err(e) => println!("    ❌ Failed to connect to target: {}", e),
            }
        });
    }
}