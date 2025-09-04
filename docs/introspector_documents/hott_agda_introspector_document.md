# HoTT-Agda Introspector Document

## Overview

`HoTT-Agda` is a formalization of Homotopy Type Theory (HoTT) and univalent foundations within the Agda proof assistant. It provides a rigorous, machine-checked development of HoTT concepts, including higher inductive types and their computational rules.

## Key Features and Purpose

*   **Homotopy Type Theory and Univalent Foundations**: Implements core concepts of HoTT, such as the identity type and truncation levels, and explores univalent foundations.
*   **Agda Implementation**: Leverages Agda's dependent type system and features like `--without-K` (to restrict pattern matching for uniqueness of identity proofs) and `--rewriting` (for computational rules of higher inductive types).
*   **Structured Development**: The codebase is organized into `hott-core` (basic definitions) and `hott-theorems` (advanced theorems and applications).
*   **Style and Naming Conventions**: Adheres to specific conventions for line length, capitalization, and naming of types, terms, and theorems to maintain consistency and readability.
*   **Cross-Pollination**: Shares ideas with other HoTT formalization efforts, such as the Coq-HoTT library.

## Usage

To use `HoTT-Agda`, users need Agda 2.5.3 (or newer, with some caveats) and must include the library paths in their Agda library list. Each Agda file typically includes specific options in its header (`--without-K --rewriting`).

## Project Structure (High-Level)

The project is divided into `core/` (for fundamental definitions and types) and `theorems/homotopy/` and `theorems/cohomology/` (for proofs of homotopy-theoretic and cohomology-theoretic theorems, respectively). There's also an `old/` directory for legacy code and a `stash/` for experimental work.

## Mentors and Influences

`HoTT-Agda` is a collaborative effort, with significant contributions from various individuals in the HoTT community. It is influenced by foundational work in HoTT and other theorem proving projects.
