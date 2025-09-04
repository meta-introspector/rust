# Task ID: task_002_extract_next_tasks

## Description
Extract potential "next tasks" from all project files by searching for common indicators like TODO, FIXME, HACK, XXX, @future, @next, and unchecked Markdown checkboxes.

## Assigned To
[Unassigned]

## Status
Pending

## Instructions

1.  **Understand the Scope and Memory Constraints**: This project contains over 100MB of files. Processing all files at once can lead to memory exhaustion and crashes. The `search_file_content` tool processes files one by one, which helps mitigate this.

2.  **Execute the Extraction Script**: Run the dedicated script to perform the extraction:
    ```bash
    ./scripts/extract_next_tasks.sh
    ```
    This script will iterate through relevant project files and search for the defined patterns. It will append all findings to a summary file.

3.  **Monitor System Resources**: During the execution of the script, it is crucial to monitor your system's memory usage. If memory usage approaches critical levels, consider stopping the script and refining the search scope (e.g., by excluding certain large directories or file types within the `extract_next_tasks.sh` script).

## Expected Output/Results
*   A summary file: `tasks/results/next_tasks_summary.md` containing all identified "next tasks" with their file paths and line numbers.

## Verification
Review the `tasks/results/next_tasks_summary.md` file. It should contain a list of potential tasks. Verify that the script ran to completion without crashing.

## How to Submit
1.  Add the summary file to Git: `git add tasks/results/next_tasks_summary.md`
2.  Commit the changes with a message like: `feat: Complete task_002_extract_next_tasks - Extracted next tasks summary`
3.  Update the status of this task in `tasks/task_002_extract_next_tasks.md` to "Completed".
