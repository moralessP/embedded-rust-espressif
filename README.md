# Embedded Rust on Espressif Chips

`embedded-rust-espressif` is a collection of Embedded Rust example projects for Espressif Chips.

## Prerequisites

Before getting started make sure Rust and the required tools for programming Espressif chips are proprely installed.

Refer to the [Software Installation Guide](Documentation/src/01_2_software.md) instructions.

## Repository structure

This repository is organized as follows:

 - `Documentation` - Documentations for each project
 - `.cargo` - Shared Configuration (Rust flags, environment variables...)
 - `Cargo.toml` - Workspace configuration file
 - `rust-toolchain.toml` - Toolchain configuration

## Installation & Usage

### 1. Clone repository

```bash
git clone https://github.com/moralessP/embedded-rust-espressif
cd embedded-rust-espressif
```

### 2. Install `just`

This poject uses [just](https://github.com/casey/just) as a command runner to simplify building and flashing firmware.


```bash
cargo install just
```

### 3. Build a project


```bash
just build-esp32s3 <pkg>
```

### 4. Flash a firmware

```bash
just flash-esp32s3 <pkg>
```
