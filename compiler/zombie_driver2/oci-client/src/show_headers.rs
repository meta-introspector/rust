use anyhow::Result;
use reqwest::Client;
use std::fs;
use rsa::{RsaPrivateKey, pkcs8::DecodePrivateKey, pkcs1::DecodeRsaPrivateKey, pkcs1v15::SigningKey, signature::{Signer, SignatureEncoding}};
use sha2::Sha256;
use base64::{Engine as _, engine::general_purpose};
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 Manual OCI request with visible headers...");
    
    // Load config manually
    let config_content = fs::read_to_string("/home/mdupont/.solfunmeme-keys/oci_config")?;
    let mut user_id = String::new();
    let mut tenancy_id = String::new();
    let mut fingerprint = String::new();
    let mut region = String::new();
    let mut key_file = String::new();
    
    for line in config_content.lines() {
        if line.starts_with("user=") {
            user_id = line.strip_prefix("user=").unwrap().to_string();
        } else if line.starts_with("tenancy=") {
            tenancy_id = line.strip_prefix("tenancy=").unwrap().to_string();
        } else if line.starts_with("fingerprint=") {
            fingerprint = line.strip_prefix("fingerprint=").unwrap().to_string();
        } else if line.starts_with("region=") {
            region = line.strip_prefix("region=").unwrap().to_string();
        } else if line.starts_with("key_file=") {
            key_file = line.strip_prefix("key_file=").unwrap().to_string();
        }
    }
    
    println!("📋 Config loaded:");
    println!("User: {}", user_id);
    println!("Tenancy: {}", tenancy_id);
    println!("Fingerprint: {}", fingerprint);
    println!("Region: {}", region);
    println!("Key file: {}", key_file);
    println!();
    
    // Show what the signing string should look like
    let method = "GET";
    let path = "/20160918/tenancies/ocid1.tenancy.oc1..aaaaaaaapxfkcjaczqslvnbekbqq2eefxgwx7kqbakvddhzaaiym62vmt5la";
    let host = format!("identity.{}.oraclecloud.com", region);
    let date = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
    
    // Create signing string (OCI format)
    let signing_string = format!(
        "date: {}\n(request-target): {} {}\nhost: {}",
        date,
        method.to_lowercase(),
        path,
        host
    );
    
    println!("📝 Signing string that should be used:");
    println!("\"{}\"", signing_string);
    println!();
    
    let key_id = format!("{}/{}/{}", tenancy_id, user_id, fingerprint);
    
    println!("📋 Headers that should be sent:");
    println!("Date: {}", date);
    println!("Host: {}", host);
    println!("Authorization: Signature version=\"1\",headers=\"date (request-target) host\",keyId=\"{}\",algorithm=\"rsa-sha256\",signature=\"[SIGNATURE]\"", key_id);
    println!();
    
    println!("✅ This shows the exact format OCI expects");
    
    Ok(())
}
