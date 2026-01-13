#!/bin/bash
# Value Lattice Server Init Script
# Usage: ./value_lattice_init.sh {start|stop|restart|status}

DAEMON_NAME="value_lattice_server"
DAEMON_PATH="$(pwd)/target/release/value_lattice_server"
PIDFILE="/tmp/value_lattice_server.pid"
LOGFILE="/tmp/value_lattice_server.log"
USER=$(whoami)

start() {
    if [ -f $PIDFILE ]; then
        echo "🔴 $DAEMON_NAME is already running (PID: $(cat $PIDFILE))"
        return 1
    fi
    
    echo "🚀 Starting $DAEMON_NAME..."
    
    # Build if needed
    if [ ! -f "$DAEMON_PATH" ]; then
        echo "📦 Building $DAEMON_NAME..."
        cargo build --release --bin value_lattice_server
    fi
    
    # Start daemon
    nohup $DAEMON_PATH > $LOGFILE 2>&1 &
    PID=$!
    echo $PID > $PIDFILE
    
    sleep 2
    if kill -0 $PID 2>/dev/null; then
        echo "✅ $DAEMON_NAME started (PID: $PID)"
        echo "🌐 Dashboard: http://localhost:3030/dashboard"
        echo "📊 API: http://localhost:3030/status"
        echo "📝 Logs: tail -f $LOGFILE"
    else
        echo "❌ Failed to start $DAEMON_NAME"
        rm -f $PIDFILE
        return 1
    fi
}

stop() {
    if [ ! -f $PIDFILE ]; then
        echo "🔴 $DAEMON_NAME is not running"
        return 1
    fi
    
    PID=$(cat $PIDFILE)
    echo "🛑 Stopping $DAEMON_NAME (PID: $PID)..."
    
    kill $PID
    sleep 2
    
    if kill -0 $PID 2>/dev/null; then
        echo "⚠️  Force killing $DAEMON_NAME..."
        kill -9 $PID
    fi
    
    rm -f $PIDFILE
    echo "✅ $DAEMON_NAME stopped"
}

status() {
    if [ -f $PIDFILE ]; then
        PID=$(cat $PIDFILE)
        if kill -0 $PID 2>/dev/null; then
            echo "🟢 $DAEMON_NAME is running (PID: $PID)"
            echo "🌐 Dashboard: http://localhost:3030/dashboard"
            echo "📊 Memory usage: $(ps -p $PID -o rss= | awk '{print $1/1024 " MB"}')"
            return 0
        else
            echo "🔴 $DAEMON_NAME is not running (stale PID file)"
            rm -f $PIDFILE
            return 1
        fi
    else
        echo "🔴 $DAEMON_NAME is not running"
        return 1
    fi
}

restart() {
    stop
    sleep 1
    start
}

case "$1" in
    start)
        start
        ;;
    stop)
        stop
        ;;
    restart)
        restart
        ;;
    status)
        status
        ;;
    *)
        echo "Usage: $0 {start|stop|restart|status}"
        echo ""
        echo "🔢 Value Lattice Server Control"
        echo "Commands:"
        echo "  start   - Start the server daemon"
        echo "  stop    - Stop the server daemon"  
        echo "  restart - Restart the server daemon"
        echo "  status  - Check server status"
        echo ""
        echo "Files:"
        echo "  PID:  $PIDFILE"
        echo "  Log:  $LOGFILE"
        echo "  Bin:  $DAEMON_PATH"
        exit 1
        ;;
esac

exit $?
