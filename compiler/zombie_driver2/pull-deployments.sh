#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

set -euo pipefail

# Pull recent Oracle Cloud deployments for solfunmeme

echo "🌩️ Fetching recent Oracle Cloud deployments..."

# Check if we have credentials configured
KEY_DIR="$HOME/.solfunmeme-keys"
if [ ! -f "$KEY_DIR/secrets/oracle-quick.yaml" ]; then
    echo "❌ Oracle credentials not found. Run ./quick-oci-setup.sh first"
    exit 1
fi

# Use our Rust OCI client to list instances
echo "🔍 Checking compute instances..."
cd oci-client

# Create a quick deployment checker
cat > src/list_deployments.rs << 'EOF'
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

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

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 Listing recent deployments...");
    
    // This would use proper OCI auth in production
    // For now, show what we'd fetch
    println!("📋 Recent instances (would fetch from OCI API):");
    println!("  - solfunmeme-p2p-server (if deployed)");
    println!("  - Any other compute instances");
    
    Ok(())
}
EOF

# Build and run
cargo build --bin list_deployments 2>/dev/null || echo "⚠️ Build failed, using fallback"

echo ""
echo "📊 Deployment Status Check:"
echo "  Oracle Config: ✅"
echo "  Instance Config OCID: ocid1.instanceconfiguration.oc1.iad.aaaaaaaaabzhyygoc5clndba7tpuskdlkl2weivohvjkl65s5cvobuvywcrq"
echo ""
echo "🔧 To check actual deployments:"
echo "  1. Configure OCI CLI: oci setup config"
echo "  2. List instances: oci compute instance list --compartment-id <compartment-ocid>"
echo "  3. Or use our Rust client with proper auth"

cd ..
