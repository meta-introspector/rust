use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use std::fs;
use rsa::{RsaPrivateKey, pkcs8::DecodePrivateKey, pkcs1v15::SigningKey, signature::{Signer, SignatureEncoding}};
use sha2::Sha256;
use base64::{Engine as _, engine::general_purpose};
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 Standalone Rust OCI signer - exact Python SDK replica...");
    
    // Load config
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
    
    // Load private key - use the converted PKCS#8 version
    let key_content = fs::read_to_string("/tmp/oracle_key_copy.pem")?;
    let private_key = RsaPrivateKey::from_pkcs8_pem(&key_content)?;
    
    // Create request details - EXACT same as Python SDK
    let method = "GET";
    let uri = format!("/20180917/stacks?compartmentId={}&lifecycleState=ACTIVE", tenancy_id);
    let host = format!("resourcemanager.{}.oraclecloud.com", region);
    let date = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
    
    // Create signing string - EXACT Python SDK format
    let signing_string = format!(
        "date: {}\n(request-target): {} {}\nhost: {}",
        date,
        method.to_lowercase(),
        uri,
        host
    );
    
    println!("📝 Signing string (Python SDK format):");
    println!("{:?}", signing_string);
    
    // Sign using PKCS#1 v1.5 with SHA256 (same as Python SDK)
    let signing_key = SigningKey::<Sha256>::new(private_key);
    let signature = signing_key.sign(signing_string.as_bytes());
    let encoded_signature = general_purpose::STANDARD.encode(signature.to_bytes());
    
    // Create authorization header - EXACT Python SDK format
    let auth_header = format!(
        "Signature algorithm=\"rsa-sha256\",headers=\"date (request-target) host\",keyId=\"{}/{}/{}\",signature=\"{}\",version=\"1\"",
        tenancy_id, user_id, fingerprint, encoded_signature
    );
    
    println!("📋 Authorization header:");
    println!("{}", auth_header);
    
    // Make the request
    let url = format!("https://{}{}", host, uri);
    let client = Client::new();
    let response = client
        .get(&url)
        .header("date", &date)
        .header("host", &host)
        .header("authorization", &auth_header)
        .header("accept", "application/json")
        .header("content-type", "application/json")
        .send()
        .await?;
    
    println!("🔍 Response status: {}", response.status());
    
    if response.status().is_success() {
        let stacks: Value = response.json().await?;
        if let Some(array) = stacks.as_array() {
            println!("✅ SUCCESS! Found {} Oracle Cloud stacks:", array.len());
            for (i, stack) in array.iter().enumerate() {
                if let Some(name) = stack.get("displayName").and_then(|v| v.as_str()) {
                    if let Some(id) = stack.get("id").and_then(|v| v.as_str()) {
                        println!("  {}. {} - {}", i+1, name, id);
                    }
                }
            }
        }
    } else {
        println!("❌ Failed: {}", response.text().await?);
    }
    
    Ok(())
}
