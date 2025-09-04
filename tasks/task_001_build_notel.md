# Task ID: task_001_build_notel

## Description
Build the `notel` telemetry server and capture its build output. This task helps verify the build process for a newly vendorized component and provides a baseline for future development.

## Assigned To
[Unassigned]

## Status
Pending

## Instructions

1.  Navigate to the `notel` rust-server directory:
    ```bash
    cd vendor/telemetry-server/notel/rust-server
    ```
2.  Clean any previous build artifacts:
    ```bash
    cargo clean
    ```
3.  Build the `notel` server in release mode, redirecting output to a log file:
    ```bash
    cargo build --release &> ../../../../tasks/results/task_001_notel_build.log
    ```
    (Note: The `&>` redirects both stdout and stderr. The path to the log file is relative to the project root.)
4.  Return to the project root:
    ```bash
    cd -
    ```

## Expected Output/Results
*   A build log file: `tasks/results/task_001_notel_build.log`
*   Successful compilation of the `notel` server.

## Verification
Review the `tasks/results/task_001_notel_build.log` file for any errors or warnings. Ensure the `notel` executable is present in `vendor/telemetry-server/notel/rust-server/target/release/`.

## How to Submit
1.  Add the build log file to Git: `git add tasks/results/task_001_notel_build.log`
2.  Commit the changes with a message like: `feat: Complete task_001_build_notel - Notel build log`
3.  Update the status of this task in `tasks/task_001_build_notel.md` to "Completed".
