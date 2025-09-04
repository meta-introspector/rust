# qFALL-crypto: Quantum-Resistant Fast Lattice Library

**Path:** `vendor/qfall-crypto`

qFALL-crypto is a Rust library developed by a project group at Paderborn University. Its primary objective is to facilitate the rapid prototyping of lattice-based cryptography for researchers and students.

## Key Features and Offerings:

*   **Comprehensive Cryptographic Implementations:** Provides a variety of schemes, constructions, and primitives, including:
    *   **Public Key Encryption:** LWE, Dual LWE, LPR, Ring-based LPR, and CCA-secure encryption.
    *   **Signatures:** Full-Domain Hash (FDH), Probabilistic FDH (PFDH), and Ring-based FDH.
    *   **Identity Based Encryption:** Including an implementation derived from Dual LWE Encryption.
    *   **Hash Functions:** SIS-Hash Function and SHA-256-based Hash.
*   **Building Blocks and Primitives:** Offers foundational components such as Preimage Samplable Functions (PSF) and various Trapdoors (e.g., G-trapdoor with short basis).
*   **Quantum Resistance:** Focuses on lattice-based cryptography, which is considered quantum-resistant.

## Project Status:

The library is currently in the development phase, and its interfaces are subject to change. It is explicitly stated that the content will evolve, and an official release is anticipated in the second half of 2023. It is not yet ready for production use.

## Dependencies (from `Cargo.toml`):

The `qfall-crypto` crate depends on `qfall-math` from `https://github.com/qfall/math`.

## License:

This library is distributed under the Mozilla Public License Version 2.0 (MPL 2.0).

## Documentation and Contact:

The project provides a dedicated website (`https://qfall.github.io/`) as a central information point, including a tutorial for installation and usage. Contact can be made via a mailing list.
