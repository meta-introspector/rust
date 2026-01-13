use anyhow::Result;
use oci_api::Oci;
use std::fs;

#[derive(Debug, serde::Deserialize)]
struct OracleConfig {
    oracle: OracleCredentials,
}

#[derive(Debug, serde::Deserialize)]
struct OracleCredentials {
    tenancy_ocid: String,
    user_ocid: String,
    fingerprint: String,
    region: String,
    private_key_path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 Fetching Oracle Cloud deployments with oci-api crate...");
    
    // Load config
    let config_path = format!("{}/.solfunmeme-keys/secrets/oracle-quick.yaml", 
                             std::env::var("HOME").unwrap());
    let config_str = fs::read_to_string(&config_path)?;
    let config: OracleConfig = serde_yaml::from_str(&config_str)?;
    
    // Create OCI config string for the crate
    let oci_config_content = format!(
        "[DEFAULT]\nuser={}\ntenancy={}\nregion={}\nfingerprint={}\nkey_file={}",
        config.oracle.user_ocid,
        config.oracle.tenancy_ocid,
        config.oracle.region,
        config.oracle.fingerprint,
        config.oracle.private_key_path
    );
    
    // Set environment variable for oci-api crate
    std::env::set_var("OCI_CONFIG", oci_config_content);
    
    println!("📋 Querying region: {}", config.oracle.region);
    
    // Create OCI client
    let oci = Oci::from_env()?;
    
    println!("✅ OCI client created successfully!");
    
    // Test basic identity first
    println!("🔍 Testing basic identity API...");
    
    let identity_url = format!(
        "https://identity.{}.oraclecloud.com/20160918/tenancies/{}",
        config.oracle.region,
        config.oracle.tenancy_ocid
    );
    
    let response = oci.client()
        .get(&identity_url)
        .send()
        .await;
    
    // Debug: Show what headers were sent
    println!("🔍 Debug - Request URL: {}", identity_url);
    
    match response {
        Ok(resp) => {
            println!("🔍 Debug - Response status: {}", resp.status());
            if resp.status().is_success() {
                println!("✅ Identity API working!");
                let text = resp.text().await?;
                println!("Tenancy: {}", text);
            } else {
                println!("❌ Identity API failed: {} - {}", 
                        resp.status(), 
                        resp.text().await.unwrap_or_default());
                return Ok(());
            }
        }
        Err(e) => {
            println!("❌ Request error: {}", e);
            return Ok(());
        }
    }
    
    // Now try Resource Manager stacks
    println!("📚 Fetching Resource Manager Stacks...");
    
    let stacks_url = format!(
        "https://resourcemanager.{}.oraclecloud.com/20180917/stacks?compartmentId={}&lifecycleState=ACTIVE",
        config.oracle.region,
        config.oracle.tenancy_ocid
    );
    
    println!("🔗 API URL: {}", stacks_url);
    
    let response = oci.client()
        .get(&stacks_url)
        .send()
        .await?;
    
    if response.status().is_success() {
        let text = response.text().await?;
        println!("✅ Active stacks found:");
        
        // Parse and display nicely
        if let Ok(stacks) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(array) = stacks.as_array() {
                println!("📊 Found {} active stacks:", array.len());
                for (i, stack) in array.iter().enumerate() {
                    if let (Some(name), Some(id), Some(state)) = (
                        stack.get("displayName").and_then(|v| v.as_str()),
                        stack.get("id").and_then(|v| v.as_str()),
                        stack.get("lifecycleState").and_then(|v| v.as_str())
                    ) {
                        println!("  {}. {} ({}) - {}", i+1, name, state, id);
                    }
                }
            }
        } else {
            println!("Raw response: {}", text);
        }
    } else {
        println!("❌ Failed to fetch stacks: {} - {}", 
                response.status(), 
                response.text().await.unwrap_or_default());
    }
    
    Ok(())
}
