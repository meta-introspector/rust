use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use base64::{Engine as _, engine::general_purpose};
use sha2::{Sha256, Digest};
use rsa::{RsaPrivateKey, pkcs1::DecodeRsaPrivateKey, pkcs8::DecodePrivateKey, signature::{RandomizedSigner, SignatureEncoding}};
use rsa::pkcs1v15::{SigningKey, Signature};
use chrono::Utc;

#[derive(Debug, Deserialize)]
struct Instance {
    id: String,
    #[serde(rename = "display-name")]
    display_name: String,
    #[serde(rename = "lifecycle-state")]
    lifecycle_state: String,
    #[serde(rename = "time-created")]
    time_created: String,
}

#[derive(Debug, Deserialize)]
struct ListInstancesResponse {
    data: Vec<Instance>,
}

#[derive(Debug, Deserialize)]
struct OracleConfig {
    oracle: OracleCredentials,
}

#[derive(Debug, Deserialize)]
struct OracleCredentials {
    tenancy_ocid: String,
    user_ocid: String,
    fingerprint: String,
    region: String,
    private_key_path: String,
}

fn create_oci_signature(
    method: &str,
    uri: &str,
    headers: &std::collections::HashMap<String, String>,
    private_key: &RsaPrivateKey,
) -> Result<String> {
    let mut signing_string = String::new();
    
    // Correct OCI header order: "date (request-target) host"
    if let Some(date) = headers.get("date") {
        signing_string.push_str(&format!("date: {}\n", date));
    }
    
    signing_string.push_str(&format!("(request-target): {} {}\n", method.to_lowercase(), uri));
    
    if let Some(host) = headers.get("host") {
        signing_string.push_str(&format!("host: {}", host));
    }
    
    let mut hasher = Sha256::new();
    hasher.update(signing_string.as_bytes());
    let hash = hasher.finalize();
    
    let signing_key = SigningKey::<Sha256>::new(private_key.clone());
    let signature = signing_key.sign_with_rng(&mut rand::thread_rng(), &hash);
    
    Ok(general_purpose::STANDARD.encode(signature.to_bytes()))
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 Fetching Oracle Cloud deployments...");
    
    // Load config
    let config_path = format!("{}/.solfunmeme-keys/secrets/oracle-quick.yaml", 
                             std::env::var("HOME").unwrap());
    let config_str = fs::read_to_string(&config_path)?;
    let config: OracleConfig = serde_yaml::from_str(&config_str)?;
    
    // Load private key
    let key_content = fs::read_to_string(&config.oracle.private_key_path)?;
    let private_key = if key_content.contains("BEGIN RSA PRIVATE KEY") {
        RsaPrivateKey::from_pkcs1_pem(&key_content)?
    } else {
        RsaPrivateKey::from_pkcs8_pem(&key_content)?
    };
    
    // Create OCI API client
    let client = Client::new();
    let base_url = format!("https://iaas.{}.oraclecloud.com", config.oracle.region);
    let uri = format!("/20160918/instances?compartmentId={}", config.oracle.tenancy_ocid);
    let url = format!("{}{}", base_url, uri);
    
    // Create headers
    let date = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
    let host = format!("iaas.{}.oraclecloud.com", config.oracle.region);
    
    let mut headers = std::collections::HashMap::new();
    headers.insert("date".to_string(), date.clone());
    headers.insert("host".to_string(), host.clone());
    
    // Create signature
    let signature = create_oci_signature("GET", &uri, &headers, &private_key)?;
    
    let auth_header = format!(
        r#"Signature algorithm="rsa-sha256",headers="date (request-target) host",keyId="{}/{}/{}",signature="{}",version="1""#,
        config.oracle.tenancy_ocid,
        config.oracle.user_ocid,
        config.oracle.fingerprint,
        signature
    );
    
    println!("📋 Querying instances in region: {}", config.oracle.region);
    println!("🔍 Debug - Auth header: {}", auth_header);
    println!("🔍 Debug - Date: {}", date);
    println!("🔍 Debug - Host: {}", host);
    
    // Make request
    let response = client
        .get(&url)
        .header("Date", date)
        .header("Host", host)
        .header("Authorization", auth_header)
        .send()
        .await?;
    
    if response.status().is_success() {
        let instances: ListInstancesResponse = response.json().await?;
        println!("✅ Found {} instances:", instances.data.len());
        for instance in instances.data {
            println!("  🖥️  {} ({}) - {}", 
                    instance.display_name, 
                    instance.lifecycle_state,
                    instance.id);
        }
    } else {
        println!("❌ Request failed: {} - {}", response.status(), response.text().await?);
    }
    
    Ok(())
}
