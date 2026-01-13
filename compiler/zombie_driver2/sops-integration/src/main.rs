use serde::{Deserialize, Serialize};
use std::process::Command;
use anyhow::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct OracleCredentials {
    pub tenancy_ocid: String,
    pub user_ocid: String,
    pub fingerprint: String,
    pub region: String,
    pub private_key_path: String,
}

pub struct SopsManager {
    config_path: String,
    secrets_dir: String,
}

impl SopsManager {
    pub fn new() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let key_dir = format!("{}/.solfunmeme-keys", home);
        
        Self {
            config_path: format!("{}/.sops.yaml", key_dir),
            secrets_dir: format!("{}/secrets", key_dir),
        }
    }

    pub fn decrypt_oracle_credentials(&self) -> Result<OracleCredentials> {
        let output = Command::new("sops")
            .env("SOPS_CONFIG_FILE", &self.config_path)
            .args(&["-d", &format!("{}/oracle-credentials.yaml", self.secrets_dir)])
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("SOPS decryption failed: {}", 
                String::from_utf8_lossy(&output.stderr)));
        }

        let yaml_content = String::from_utf8(output.stdout)?;
        let credentials: OracleCredentials = serde_yaml::from_str(&yaml_content)?;
        Ok(credentials)
    }

    pub fn get_pin(&self) -> Result<String> {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let pin_file = format!("{}/.solfunmeme-keys/agent.pin", home);
        let pin = std::fs::read_to_string(pin_file)?.trim().to_string();
        Ok(pin)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔐 Solfunmeme SOPS Integration Test");
    
    let sops = SopsManager::new();
    
    match sops.get_pin() {
        Ok(pin) => println!("✅ PIN loaded: {}****", &pin[..4]),
        Err(e) => println!("❌ PIN error: {}", e),
    }
    
    match sops.decrypt_oracle_credentials() {
        Ok(creds) => {
            println!("✅ Oracle credentials decrypted:");
            println!("  Region: {}", creds.region);
            println!("  Tenancy: {}****", &creds.tenancy_ocid[..20]);
        }
        Err(e) => println!("❌ Credentials error: {}", e),
    }
    
    Ok(())
}
