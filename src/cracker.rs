use md5;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

pub fn crack_hash(target_hash: &str, wordlist_path: &str) {
    println!("\n[🔨] Starting Hash Cracker...");
    println!("    🎯 Target: {}", target_hash);
    println!("    📂 Wordlist: {}\n", wordlist_path);

    let file = match File::open(wordlist_path) {
        Ok(f) => f,
        Err(_) => {
            println!("❌ Error: Wordlist file not found!");
            return;
        }
    };

    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        if let Ok(password) = line {
            let clean_pass = password.trim(); // අනවශ්‍ය හිස්තැන් අයින් කරනවා

            // 1. MD5 Check
            let md5_digest = md5::compute(clean_pass);
            let md5_hash = format!("{:x}", md5_digest);

            // 2. SHA256 Check
            let mut sha256_hasher = Sha256::new();
            sha256_hasher.update(clean_pass);
            let sha256_hash = format!("{:x}", sha256_hasher.finalize());

            // අපි හොයන Hash එක සමානද බලනවා
            if md5_hash == target_hash || sha256_hash == target_hash {
                println!("-----------------------------------------");
                println!("🔥 CRACKED SUCCESSFULLY!");
                println!("🔑 Password Found: {}", clean_pass);
                println!("📜 Hash Type: {}", if md5_hash == target_hash { "MD5" } else { "SHA-256" });
                println!("-----------------------------------------");
                process::exit(0); // පාස්වර්ඩ් එක හම්බුනාම වැඩේ නවත්වනවා
            }

            // හැම 1000 කට සැරයක් Progress එක පෙන්වනවා
            if index % 1000 == 0 {
                print!("\r    ⏳ Checked {} passwords...", index);
            }
        }
    }

    println!("\n❌ Password not found in the wordlist.");
}