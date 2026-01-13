#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

set -euo pipefail

# Management script for Unified P2P Server
# Usage: ./manage-service.sh [start|stop|restart|status|logs|update]

SERVICE_NAME="unified-p2p-server"
SERVICE_DIR="/opt/unified-p2p-server"

case "${1:-status}" in
    start)
        echo "🚀 Starting $SERVICE_NAME..."
        sudo systemctl start $SERVICE_NAME
        sudo systemctl status $SERVICE_NAME --no-pager
        ;;
    stop)
        echo "🛑 Stopping $SERVICE_NAME..."
        sudo systemctl stop $SERVICE_NAME
        ;;
    restart)
        echo "🔄 Restarting $SERVICE_NAME..."
        sudo systemctl restart $SERVICE_NAME
        sudo systemctl status $SERVICE_NAME --no-pager
        ;;
    status)
        echo "📊 Status of $SERVICE_NAME:"
        sudo systemctl status $SERVICE_NAME --no-pager
        ;;
    logs)
        echo "📋 Following logs for $SERVICE_NAME (Ctrl+C to exit):"
        sudo journalctl -u $SERVICE_NAME -f
        ;;
    update)
        echo "🔄 Updating $SERVICE_NAME..."
        echo "Building new version..."
        cargo build --release --bin unified_p2p_server
        
        echo "Stopping service..."
        sudo systemctl stop $SERVICE_NAME
        
        echo "Updating binary..."
        sudo cp target/release/unified_p2p_server $SERVICE_DIR/
        sudo chown p2p-rust:p2p-rust $SERVICE_DIR/unified_p2p_server
        
        echo "Starting service..."
        sudo systemctl start $SERVICE_NAME
        sudo systemctl status $SERVICE_NAME --no-pager
        ;;
    *)
        echo "Usage: $0 [start|stop|restart|status|logs|update]"
        exit 1
        ;;
esac
