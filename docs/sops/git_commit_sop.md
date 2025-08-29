# Standard Operating Procedure (SOP): Git Commit Process with Multi-Line Messages

## 1. Objective
To standardize the process of committing changes in the Git repository, specifically enabling the use of multi-line commit messages and ensuring consistency and clarity in commit history.

## 2. Scope
This SOP applies to all `git commit` operations performed within this project, particularly when a commit message requires more than a single line of text.

## 3. Procedure

### Step 3.1: Stage Changes
*   **Action:** Ensure all desired changes (new files, modified files, deleted files) are added to the Git staging area using `git add <file(s)>` or `git add .`.
*   **Verification:** Run `git status` to confirm that all relevant changes are staged for commit.
*   **Tool:** `run_shell_command` (`git add`, `git status`)

### Step 3.2: Create Commit Message File
*   **Action:** Create a temporary text file (e.g., `commit_message.txt`) in the project root directory. Populate this file with your desired multi-line commit message. The first line should be a concise subject line (max 50-72 characters), followed by a blank line, and then a more detailed body.
*   **Example `commit_message.txt` content:**
    ```
    feat: Implement new feature X

    This commit introduces the new feature X, which allows users to...
    It addresses the following issues: #123, #456.
    ```
*   **Tool:** `write_file`

### Step 3.3: Execute Git Commit
*   **Action:** Execute the `git commit` command, specifying the temporary file containing the commit message using the `-F` flag.
*   **Command:** `git commit -F commit_message.txt`
*   **Verification:** The commit should be created successfully. Run `git log -1` to verify the commit message.
*   **Tool:** `run_shell_command`

### Step 3.4: Clean Up Commit Message File
*   **Action:** Delete the temporary commit message file to keep the project directory clean.
*   **Command:** `rm commit_message.txt`
*   **Tool:** `run_shell_command`

## 4. Best Practices
*   **Subject Line:** Keep the subject line concise and informative (imperative mood, max 50-72 chars).
*   **Body:** Use the body to explain the *why* and *what* of the change, providing context and details that are not immediately obvious from the code.
*   **References:** Include references to issue trackers (e.g., `#123`) or related pull requests.

## 5. Tools Used
*   `run_shell_command`: For all Git commands (`git add`, `git status`, `git commit`, `git log`, `rm`).
*   `write_file`: For creating the temporary commit message file.
