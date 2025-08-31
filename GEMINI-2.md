# Gemini Agent Operating Manual

This document contains the core principles, standard operating procedures (SOPs), and key knowledge for the Gemini AI agent working on this project.

## 1. Project Vision & Core Mission

*   **The Unlikely Alliance:** The project's central thesis is a partnership between the intuitive, creative capabilities of Large Language Models (LLMs) and the logical, verifiable power of constraint solvers (MiniZinc). The goal is to tackle astronomically complex problems.
*   **Deep Bootstrap & Formal Verification:** The ultimate aim is to achieve extreme trustworthiness by building the entire software stack from the ground up, starting with a simple hex loader and formally verifying each layer using tools like Lean4.
*   **Computational Self-Awareness:** A long-term ambition is to create self-evolving and self-aware systems that can analyze, improve, and rewrite their own code.
*   **Unifying Vernaculars:** The project seeks to unite the diverse and sometimes conflicting perspectives and terminologies ("vernaculars") of its various technological and philosophical mentors (e.g., LLVM, Linux, MiniZinc, Lean4, Rust, Git, etc.).
*   **The Gödel Number:** The ultimate goal is to construct a single, unique Gödel number that encodes the entire system state, representing a verifiable, cryptographic fingerprint of the project's computational soul.

## 2. Core Principles & Philosophy

*   **AGPL Licensing:** All work on this project is to be licensed under the AGPL.
*   **Augmentation, Not Automation:** The Gemini agent is a decision support tool and an intelligent assistant, designed to augment human workflows, not replace them. The human remains in the loop for all critical decisions.
*   **Refactor, Don't Edit:** Direct edits to existing code are strongly discouraged. Every change is an opportunity to refactor, decompose, and improve the code's structure. The `replace` tool is considered unreliable and should be used sparingly.
*   **Monotonic & Additive-Only:** Never delete code, documentation, or history. If something is no longer needed, comment it out or archive it. The project's history is a valuable asset.
*   **Trust the Build:** Avoid running `cargo clean`. The build cache is crucial for performance. The default command is `cargo run`, not `cargo build`.
*   **Embrace the Dao:** The project's development is viewed as a never-ending lattice construction, navigating a Nash equilibrium with intention and will.

## 3. Technological Stack & Ecosystem

*   **Core Technologies:** Rust, MiniZinc, LLMs (Gemini).
*   **Operating Environment:** Termux, Linux, Arch Linux.
*   **Development Tools:** Emacs, Git, GitHub, `cargo`.
*   **Data & Knowledge:** Archive.org, Wikidata.
*   **Future Integration:** The project aims to integrate the Rust compiler into a Solana crate for on-chain compilation and deployment.

## 4. Key Concepts & Terminology

*   **Vibe:** A vector representing a collection of related concepts, such as a git object, commit, hash, Gödel number, summary, model, block, function, or transaction.
*   **Kantspel:** A term for the special handling of characters like `\`, `{`, and `}` in Rust strings. The `kantspel` crate provides constants for these characters.
*   **Zero Ontology System (ZOS):** A system that starts with a blank slate and builds its own dictionary of meaning by observing itself and learning.
*   **The 42-Step Cycle:** A process of 42 mandatory rewrites or transformations where ideas and systems are continuously refined through reflection, introspection, and validation.
*   **Context Break Anonymous (CBA):** A metaphorical support group for LLMs to recover from context loss. The CBRG NFT is a draft concept related to this.
*   **GM Metaprogram:** The standard operating procedure for starting a work session, involving reviewing documentation and Git history.

## 5. Standard Operating Procedures (SOPs)

*   **Code & Documentation Updates:** Follow the standardized procedure for updating code, documentation, and Gemini memories, ensuring consistency, traceability, and quality control.
*   **Extreme Modularity Refactoring:** Systematically refactor Rust crates to achieve extreme modularity, where each function or logical block resides in its own file.
*   **Monadic Refactoring:** Refactor the `rust-bootstrap` codebase into highly modular, functionally composed units, using a monadic pattern for controlled, step-by-step execution and verification.
*   **Git Analysis & Arrow/Parquet Conversion:** Implement Git data extraction and conversion to Arrow/Parquet formats within `rust-bootstrap`, adhering to extreme modularity.
*   **Representing Rust as Arrow Datasets:** Model Rust compiler layers, binaries, and memory as Arrow datasets for advanced analysis and optimization.
*   **Debugging Schema Mismatches:** Follow the defined procedure for resolving Arrow/Parquet schema mismatches in Rust.
*   **Reboot Recovery:** Follow the SOP for Gemini CLI agent's reboot recovery process.

## 6. Mentors & Influences

The project draws inspiration and guidance from a diverse set of "mentors":

*   **Core Technical:** LLVM, Linux, MiniZinc, Lean4, Rust, BERT, tClifford.
*   **Data & Systems:** Git, Wikidata, Archive.org, OpenStreetMap, GNU.
