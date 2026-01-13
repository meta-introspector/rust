#!/bin/bash
# Log Archive Management Script

ARCHIVE_ROOT="log_archive"
DATE=$(date +%Y/%m/%d)
ARCHIVE_DIR="$ARCHIVE_ROOT/$DATE"

archive_logs() {
    mkdir -p "$ARCHIVE_DIR"
    
    # Archive root level logs
    find . -maxdepth 1 -name "*.log" -exec mv {} "$ARCHIVE_DIR/" \;
    
    # Copy compiler logs (preserve originals for active development)
    find compiler/ -name "*.log" -exec cp {} "$ARCHIVE_DIR/" \;
    
    echo "Logs archived to: $ARCHIVE_DIR"
    echo "Files archived: $(ls -1 "$ARCHIVE_DIR"/*.log 2>/dev/null | wc -l)"
}

list_archives() {
    find "$ARCHIVE_ROOT" -name "*.log" | head -10
    echo "..."
    echo "Total archives: $(find "$ARCHIVE_ROOT" -name "*.log" | wc -l)"
}

case "$1" in
    "archive") archive_logs ;;
    "list") list_archives ;;
    *) echo "Usage: $0 {archive|list}" ;;
esac
