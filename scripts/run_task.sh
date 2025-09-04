#!/bin/bash

TASK_ID=$1
TASK_FILE="tasks/${TASK_ID}.md"

if [ -z "$TASK_ID" ]; then
  echo "Usage: ./run_task.sh <task_id>"
  exit 1
fi

if [ ! -f "$TASK_FILE" ]; then
  echo "Error: Task file '$TASK_FILE' not found."
  exit 1
fi

echo "--- Running Task: $TASK_ID ---"

# Extract instructions from the Markdown file
# This is a simplified extraction. A more robust solution might use a Markdown parser.
# For now, it assumes instructions are within a bash code block after "## Instructions"
INSTRUCTIONS=$(awk '/^## Instructions/{flag=1;next}/^## Expected Output/{flag=0}flag' "$TASK_FILE" | grep -v '```' | sed 's/^[[:space:]]*//')

if [ -z "$INSTRUCTIONS" ]; then
  echo "Warning: No instructions found in '$TASK_FILE'."
else
  echo "Executing instructions:"
  echo "$INSTRUCTIONS"
  # Execute the instructions. Be careful with arbitrary code execution!
  # For this simplified example, we'll just echo them.
  # In a real system, this would involve careful sandboxing and execution.
  eval "$INSTRUCTIONS"
fi

echo "--- Task $TASK_ID execution complete. ---"
echo "Please review the task file for next steps (e.g., committing results)."
