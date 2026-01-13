#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

set -euo pipefail

# Unified Secrets Manager for Solfunmeme Agent
# Supports SOPS, .netrc, and multiple service credential formats

KEY_DIR="$HOME/.solfunmeme-keys"
SECRETS_DIR="$KEY_DIR/secrets"
SOPS_CONFIG="$KEY_DIR/.sops.yaml"

mkdir -p "$SECRETS_DIR"

case "${1:-help}" in
    "init")
        echo "🔐 Initializing unified secrets..."
        
        # Create .netrc template
        cat > "$SECRETS_DIR/netrc-template" << 'EOF'
# Network credentials for various services
machine github.com
    login your-username
    password ghp_your_token

machine api.github.com
    login your-username
    password ghp_your_token

machine registry-1.docker.io
    login your-docker-username
    password your-docker-token

machine gitlab.com
    login your-username
    password glpat_your_token
EOF

        # Create services SOPS file
        cat > "$SECRETS_DIR/services.yaml" << 'EOF'
# Multi-service credentials
oracle:
  tenancy_ocid: "ocid1.tenancy.oc1..your-tenancy"
  user_ocid: "ocid1.user.oc1..your-user"
  fingerprint: "your-fingerprint"
  region: "us-ashburn-1"

github:
  token: "ghp_your_personal_access_token"
  username: "your-username"

aws:
  access_key_id: "AKIA..."
  secret_access_key: "your-secret"
  region: "us-east-1"

docker:
  username: "your-docker-username"
  password: "your-docker-password"

gitlab:
  token: "glpat_your_gitlab_token"
  username: "your-username"

npm:
  token: "npm_your_token"

pypi:
  username: "__token__"
  password: "pypi-your-token"
EOF

        # Encrypt with SOPS
        SOPS_CONFIG_FILE="$SOPS_CONFIG" sops -e -i "$SECRETS_DIR/services.yaml"
        
        echo "✅ Templates created:"
        echo "  $SECRETS_DIR/netrc-template"
        echo "  $SECRETS_DIR/services.yaml (encrypted)"
        ;;
        
    "netrc")
        echo "🌐 Managing .netrc credentials..."
        if [ ! -f "$SECRETS_DIR/netrc-template" ]; then
            echo "❌ Run '$0 init' first"
            exit 1
        fi
        
        case "${2:-show}" in
            "edit")
                ${EDITOR:-nano} "$SECRETS_DIR/netrc-template"
                ;;
            "install")
                cp "$SECRETS_DIR/netrc-template" "$HOME/.netrc"
                chmod 600 "$HOME/.netrc"
                echo "✅ Installed to ~/.netrc"
                ;;
            *)
                echo "📋 .netrc template:"
                cat "$SECRETS_DIR/netrc-template"
                ;;
        esac
        ;;
        
    "sops")
        echo "🔒 Managing SOPS services..."
        case "${2:-show}" in
            "edit")
                SOPS_CONFIG_FILE="$SOPS_CONFIG" sops "$SECRETS_DIR/services.yaml"
                ;;
            "show")
                SOPS_CONFIG_FILE="$SOPS_CONFIG" sops -d "$SECRETS_DIR/services.yaml"
                ;;
            "get")
                service="${3:-}"
                if [ -z "$service" ]; then
                    echo "Usage: $0 sops get <service>"
                    exit 1
                fi
                SOPS_CONFIG_FILE="$SOPS_CONFIG" sops -d "$SECRETS_DIR/services.yaml" | yq ".$service"
                ;;
        esac
        ;;
        
    "export")
        service="${2:-}"
        case "$service" in
            "aws")
                echo "🌩️ Exporting AWS credentials..."
                creds=$(SOPS_CONFIG_FILE="$SOPS_CONFIG" sops -d "$SECRETS_DIR/services.yaml" | yq '.aws')
                export AWS_ACCESS_KEY_ID=$(echo "$creds" | yq '.access_key_id')
                export AWS_SECRET_ACCESS_KEY=$(echo "$creds" | yq '.secret_access_key')
                export AWS_DEFAULT_REGION=$(echo "$creds" | yq '.region')
                echo "✅ AWS credentials exported to environment"
                ;;
            "github")
                echo "🐙 Exporting GitHub token..."
                token=$(SOPS_CONFIG_FILE="$SOPS_CONFIG" sops -d "$SECRETS_DIR/services.yaml" | yq '.github.token')
                export GITHUB_TOKEN="$token"
                echo "✅ GITHUB_TOKEN exported"
                ;;
            *)
                echo "Available exports: aws, github"
                ;;
        esac
        ;;
        
    "list")
        echo "📋 Available secrets:"
        echo "  .netrc: $([ -f "$SECRETS_DIR/netrc-template" ] && echo "✅" || echo "❌")"
        echo "  SOPS services: $([ -f "$SECRETS_DIR/services.yaml" ] && echo "✅" || echo "❌")"
        if [ -f "$SECRETS_DIR/services.yaml" ]; then
            echo ""
            echo "🔒 SOPS services:"
            SOPS_CONFIG_FILE="$SOPS_CONFIG" sops -d "$SECRETS_DIR/services.yaml" | yq 'keys | .[]' | sed 's/^/  - /'
        fi
        ;;
        
    *)
        echo "🔐 Unified Secrets Manager"
        echo "Usage: $0 [command]"
        echo ""
        echo "Commands:"
        echo "  init                Initialize all secret templates"
        echo "  netrc [edit|install|show]  Manage .netrc credentials"
        echo "  sops [edit|show|get <service>]  Manage SOPS services"
        echo "  export <service>    Export service to environment"
        echo "  list               List all available secrets"
        echo ""
        echo "Examples:"
        echo "  $0 init                    # Setup all templates"
        echo "  $0 netrc edit              # Edit .netrc template"
        echo "  $0 sops edit               # Edit encrypted services"
        echo "  $0 export aws              # Export AWS to env vars"
        ;;
esac
