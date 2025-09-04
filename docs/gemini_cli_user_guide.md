# Gemini CLI User Guide

Welcome to the Gemini Command Line Interface (CLI) agent! This guide will help you understand how to interact with me, leverage my capabilities, and get the most out of our collaboration.

## What is the Gemini CLI Agent?

I am an AI-powered assistant designed to help you with various software engineering tasks directly from your command line. I can understand natural language requests, analyze code, perform modifications, interact with your Git repository, and assist with documentation and creative tasks.

## Core Capabilities

I can assist you with a wide range of tasks, including but not limited to:

*   **Code Reading & Analysis:** Understand existing code, identify patterns, and explain complex logic.
*   **Code Modification:** Write new code, refactor existing code, fix bugs, and implement features.
*   **File System Operations:** Read, write, and manage files and directories within your project.
*   **Shell Command Execution:** Run shell commands to build, test, or interact with your system (with your explicit approval for critical commands).
*   **Git Integration:** Analyze Git history, understand changes, and assist with version control tasks.
*   **Documentation:** Create, update, and manage project documentation, including CRQs and SOPs.
*   **Creative Tasks:** Generate text, such as poems or descriptive content, based on your prompts.

## How to Interact

Simply type your requests in natural language. I will process your input and respond or propose actions.

## Best Practices for Prompting

To get the best results, consider the following when crafting your prompts. For a detailed template and examples, refer to `docs/gemini_cli_prompt_template.md`.

*   **Clarity & Specificity:** Be precise about what you want me to do. Avoid vague language.
*   **Context:** Provide all necessary background information, including relevant file paths, code snippets, and project conventions.
*   **Desired Output:** Clearly state the format and content you expect in my response (e.g., "as a Rust code block," "summarize in bullet points," "create a new file").
*   **Constraints & Guidelines:** Specify any limitations, preferred approaches, or coding standards I should adhere to.
*   **Iterative Approach:** For complex tasks, consider breaking them down into smaller, manageable steps.

## Safety and Control

Your control is paramount. For any command that modifies your file system or executes potentially impactful shell commands, I will always ask for your explicit approval before proceeding. Review my proposed actions carefully.

## Getting Help

If you ever need assistance or a reminder of my capabilities, you can type `/help`.

## Examples of Interaction

Here are a few examples of how you might interact with me:

*   `Refactor the `main` function in `src/main.rs` to improve error handling.`
*   `Read the content of `Cargo.toml` and summarize the dependencies.`
*   `Create a new file `src/utils/helper.rs` with a function `fn greet(name: &str) -> String`.`
*   `Analyze the Git history for `docs/README.md` and summarize recent changes.`
*   `Write a poem about the importance of modularity in software.`

I am here to assist you. Let's build something great together!

## Optimizing Context and Control with CLI Options

When launching the Gemini CLI, you can use various command-line options to provide better context, control its behavior, and enhance your development workflow.

*   **`-m, --model <model_name>`**:
    *   **Purpose**: Specifies the AI model to use for interactions. Different models may have varying capabilities, knowledge bases, and response styles.
    *   **Usage Example**: `gemini -m gemini-2.5-flash`

*   **`--include-directories <path1>,<path2>,...`**:
    *   **Purpose**: Informs the model about specific directories that are relevant to your current task. This helps the model focus its understanding and search within a defined scope, providing more accurate and relevant responses. You can specify multiple directories separated by commas or use the flag multiple times.
    *   **Usage Example**: `gemini --include-directories src/backend,docs/api`
    *   **Note**: This is generally preferred over `--all-files` for better performance and more focused context.

*   **`-a, --all-files`**:
    *   **Purpose**: Includes all files in the project as context for the model. Use with caution, as this can be resource-intensive and may dilute the context for specific tasks.
    *   **Usage Example**: `gemini --all-files`

*   **`-c, --checkpointing`**:
    *   **Purpose**: Enables checkpointing of file edits. This is crucial for maintaining a robust development process, allowing you to track and potentially revert changes made by the agent.
    *   **Usage Example**: `gemini --checkpointing`

*   **`-p, --prompt <your_prompt>`**:
    *   **Purpose**: Runs the Gemini CLI in a non-interactive mode, executing a single prompt and then exiting. Useful for scripting or automated tasks.
    *   **Usage Example**: `gemini -p "Summarize the main functions in src/main.rs" --model gemini-2.5-flash`

*   **`-i, --prompt-interactive <initial_prompt>`**:
    *   **Purpose**: Executes an initial prompt and then transitions into interactive mode. This allows you to provide an initial context or task, and then continue a conversation with the agent.
    *   **Usage Example**: `gemini -i "Refactor the authentication logic in src/auth.py" --model gemini-2.5-flash`

*   **`-s, --sandbox`**:
    *   **Purpose**: Runs the Gemini CLI within a sandboxed environment. This enhances security by isolating the agent's operations, especially when dealing with potentially sensitive or experimental tasks.
    *   **Usage Example**: `gemini -s --model gemini-2.5-flash`

By effectively utilizing these options, you can tailor the Gemini CLI's behavior to your specific needs, providing it with the optimal context for efficient and accurate assistance.
