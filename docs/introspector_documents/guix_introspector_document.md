# GNU Guix Introspector Document

## Overview

GNU Guix is a purely functional package manager and a free software distribution for the GNU system. It offers advanced features such as transactional upgrades and roll-backs, unprivileged package management, per-user profiles, and garbage collection. Guix distinguishes itself by using Guile Scheme as its primary language for defining packages and system configurations, providing a powerful and flexible environment for reproducible software management.

## Key Features and Purpose

*   **Functional Package Management**: Ensures reproducible builds and deployments through a purely functional approach.
*   **Transactional Upgrades and Roll-backs**: Allows for safe system updates and easy reversion to previous states.
*   **Unprivileged Package Management**: Enables users to install software without requiring root privileges.
*   **Guile Scheme APIs**: Utilizes Guile Scheme for defining package recipes and system configurations, offering a high-level, extensible, and programmable interface.
*   **Based on Nix**: Builds upon the foundational ideas of the Nix package manager, while making different engineering decisions, particularly in its use of Scheme.
*   **Reproducible Builds**: Aims to provide a fully transparent and verifiable software stack, contributing to the bootstrappable builds effort.

## Usage

GNU Guix can be used as a package manager on top of an existing GNU/Linux distribution or as a standalone operating system (Guix System). It is primarily used by developers and users who prioritize software freedom, reproducibility, and fine-grained control over their system.

## Project Structure (High-Level)

The repository contains Scheme source files (`.scm`) for package definitions and system configurations, along with C++ code (reused from Nix), documentation (`doc/`), build scripts (`build-aux/`), and various configuration files. Notable directories include `gnu/` (for core Guix definitions) and `guix/` (for Guix-specific modules).

## Mentors and Influences

GNU Guix is heavily influenced by the Nix package manager and the broader free software movement. Its design reflects a commitment to transparency, reproducibility, and user control, aligning with the principles of the GNU Project.