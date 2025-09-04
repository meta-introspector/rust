#!/bin/bash

# Start asciinema recording
/data/data/com.termux/files/home/storage/github/rustc/scripts/start_recording_v2.sh &

# Give asciinema a moment to start
sleep 2

# Define log file path with timestamp
LOG_FILE="/data/data/com.termux/files/home/storage/github/rustc/logs/gemini_non_interactive_output_$(date +"%Y%m%d_%H%M%S").log"

echo "Running gemini non-interactively and logging output to $LOG_FILE"

# Run the non-interactive gemini script and redirect all output to the log file
/data/data/com.termux/files/home/storage/github/rustc/scripts/run_gemini_non_interactive.sh &> "$LOG_FILE"

echo "Gemini non-interactive run complete. Check $LOG_FILE for output."
