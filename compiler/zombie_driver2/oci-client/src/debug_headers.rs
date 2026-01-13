use anyhow::Result;
use reqwest::Client;
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 Testing manual request with debug headers...");
    
    // Load config
    let config_content = fs::read_to_string("/home/mdupont/.solfunmeme-keys/oci_config")?;
    println!("📋 Config loaded");
    
    // Simple test request to identity API
    let url = "https://identity.us-ashburn-1.oraclecloud.com/20160918/tenancies/ocid1.tenancy.oc1..aaaaaaaapxfkcjaczqslvnbekbqq2eefxgwx7kqbakvddhzaaiym62vmt5la";
    
    let client = Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "test-client")
        .send()
        .await?;
    
    println!("🔍 Debug - URL: {}", url);
    println!("🔍 Debug - Status: {}", response.status());
    println!("🔍 Debug - Response: {}", response.text().await?);
    
    Ok(())
}
