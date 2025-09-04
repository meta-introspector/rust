# Ground Zero Introspector Document

## Overview

`ground_zero` is a Lean 4 project dedicated to developing Homotopy Type Theory (HoTT). Similar to `hott3` for Lean 3, it aims to formalize HoTT without modifying the Lean kernel, primarily by constructing Higher Inductive Types (HITs) using Lean's quotient types.

## Key Features and Purpose

*   **HoTT in Lean 4**: A foundational library for formalizing Homotopy Type Theory within the Lean 4 theorem prover.
*   **Kernel-Compatible HITs**: Defines HITs (such as Interval, Pushout, Homotopical reals, Colimit, Suspension, Circle, Sphere, and Join) using quotients, ensuring compatibility with the Lean 4 kernel.
*   **Large Eliminator Checker**: Incorporates a mechanism to detect and prevent the use of inconsistent induction principles, ensuring the soundness of the formalization.
*   **Dependency Map**: Provides a visual dependency map to illustrate the structure and interconnections of its formalizations.

## Usage

This project is for researchers and developers interested in the formalization of HoTT in Lean 4. It provides a robust framework for building and verifying advanced mathematical concepts within a type-theoretic setting.

## Project Structure (High-Level)

The repository contains Lean source files (`.lean`) organized into modules, particularly focusing on `GroundZero/HITs` for Higher Inductive Types and `GroundZero/Meta` for metaprogramming tools like the `HottTheory` checker. It also includes a `README.md` with a dependency map and licensing information.

## Mentors and Influences

`ground_zero` is influenced by previous HoTT formalization efforts in Lean (like `gebner/hott3`) and the broader HoTT community. It contributes to the ongoing development of formal mathematics in Lean 4.
