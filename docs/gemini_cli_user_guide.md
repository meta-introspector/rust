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