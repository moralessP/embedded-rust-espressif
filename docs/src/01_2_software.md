# Toolchain Installation

# Xtensa Devices

The ESP32-S3 is based on the Xtensa architecture. You will need to use a fork of the Rust compiler.

[espup](https://github.com/esp-rs/espup) is a project that simplifies installing the toolchain required to develop Rust applications for Espressif targets.

1. Install `espup`

    ```sh
    cargo install espup --locked
    ```

2. Install all necessary toolchains for all supported Espressif targets

    ```sh
    espup install
    ```

# Tooling

To simplify the development of Rust applications for Espressif devices, you should install the following tools.

1. `esp-generate` a tool that generates Rust projects for Espressif chips based on a generic template.

    ```sh
    cargo install esp-generate --locked
    ```

2. `espflash` a serial flasher utility designed for Espressif SoCs.

    ```sh
    cargo install espflash --locked
    ```
For more information about toolchain installation and tooling, refer to the official documentation [The Rust on ESP Book](https://docs.espressif.com/projects/rust/book/).
