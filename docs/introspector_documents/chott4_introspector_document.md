# cHoTT4 Introspector Document

## Overview

`cHoTT4` is a Lean 4 project dedicated to formalizing a model of Cubical Type Theory within Homotopy Type Theory (HoTT). Its core objectives include proving Function Extensionality and Univalence, and constructing Higher Inductive Types (HITs) in this framework. The project navigates the intricacies of Lean 4 to ensure consistency with HoTT principles, particularly by managing universe levels and avoiding certain Lean 4 features.

## Key Features and Purpose

*   **Cubical Type Theory in HoTT**: Formalizes a model of Cubical Type Theory, which provides a computational interpretation of HoTT.
*   **Proof of Univalence and HITs**: Aims to formally prove the Univalence Axiom and construct HITs within its cubical HoTT model, leveraging the computational properties of Cubical Type Theory.
*   **Lean 4 Compatibility**: Developed specifically for Lean 4, with strategies to handle potential inconsistencies between Lean 4's kernel and HoTT principles (e.g., by redefining equality and using namespaces).
*   **Foundational Research**: Contributes to the foundational understanding and formal verification of advanced type theory concepts.

## Usage

This repository is primarily for researchers and formal methods enthusiasts working on the foundations of type theory, HoTT, and Cubical Type Theory. It provides a reference implementation and a platform for further research in these areas.

## Project Structure (High-Level)

The project contains Lean source files (`.lean`) organized to reflect the formalization of Cubical Type Theory concepts. The `README.md` provides a detailed explanation of its design choices and goals.

## Mentors and Influences

`cHoTT4` is heavily influenced by the work of Cohen, Coquand, Huber, and Mörtberg on Cubical Type Theory, as well as the `HOTT3` library for Lean 3. It represents a continuation of efforts to formalize HoTT in the Lean ecosystem.
