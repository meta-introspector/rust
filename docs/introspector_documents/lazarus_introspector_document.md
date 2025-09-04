# Lazarus: Lattice-based Zero-knowledge Arguments Framework

**Path:** `vendor/Lazarus`

Lazarus is a Rust framework designed for implementing lattice-based zero-knowledge arguments. It aims to provide a modular set of building blocks for constructing efficient zero-knowledge proofs that rely on lattice assumptions.

## Key Features and Components:

*   **Lattice-based Cryptography:** Focuses on LWE (Learning With Errors) and SIS (Short Integer Solution) assumptions for its cryptographic primitives.
*   **Polynomial Commitment Schemes:** Supports polynomial commitment schemes, a crucial component for many zero-knowledge proof systems.
*   **Zero-Knowledge Proofs for Linear Relations:** Provides mechanisms to prove linear relationships without revealing the underlying data.
*   **Sigma Protocols:** Implements sigma protocols for lattice statements, which are a class of interactive zero-knowledge proofs.
*   **Fiat-Shamir Transformations:** Includes support for converting interactive proofs into non-interactive ones.
*   **Modular Building Blocks:** Designed with modularity in mind, allowing for flexible construction of various zero-knowledge arguments.
*   **Performance Benchmarks:** The `README.md` includes detailed benchmarks comparing Lazarus's performance (e.g., proof generation, verification, setup times, proof size, memory usage) against other frameworks like Labrador.

## Project Status:

Lazarus is currently under active development, and its API is subject to change. It is explicitly stated as **not ready for production use**.

## Project Structure (from `Cargo.toml`):

The `Lazarus` repository is a Rust workspace containing the following member crates:

*   `labrador`
*   `algebra`

## License:

The project is licensed under the Apache License Version 2.0.
