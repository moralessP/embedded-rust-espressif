# no_std Rust Environment

The `no_std` model is used to develop **bare metal** applications.
Bare metal means that the application runs **directly on the MCU**, without any operating system.

In Rust, it is normally possible to use the standard library (`std`), which provides
higher-level features and abstractions. However, in embedded systems, these features
are often disabled to reduce memory usage and improve firmware performance.

To support `no_std` development on ESP32 devices, Espressif provides several dedicated crates.

- `esp-hal`: ESP32 Hardware Abstraction Layer (HAL)
- `esp-backtrace`: panic handling and backtraces
- `esp-println`: serial output for debugging

Throughout the examples in this book, we will use additional crates that are part of
the `esp-hal` ecosystem.

Please refer to the official [esp-hal](https://github.com/esp-rs/esp-hal) documentation for more details.
