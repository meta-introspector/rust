#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

# Quick OCI setup with known OCIDs

TENANCY_OCID="ocid1.tenancy.oc1..aaaaaaaapxfkcjaczqslvnbekbqq2eefxgwx7kqbakvddhzaaiym62vmt5la"
USER_OCID="ocid1.user.oc1..aaaaaaaas5losxb2h3z4gvjw7llmttn2a5pmjhleaz5bk4p54h7msvpd3o4q"

echo "🌩️ Oracle Cloud Setup for Solfunmeme"
echo "Tenancy: $TENANCY_OCID ✅"
echo "User: $USER_OCID ✅"
echo ""

echo "Still need:"
echo "1. API Key Fingerprint (from User Settings → API Keys)"
echo "2. Region (e.g., us-ashburn-1)"
echo "3. Private key .pem file path"
echo ""

# Check for existing SSH keys and calculate fingerprint
if [ -f "ssh-key-2025-11-19.key.pub" ]; then
    echo "🔑 Found SSH key, calculating fingerprint..."
    fingerprint=$(ssh-keygen -l -f "ssh-key-2025-11-19.key.pub" | awk '{print $2}' | tr ':' ':')
    echo "Calculated fingerprint: $fingerprint"
    echo ""
    echo "✅ Using calculated fingerprint: $fingerprint"
elif [ -f "$HOME/.ssh/id_rsa.pub" ]; then
    echo "🔑 Found SSH key, calculating fingerprint..."
    fingerprint=$(ssh-keygen -l -f "$HOME/.ssh/id_rsa.pub" | awk '{print $2}' | tr ':' ':')
    echo "Calculated fingerprint: $fingerprint"
    echo ""
    echo "✅ Using calculated fingerprint: $fingerprint"
fi

read -p "Region [us-ashburn-1]: " region
region=${region:-us-ashburn-1}

# Check if key is in ssh-agent
echo "🔐 Checking for SSH keys..."
if [ -f "$HOME/.ssh/ssh-key-2025-11-19.key" ]; then
    key_path="$HOME/.ssh/ssh-key-2025-11-19.key"
    echo "✅ Using default SSH key: $key_path"
elif [ -f "$HOME/.ssh/id_rsa" ]; then
    key_path="$HOME/.ssh/id_rsa"
    echo "✅ Using fallback SSH key: $key_path"
else
    read -p "Private key file path: " key_path
fi

# Quick validation
if [ ! -f "$key_path" ]; then
    echo "❌ Key file not found: $key_path"
    exit 1
fi

KEY_DIR="$HOME/.solfunmeme-keys"
mkdir -p "$KEY_DIR/secrets"

# Copy key securely
cp "$key_path" "$KEY_DIR/oci_private_key.pem"
chmod 600 "$KEY_DIR/oci_private_key.pem"

# Create Oracle config
cat > "$KEY_DIR/secrets/oracle-quick.yaml" << EOF
oracle:
  tenancy_ocid: "$TENANCY_OCID"
  user_ocid: "$USER_OCID"
  fingerprint: "$fingerprint"
  region: "$region"
  private_key_path: "$KEY_DIR/oci_private_key.pem"
  instance_config_ocid: "ocid1.instanceconfiguration.oc1.iad.aaaaaaaaabzhyygoc5clndba7tpuskdlkl2weivohvjkl65s5cvobuvywcrq"
EOF

echo ""
echo "✅ Oracle credentials configured!"
echo "📁 Config saved to: $KEY_DIR/secrets/oracle-quick.yaml"
echo ""
echo "🔧 Next steps:"
echo "  # Encrypt with SOPS:"
echo "  ./secrets-manager.sh init"
echo "  # Test connection:"
echo "  cd oci-client && cargo run"
