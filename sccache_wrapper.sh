#!/bin/bash

LOGFILE="sccache_calls.log"
SCCACHE_BIN="$HOME/.cargo/bin/sccache"

# Log the call with timestamp
echo "$(date '+%H:%M:%S') $*" >> "$LOGFILE"

# Get cache stats before
STATS_BEFORE=$($SCCACHE_BIN --show-stats 2>/dev/null)

# Call actual sccache
$SCCACHE_BIN "$@"
RESULT=$?

# Get cache stats after
STATS_AFTER=$($SCCACHE_BIN --show-stats 2>/dev/null)

# Log stats change
echo "$(date '+%H:%M:%S') STATS_CHANGE: $STATS_BEFORE -> $STATS_AFTER" >> "$LOGFILE"

# Check for duplicates
CURRENT_CALL="$*"
DUPLICATE_COUNT=$(grep -c "$CURRENT_CALL" "$LOGFILE" 2>/dev/null || echo 0)
if [ "$DUPLICATE_COUNT" -gt 1 ]; then
    echo "$(date '+%H:%M:%S') DUPLICATE #$DUPLICATE_COUNT: $CURRENT_CALL" >> "$LOGFILE"
fi

exit $RESULT
