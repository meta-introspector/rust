# Solfunmeme Agent Key Management

Secure key generation and management system for the solfunmeme agent with PIN-based encryption.

## Quick Setup

```bash
# Generate all keys with single PIN
./setup-agent-keys.sh

# Daily key operations
./manage-agent-keys.sh status
./manage-agent-keys.sh show-pin
./manage-agent-keys.sh decrypt-secrets
```

## What Gets Generated

1. **Secure PIN** (16 chars) - Master key for everything
2. **GPG Key Pair** (4096-bit RSA) - For SOPS encryption
3. **SSH Key** (Ed25519) - For Git and server access  
4. **SOPS Configuration** - Encrypted secrets management
5. **Sample Secrets File** - Oracle Cloud credentials template

## Key Features

- **Single PIN** unlocks all keys
- **GPG-encrypted** secrets with SOPS
- **Automatic SOPS** installation and configuration
- **Secure file permissions** (600/700)
- **Encrypted backups** capability
- **Status monitoring** for all components

## File Structure

```
~/.solfunmeme-keys/
├── agent.pin              # Master PIN (keep safe!)
├── ssh_key                # SSH private key
├── ssh_key.pub            # SSH public key  
├── gpg_public.asc         # GPG public key
├── gpg_private.asc        # GPG private key
├── .sops.yaml             # SOPS configuration
└── secrets/
    └── oracle-credentials.yaml  # Encrypted Oracle creds
```

## Security Model

- **PIN-protected**: All keys use the same PIN for consistency
- **GPG-encrypted**: Secrets encrypted with generated GPG key
- **File permissions**: Restrictive permissions on all key files
- **No plaintext**: Credentials stored encrypted at rest

## Usage Examples

```bash
# Show your PIN
./manage-agent-keys.sh show-pin

# Load SSH key for Git operations
./manage-agent-keys.sh load-ssh

# Edit Oracle credentials securely
./manage-agent-keys.sh edit-secrets

# Create encrypted backup
./manage-agent-keys.sh backup
```

## Integration

The generated keys integrate with:
- **Oracle Cloud**: Encrypted OCI credentials
- **Git repositories**: SSH key for authentication
- **SOPS**: Encrypted configuration management
- **P2P server**: Secure credential storage

## Recovery

If you lose access, you need:
1. The PIN from `agent.pin` file
2. The GPG private key (encrypted with PIN)
3. The SSH private key (encrypted with PIN)

Keep the PIN safe - it's your master key to everything!
