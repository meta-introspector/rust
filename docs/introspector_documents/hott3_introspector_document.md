# hott3 Introspector Document

## Overview

`hott3` is a Lean 3 project that serves as a work-in-progress port of the Homotopy Type Theory (HoTT) library from Lean 2. Its primary goal is to bring HoTT formalizations to Lean 3 while adhering to specific design principles, notably avoiding modifications to the Lean kernel and axiomatizing univalence.

## Key Features and Purpose

*   **Lean 3 Port of HoTT**: Aims to make existing Lean 2 HoTT formalizations compatible with Lean 3.
*   **Kernel Preservation**: Designed to operate without altering the Lean kernel, ensuring consistency with Lean 3's foundational principles.
*   **Univalence Axiom**: Incorporates the univalence axiom as a fundamental principle.
*   **Higher Inductive Types (HITs)**: Implements HITs using a specific technique that checks for proper usage and avoids inconsistent induction principles.
*   **Metaprogramming for Automation**: Leverages Lean 3's metaprogramming capabilities to develop domain-specific automation for HoTT proofs.
*   **Guidance for Contributors**: Provides clear guidelines and common changes for porting files and contributing to the project.

## Usage

This project is for users and developers interested in HoTT formalization within the Lean 3 environment. It provides a foundation for building and verifying mathematical theories in HoTT using Lean 3.

## Project Structure (High-Level)

The repository contains Lean source files (`.lean`) and documentation related to porting and contributing. It also mentions specific tactics and their usage within the Lean 3 environment.

## Mentors and Influences

`hott3` is influenced by the original Lean 2 HoTT library and the broader HoTT community. It represents an effort to continue the formalization of HoTT within the evolving Lean ecosystem.
