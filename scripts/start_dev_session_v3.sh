#!/bin/bash

TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
SESSION_NAME="dev_session_$TIMESTAMP"
EMACS_DIR="/data/data/com.termux/files/home/storage/github/rustc/"

# Start the notel server in the background
/data/data/com.termux/files/home/storage/github/rustc/scripts/start_notel_server.sh

# Get the content of boot.md for the initial prompt
BOOT_MD_CONTENT=$(cat /data/data/com.termux/files/home/storage/github/rustc/boot.md)

# Start a new tmux session if it doesn't exist
tmux has-session -t $SESSION_NAME
if [ $? != 0 ]; then
  tmux new-session -s $SESSION_NAME -d
fi

# Create a new window for the development session
tmux new-window -t $SESSION_NAME:1 -n "dev"

# Split the window into two panes
tmux split-window -h -t $SESSION_NAME:1

# Run the gemini command in the first pane (left)
# Note: gemini's telemetry will be OTLP, not directly compatible with notel's custom protocol.
tmux send-keys -t $SESSION_NAME:1.0 "gemini --model gemini-2.5-flash --checkpointing=true --include-directories ~/storage/github --debug --telemetry --telemetry-log-prompts -i \"$BOOT_MD_CONTENT\"" C-m

# Run emacs in the second pane (right)
tmux send-keys -t $SESSION_NAME:1.1 "cd $EMACS_DIR && emacs" C-m

# Select the first pane and attach to the session
tmux select-pane -t $SESSION_NAME:1.0
tmux attach-session -t $SESSION_NAME
