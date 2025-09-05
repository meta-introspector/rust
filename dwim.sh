#!/bin/bash

# Define paths and variables
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
SESSION_NAME="dwim_session_$TIMESTAMP"
EMACS_DIR="/data/data/com.termux/files/home/storage/github/rustc/"
RECORDING_FILE="/data/data/com.termux/files/home/storage/github/rustc/docs/asciicast_$TIMESTAMP.cast"
NOTEL_SERVER_SCRIPT="/data/data/com.termux/files/home/storage/github/rustc/scripts/start_notel_server.sh"

# Start asciinema recording in the background
asciinema rec "$RECORDING_FILE" &
echo "Asciinema recording started: $RECORDING_FILE"

# Give asciinema a moment to start
sleep 2

# Start the notel server in the background (optional, comment out if not needed)
if [ -f "$NOTEL_SERVER_SCRIPT" ]; then
  "$NOTEL_SERVER_SCRIPT"
else
  echo "Warning: notel server script not found at $NOTEL_SERVER_SCRIPT. Skipping."
fi

# Determine the gemini prompt argument
GEMINI_PROMPT_ARG=""
if [ -n "$1" ]; then
  # If a prompt is provided, use it for interactive mode
  GEMINI_PROMPT_ARG="-i \"$1\""
  echo "Starting Gemini with initial prompt: \"$1\""
else
  # Otherwise, start Gemini in general interactive mode
  echo "Starting Gemini in general interactive mode."
fi

# Define the command to be executed by asciinema
# This command will set up tmux and launch gemini/emacs
TMUX_COMMAND="
  # Start the notel server in the background (optional, comment out if not needed)
  if [ -f \"$NOTEL_SERVER_SCRIPT\" ]; then
    \"$NOTEL_SERVER_SCRIPT\"
  else
    echo \"Warning: notel server script not found at $NOTEL_SERVER_SCRIPT. Skipping.\"
  fi

  # Start a new tmux session if it doesn't exist
  tmux has-session -t $SESSION_NAME
  if [ \$? != 0 ]; then
    tmux new-session -s $SESSION_NAME -d
  fi

  # Create a new window for the development session
  tmux new-window -t $SESSION_NAME:1 -n \"dev\"

  # Split the window into two panes
  tmux split-window -h -t $SESSION_NAME:1

  # Run the gemini command in the first pane (left)
  tmux send-keys -t $SESSION_NAME:1.0 \"gemini --model gemini-2.5-flash --checkpointing=true --include-directories ~/storage/github --debug --telemetry --telemetry-log-prompts $GEMINI_PROMPT_ARG\" C-m

  # Run emacs in the second pane (right)
  tmux send-keys -t $SESSION_NAME:1.1 \"cd $EMACS_DIR && emacs\" C-m

  # Select the first pane and attach to the session
  tmux select-pane -t $SESSION_NAME:1.0
  tmux attach-session -t $SESSION_NAME
"

# Start asciinema recording, executing the TMUX_COMMAND
echo "Asciinema recording started: $RECORDING_FILE"
asciinema rec "$RECORDING_FILE" --command "$TMUX_COMMAND"


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
tmux send-keys -t $SESSION_NAME:1.0 "gemini --model gemini-2.5-flash --checkpointing=true --include-directories ~/storage/github --debug --telemetry --telemetry-log-prompts $GEMINI_PROMPT_ARG" C-m

# Run emacs in the second pane (right)
tmux send-keys -t $SESSION_NAME:1.1 "cd $EMACS_DIR && emacs" C-m

# Select the first pane and attach to the session
tmux select-pane -t $SESSION_NAME:1.0
tmux attach-session -t $SESSION_NAME
