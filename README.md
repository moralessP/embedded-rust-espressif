# Embedded Rust on Espressif Chips

`embedded-rust-espressif` is a collection of Embedded Rust example projects for Espressif Chips.

## Prerequisites

Before getting started make sure Rust and the required tools for programming Espressif chips are proprely installed.

Refer to the [Software Installation Guide](docs/src/01_2_software.md) instructions.

## Repository structure

This repository is organized as follows:

 - `docs/` - Documentations for each project
 - `scr/` - Source code for each project
 - `.cargo/` - Shared Configuration (Rust flags, environment variables...)
 - `Cargo.toml` - Workspace configuration file
 - `rust-toolchain.toml` - Toolchain configuration

## Installation & Usage

### 1. Clone repository

```bash
git clone https://github.com/moralessP/embedded-rust-espressif
cd embedded-rust-espressif
```

### 2. Build the projects

To build all projects in the Workspace

```bash
cargo build --release
```

To build  a specific project only

```
cargo build --release -p <project_name>
```

### 3. Flash and Run a Project

Make sure your board is connected to your PC via USB before flashing.

```bash
cargo run --release -p <project_name>
```
