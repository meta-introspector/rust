# GNU Mes Introspector Document

## Overview

GNU Mes is a foundational project in the realm of bootstrappable builds, serving as a Scheme interpreter and a C compiler. Its primary goal is to reduce the size of opaque, uninspectable binary seeds used in bootstrapping the GNU System, contributing to the broader effort of achieving full source bootstraps for UNIX-like operating systems.

## Key Features and Purpose

*   **Scheme Interpreter**: Implemented in approximately 5,000 lines of C code, providing a minimal yet functional Scheme environment.
*   **Self-Hosting C Compiler (mescc)**: Written in Scheme, allowing for mutual self-hosting capabilities.
*   **Reduced Binary Seed Bootstrap**: Significantly contributes to minimizing the trusted computing base by reducing the size of necessary binary seeds.
*   **Garbage Collector**: Includes a garbage collector for efficient memory management.
*   **Loadable Scheme Modules**: Supports a library of loadable Scheme modules, including LALR and portable syntax-case.
*   **Bootstrappable TinyCC**: Mes and MesCC can build a self-hosting TinyCC, further reducing reliance on pre-compiled binaries.
*   **Inspiration**: Inspired by foundational concepts like LISP-1.5 and the principles of source/binary packaging transparency from GNU Guix.

## Usage

GNU Mes is a critical component in the bootstrappable builds ecosystem, primarily used by projects like GNU Guix to achieve verifiable and transparent software distribution. It can be built from source and provides a REPL and a C99 compiler.

## Project Structure (High-Level)

The repository contains source code for the Scheme interpreter and C compiler, along with build scripts, documentation (including `README` and `INSTALL` files), and various utilities. Key directories include `mes/`, `src/`, `lib/`, and `tests/`.

## Mentors and Influences

GNU Mes is deeply influenced by the philosophy of free software, the LISP programming language, and the bootstrappable builds movement. It is a testament to the idea of building computing systems from minimal, verifiable foundations.
