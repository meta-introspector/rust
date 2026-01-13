# Unified P2P Server System Service

This directory contains the systemd service configuration and management scripts for running the Unified P2P Rust Compilation Server as a proper system service.

## Quick Setup

```bash
# Run as root to install the service
sudo ./setup-service.sh

# Manage the service
./manage-service.sh status    # Check status
./manage-service.sh logs      # Follow logs
./manage-service.sh restart   # Restart service
./manage-service.sh update    # Update and restart
```

## Service Details

- **User**: `p2p-rust` (dedicated system user)
- **Directory**: `/opt/unified-p2p-server`
- **Config**: `/etc/unified-p2p-server/config.env`
- **Service**: `unified-p2p-server.service`

## Files

- `unified-p2p-server.service` - Systemd service definition
- `unified-p2p-server.env` - Environment configuration
- `setup-service.sh` - Installation script (run as root)
- `manage-service.sh` - Service management commands

## Security Features

- Dedicated system user with minimal privileges
- Protected filesystem access
- Resource limits (file descriptors, processes)
- No new privileges escalation
- Private temporary directory

## Configuration

Edit `/etc/unified-p2p-server/config.env` to customize:
- Network ports and binding
- Compilation settings
- Security options
- Performance tuning

## Logs

View logs with:
```bash
sudo journalctl -u unified-p2p-server -f
```

## Self-Management

The server includes self-compilation and update capabilities, but these are disabled by default in production for security. Enable with `ENABLE_SELF_UPDATE=true` in config.env if needed.
