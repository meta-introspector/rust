#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

set -euo pipefail

# Solfunmeme Agent Key Manager - Daily operations script

AGENT_NAME="solfunmeme"
KEY_DIR="$HOME/.${AGENT_NAME}-keys"
PIN_FILE="$KEY_DIR/agent.pin"
SOPS_CONFIG="$KEY_DIR/.sops.yaml"

if [ ! -f "$PIN_FILE" ]; then
    echo "❌ No keys found. Run ./setup-agent-keys.sh first"
    exit 1
fi

PIN=$(cat "$PIN_FILE")

case "${1:-help}" in
    "show-pin")
        echo "🔑 Agent PIN: $PIN"
        ;;
    "decrypt-secrets")
        echo "🔓 Decrypting Oracle credentials..."
        SOPS_CONFIG_FILE="$SOPS_CONFIG" sops -d "$KEY_DIR/secrets/oracle-credentials.yaml"
        ;;
    "edit-secrets")
        echo "✏️ Editing encrypted secrets..."
        SOPS_CONFIG_FILE="$SOPS_CONFIG" sops "$KEY_DIR/secrets/oracle-credentials.yaml"
        ;;
    "load-ssh")
        echo "🔐 Loading SSH key..."
        ssh-add "$KEY_DIR/ssh_key"
        ;;
    "gpg-info")
        echo "🔒 GPG key information:"
        gpg --list-secret-keys "agent@solfunmeme.cloud"
        ;;
    "backup")
        echo "💾 Creating encrypted backup..."
        tar -czf "/tmp/solfunmeme-keys-$(date +%Y%m%d).tar.gz" -C "$HOME" ".${AGENT_NAME}-keys"
        gpg --cipher-algo AES256 --compress-algo 1 --symmetric --output "/tmp/solfunmeme-keys-$(date +%Y%m%d).tar.gz.gpg" "/tmp/solfunmeme-keys-$(date +%Y%m%d).tar.gz"
        rm "/tmp/solfunmeme-keys-$(date +%Y%m%d).tar.gz"
        echo "✅ Backup created: /tmp/solfunmeme-keys-$(date +%Y%m%d).tar.gz.gpg"
        ;;
    "status")
        echo "📊 Solfunmeme Agent Key Status:"
        echo "  PIN file: $([ -f "$PIN_FILE" ] && echo "✅" || echo "❌")"
        echo "  SSH key: $([ -f "$KEY_DIR/ssh_key" ] && echo "✅" || echo "❌")"
        echo "  GPG key: $(gpg --list-secret-keys "agent@solfunmeme.cloud" &>/dev/null && echo "✅" || echo "❌")"
        echo "  SOPS config: $([ -f "$SOPS_CONFIG" ] && echo "✅" || echo "❌")"
        echo "  Secrets: $([ -f "$KEY_DIR/secrets/oracle-credentials.yaml" ] && echo "✅" || echo "❌")"
        ;;
    *)
        echo "🔐 Solfunmeme Agent Key Manager"
        echo "Usage: $0 [command]"
        echo ""
        echo "Commands:"
        echo "  show-pin        Show the agent PIN"
        echo "  decrypt-secrets Decrypt and show Oracle credentials"
        echo "  edit-secrets    Edit encrypted secrets file"
        echo "  load-ssh        Load SSH key into agent"
        echo "  gpg-info        Show GPG key information"
        echo "  backup          Create encrypted backup of all keys"
        echo "  status          Show key status"
        ;;
esac
