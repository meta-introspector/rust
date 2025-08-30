# Why We Don't Add All of Rust into Our Bootstrap Code Directly

The project's core philosophy revolves around **deep bootstrap, formal verification, extreme modularity, and minimizing the Trusted Computing Base (TCB)**. Adding the entire Rust compiler and standard library directly into our `rust-bootstrap` would fundamentally contradict these principles and introduce significant challenges:

1.  **Complexity and Maintainability:**
    *   The Rust compiler (`rustc`) and its standard library are massive, complex codebases. Integrating them directly would make `rust-bootstrap` unwieldy, difficult to understand, and extremely challenging to maintain.
    *   Debugging would become a nightmare, as issues could arise from any part of the integrated Rust codebase, blurring the lines between the bootstrap tool and the compiler it's building.

2.  **Minimizing the Trusted Computing Base (TCB):**
    *   A primary goal of a deep bootstrap is to minimize the TCB – the amount of code that *must* be trusted to ensure the integrity and correctness of the final system.
    *   If `rust-bootstrap` contained the entire Rust compiler, the TCB would be enormous. Our approach aims to build Rust from a minimal, formally verifiable base, ensuring trustworthiness from the ground up. This is why we focus on building GNU Guix/Nix from GNU Mes, starting from a hex loader, and using Lean4 for formal verification.

3.  **Formal Verification Challenges:**
    *   Formally verifying a codebase of the Rust compiler's size and complexity is an astronomical task.
    *   By keeping `rust-bootstrap` lean and modular, we increase the feasibility of formally verifying its critical components, which is a cornerstone of this project's trustworthiness goals. The "one declaration per file" and monadic refactoring principles are designed to aid in this.

4.  **Modularity and Reusability:**
    *   The project emphasizes "extreme modularity" where each function or logical block resides in its own file. This promotes reusability, testability, and independent verification of components.
    *   A monolithic `rust-bootstrap` containing the entire Rust codebase would destroy this modularity, making it impossible to leverage individual components or test them in isolation.

5.  **Self-Hosting vs. Deep Bootstrap:**
    *   Rust is a self-hosting compiler, meaning it can compile itself. However, our project aims for a *deep bootstrap*, which goes beyond mere self-hosting. A deep bootstrap starts from a much more fundamental and minimal set of trusted components.
    *   The `rust-bootstrap` tool's role is to *orchestrate* the building of the Rust compiler, not to *be* the Rust compiler. It acts as the "universal translator" and "creative collaborator" that guides the process, leveraging existing `cargo` and `rustc` libraries where appropriate, but not embedding their entire source.

6.  **Performance and Resource Usage:**
    *   Loading and managing the entire Rust codebase within a single bootstrap tool would lead to excessive memory consumption and slower execution, especially on resource-constrained environments like ARM64 Termux.

In essence, our `rust-bootstrap` is designed to be the **orchestrator and verifier** of the Rust build process, not a monolithic re-implementation of the Rust compiler itself. It leverages the existing, well-tested `cargo` and `rustc` libraries for their core functionalities, while providing the necessary control, introspection, and formal verification hooks required for a truly trustworthy and deeply bootstrapped Rust toolchain.
