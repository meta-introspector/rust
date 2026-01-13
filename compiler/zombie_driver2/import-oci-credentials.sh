#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

set -euo pipefail

# Oracle OCI Credential Import Helper
# Guides you through collecting the required OCIDs and keys

echo "🌩️ Oracle Cloud OCI Credential Import"
echo "====================================="
echo ""

echo "📋 Required Information:"
echo ""
echo "1. TENANCY OCID"
echo "   Location: Oracle Cloud Console → Profile → Tenancy"
echo "   Format: ocid1.tenancy.oc1..aaaaaaaaXXXXXXXX"
echo ""

echo "2. USER OCID" 
echo "   Location: Oracle Cloud Console → Profile → User Settings"
echo "   Format: ocid1.user.oc1..aaaaaaaaXXXXXXXX"
echo ""

echo "3. COMPARTMENT OCID (optional, defaults to tenancy)"
echo "   Location: Oracle Cloud Console → Identity → Compartments"
echo "   Format: ocid1.compartment.oc1..aaaaaaaaXXXXXXXX"
echo ""

echo "4. API KEY FINGERPRINT"
echo "   Location: User Settings → API Keys → View fingerprint"
echo "   Format: aa:bb:cc:dd:ee:ff:00:11:22:33:44:55:66:77:88:99"
echo ""

echo "5. REGION"
echo "   Examples: us-ashburn-1, us-phoenix-1, eu-frankfurt-1"
echo ""

echo "6. PRIVATE KEY (.pem file)"
echo "   The private key file you downloaded when creating the API key"
echo ""

read -p "Do you have all this information? (y/n): " ready

if [ "$ready" != "y" ]; then
    echo ""
    echo "📖 How to get these:"
    echo ""
    echo "1. Go to Oracle Cloud Console"
    echo "2. Click your profile → User Settings"
    echo "3. Copy your User OCID"
    echo "4. Click Tenancy → Copy Tenancy OCID"
    echo "5. Go to API Keys → Add API Key"
    echo "6. Download the private key (.pem file)"
    echo "7. Copy the fingerprint shown"
    echo ""
    echo "Run this script again when ready!"
    exit 0
fi

echo ""
echo "🔧 Collecting OCI Information..."
echo ""

read -p "Tenancy OCID: " tenancy_ocid
read -p "User OCID: " user_ocid
read -p "Compartment OCID (or press Enter for tenancy): " compartment_ocid
read -p "API Key Fingerprint: " fingerprint
read -p "Region (e.g., us-ashburn-1): " region
read -p "Path to private key .pem file: " private_key_path

# Default compartment to tenancy if not provided
if [ -z "$compartment_ocid" ]; then
    compartment_ocid="$tenancy_ocid"
fi

# Validate the private key file exists
if [ ! -f "$private_key_path" ]; then
    echo "❌ Private key file not found: $private_key_path"
    exit 1
fi

# Copy private key to our key directory
KEY_DIR="$HOME/.solfunmeme-keys"
cp "$private_key_path" "$KEY_DIR/oci_private_key.pem"
chmod 600 "$KEY_DIR/oci_private_key.pem"

echo ""
echo "✅ Information collected! Updating secrets..."

# Update the SOPS encrypted services file
SECRETS_DIR="$KEY_DIR/secrets"
SOPS_CONFIG="$KEY_DIR/.sops.yaml"

# Create temporary file with Oracle credentials
temp_file=$(mktemp)
cat > "$temp_file" << EOF
oracle:
  tenancy_ocid: "$tenancy_ocid"
  user_ocid: "$user_ocid"
  compartment_ocid: "$compartment_ocid"
  fingerprint: "$fingerprint"
  region: "$region"
  private_key_path: "$KEY_DIR/oci_private_key.pem"
  
  # Instance configuration OCID (your solfunmeme config)
  instance_config_ocid: "ocid1.instanceconfiguration.oc1.iad.aaaaaaaaabzhyygoc5clndba7tpuskdlkl2weivohvjkl65s5cvobuvywcrq"
EOF

# If services.yaml exists, merge with existing content
if [ -f "$SECRETS_DIR/services.yaml" ]; then
    # Decrypt existing, merge oracle section, re-encrypt
    existing_content=$(SOPS_CONFIG_FILE="$SOPS_CONFIG" sops -d "$SECRETS_DIR/services.yaml")
    echo "$existing_content" | yq eval-all 'select(fileIndex == 0) * select(fileIndex == 1)' - "$temp_file" > "$temp_file.merged"
    mv "$temp_file.merged" "$temp_file"
fi

# Encrypt and save
cp "$temp_file" "$SECRETS_DIR/services.yaml"
SOPS_CONFIG_FILE="$SOPS_CONFIG" sops -e -i "$SECRETS_DIR/services.yaml"
rm "$temp_file"

echo ""
echo "🎉 Oracle OCI credentials imported successfully!"
echo ""
echo "📋 Summary:"
echo "  Tenancy: $tenancy_ocid"
echo "  User: $user_ocid"
echo "  Region: $region"
echo "  Private key: $KEY_DIR/oci_private_key.pem"
echo ""
echo "🔧 Test your credentials:"
echo "  ./secrets-manager.sh sops get oracle"
echo "  cd oci-client && cargo run"
echo ""
echo "🚀 Ready to deploy to Oracle Cloud!"
