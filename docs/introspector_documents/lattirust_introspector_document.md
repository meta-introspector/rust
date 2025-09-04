# Lattirust: Lattice-based Cryptography Library

**Path:** `vendor/lattirust`

Lattirust is a Rust library dedicated to safe and efficient lattice-based cryptography, with a particular focus on zero-knowledge proofs. It aims to be a foundational library in the Rust ecosystem for lattice constructions, drawing inspiration from projects like `arkworks` and `lattigo`.

## Key Features and Components:

*   **Quantum-Secure Cryptography:** Leverages lattice assumptions for quantum-secure cryptographic protocols, often more efficient due to their inherent structure.
*   **Rust Advantages:** Built in Rust to benefit from its memory safety, fine-grained security properties via its type system, close-to-the-metal performance, and compatibility with the growing zero-knowledge proof ecosystem in Rust.
*   **Core Arithmetic:** Includes `lattirust_arithmetic` for implementations of power-of-two cyclotomic rings, number-theoretic transforms, matrices, vectors, and various norms. It also provides wrappers around the `nimue` library for secure Fiat-Shamir transformations.
*   **Lattice Estimators:** Integrates `lattice-estimator` for wrappers around external lattice estimators (e.g., for SIS and MSIS).
*   **Implemented Protocols:** Features implementations of cryptographic protocols such as LaBRADOR (concretely small proof sizes, linear verifier) and Lova (a lattice folding scheme).

## Project Structure (from `Cargo.toml`):

The `lattirust` repository is a Rust workspace containing the following member crates:

*   `lattirust-arithmetic`
*   `lattice-estimator`
*   `relations`

## Security Note:

Lattirust is currently provided for research and prototyping purposes. It has not been audited and is not suitable for real-world, security-critical deployments. Users are advised to consult with a trusted cryptographer before using it in production environments.

## Future Plans (from `README.md`):

Planned implementations include proof-of-encryption/decryption for LWE/RLWE, a fully integrated GPU backend, an in-tree Rust estimator, the Greyhound proof system, and the LatticeFold folding scheme.
