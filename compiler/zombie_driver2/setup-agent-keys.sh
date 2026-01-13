#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

set -euo pipefail

# Solfunmeme Agent Key Management System
# Generates PIN, GPG key, SSH key, and sets up SOPS for secret management

AGENT_NAME="solfunmeme"
KEY_DIR="$HOME/.${AGENT_NAME}-keys"
PIN_FILE="$KEY_DIR/agent.pin"

echo "🔐 Solfunmeme Agent Key Generation System"
echo "========================================"

# Create key directory
mkdir -p "$KEY_DIR"
chmod 700 "$KEY_DIR"

# Generate PIN
if [ ! -f "$PIN_FILE" ]; then
    echo "🎲 Generating secure PIN..."
    PIN=$(openssl rand -base64 32 | tr -d "=+/" | cut -c1-16)
    echo "$PIN" > "$PIN_FILE"
    chmod 600 "$PIN_FILE"
    echo "📌 PIN generated and saved securely"
    echo ""
else
    PIN=$(cat "$PIN_FILE")
    echo "📌 Using existing PIN from: $PIN_FILE"
    echo ""
fi

# Generate GPG key
echo "🔒 Generating GPG key for $AGENT_NAME..."
GPG_BATCH_FILE="$KEY_DIR/gpg_batch"
cat > "$GPG_BATCH_FILE" << EOF
%echo Generating GPG key for $AGENT_NAME
Key-Type: RSA
Key-Length: 4096
Subkey-Type: RSA
Subkey-Length: 4096
Name-Real: Solfunmeme Agent
Name-Email: agent@solfunmeme.cloud
Expire-Date: 2y
Passphrase: $PIN
%commit
%echo GPG key generation complete
EOF

gpg --batch --generate-key "$GPG_BATCH_FILE"
rm "$GPG_BATCH_FILE"

# Get GPG key ID
GPG_KEY_ID=$(gpg --list-secret-keys --keyid-format LONG "agent@solfunmeme.cloud" | grep sec | awk '{print $2}' | cut -d'/' -f2)
echo "🔑 GPG Key ID: $GPG_KEY_ID"

# Export GPG keys
gpg --armor --export "$GPG_KEY_ID" > "$KEY_DIR/gpg_public.asc"
gpg --armor --export-secret-keys "$GPG_KEY_ID" > "$KEY_DIR/gpg_private.asc"
echo "📁 GPG keys exported to $KEY_DIR/"

# Generate SSH key
echo "🔐 Generating SSH key..."
SSH_KEY_FILE="$KEY_DIR/ssh_key"
ssh-keygen -t ed25519 -f "$SSH_KEY_FILE" -N "$PIN" -C "solfunmeme-agent@$(hostname)"
echo "🔑 SSH key generated: $SSH_KEY_FILE"

# Install SOPS
echo "📦 Installing SOPS..."
if ! command -v sops &> /dev/null; then
    SOPS_VERSION="3.8.1"
    curl -LO "https://github.com/mozilla/sops/releases/download/v${SOPS_VERSION}/sops-v${SOPS_VERSION}.linux.amd64"
    chmod +x "sops-v${SOPS_VERSION}.linux.amd64"
    sudo mv "sops-v${SOPS_VERSION}.linux.amd64" /usr/local/bin/sops
    echo "✅ SOPS installed"
else
    echo "✅ SOPS already installed"
fi

# Create SOPS config
echo "⚙️ Creating SOPS configuration..."
SOPS_CONFIG="$KEY_DIR/.sops.yaml"
cat > "$SOPS_CONFIG" << EOF
creation_rules:
  - path_regex: \.enc\.(yaml|yml|json)$
    pgp: $GPG_KEY_ID
  - path_regex: secrets/.*
    pgp: $GPG_KEY_ID
EOF

# Create sample encrypted secrets file
echo "🔒 Creating sample secrets file..."
SECRETS_DIR="$KEY_DIR/secrets"
mkdir -p "$SECRETS_DIR"

cat > "$SECRETS_DIR/oracle-credentials.yaml" << EOF
# Oracle Cloud credentials for solfunmeme
tenancy_ocid: "ocid1.tenancy.oc1..your-tenancy-here"
user_ocid: "ocid1.user.oc1..your-user-here"
fingerprint: "your-fingerprint-here"
region: "us-ashburn-1"
private_key_path: "$KEY_DIR/oci_private_key.pem"
EOF

# Encrypt the secrets file
SOPS_CONFIG_FILE="$SOPS_CONFIG" sops -e -i "$SECRETS_DIR/oracle-credentials.yaml"

echo ""
echo "✅ Solfunmeme Agent Key Setup Complete!"
echo "======================================="
echo "📌 PIN: $(cat $PIN_FILE)"
echo "🔑 GPG Key ID: $GPG_KEY_ID"
echo "🔐 SSH Key: $SSH_KEY_FILE"
echo "📁 All keys stored in: $KEY_DIR"
echo ""
echo "🔧 Usage:"
echo "  # Decrypt secrets:"
echo "  SOPS_CONFIG_FILE=$SOPS_CONFIG sops -d $SECRETS_DIR/oracle-credentials.yaml"
echo ""
echo "  # Edit encrypted secrets:"
echo "  SOPS_CONFIG_FILE=$SOPS_CONFIG sops $SECRETS_DIR/oracle-credentials.yaml"
echo ""
echo "  # Use SSH key:"
echo "  ssh-add $SSH_KEY_FILE  # (will prompt for PIN)"
echo ""
echo "🔒 Keep your PIN safe - it unlocks everything!"
