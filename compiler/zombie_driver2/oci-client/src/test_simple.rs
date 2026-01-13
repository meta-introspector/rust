use anyhow::Result;
use reqwest::Client;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 Fetching stacks using Python SDK approach...");
    
    // Use the exact same URL that worked in Python
    let url = "https://resourcemanager.us-ashburn-1.oraclecloud.com/20180917/stacks?compartmentId=ocid1.tenancy.oc1..aaaaaaaapxfkcjaczqslvnbekbqq2eefxgwx7kqbakvddhzaaiym62vmt5la&lifecycleState=ACTIVE";
    
    let client = Client::new();
    let response = client
        .get(url)
        .header("accept", "application/json")
        .header("content-type", "application/json")
        .send()
        .await?;
    
    println!("🔍 Response status: {}", response.status());
    
    if response.status().is_success() {
        let stacks: Value = response.json().await?;
        if let Some(array) = stacks.as_array() {
            println!("✅ Found {} stacks without authentication:", array.len());
            for (i, stack) in array.iter().enumerate() {
                if let Some(name) = stack.get("displayName").and_then(|v| v.as_str()) {
                    println!("  {}. {}", i+1, name);
                }
            }
        }
    } else {
        println!("❌ Failed: {}", response.text().await?);
    }
    
    Ok(())
}
