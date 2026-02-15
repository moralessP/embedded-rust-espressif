# Hello World

In this first program, the goal is to print “Hello, World!” to the serial monitor using different log levels.
This example introduces the basic structure of a `no_std` embedded Rust application on Espressif chips.

## Crates
### esp-hal

`esp-hal` allows board configuration and initialization. It also provides the `main` entry point required for `no_std` applications.

In a `no_std` environment, Rust does not provide the standard runtime. Therefore, the hardware abstraction layer (HAL) is responsible for low-level system initialization.

The delay functionality provided by `esp-hal` is used to pause execution between log messages. This makes it possible to print “Hello, World!” with different log levels separated by 500 ms.

```rust
use esp_hal::{Config, delay::Delay};
```
### esp-backtrace

`esp-backtrace` provides panic handling and prints useful debugging information if the program crashes.

```rust
use esp_backtrace as _;
```
### esp-bootloader-esp-idf

Espressif provides a ready-to-use bootloader through the `esp-bootloader-esp-idf` crate.
This ensures that the firmware starts and runs properly on the board.

```rust
esp_bootloader_esp_idf::esp_app_desc!();
```
### log and esp-println

`log` and `esp-println` crates are usefull to print messages with different log levels:
- The `log` crate for log macros like `debug!`, `info!`, `warn!`, `error!`
- The `esp-println` crate as our logger implementation

```rust
use log::{debug, info, warn, error};
```
## Code structure
### Logger initialization

The first step in `main` is logger initialization.

```rust
esp_println::logger::init_logger_from_env();
```

This is important because any function called afterward can use logging macros.
A maximum log level must be defined for the logger.

This value is configured in `.cargo/config.toml`.

```toml
# emmbedded-rust-espressif/.cargo/config.toml

[env]
ESP_LOG = "debug"
```
This configuration means that all log messages with a level equal to or higher than `debug` will be displayed.

### Peripheral Initialization

Next, the board peripherals are initialized.
In this example, no specific peripheral is actively used, so the default configuration provided by `esp-hal` is sufficient.

```rust
let _peripherals = esp_hal::init(Config::default());
```

### Delay initialization

To use delays, a `Delay` instance must be initialized.

```rust
let delay = Delay::new();
```
The delay function blocks execution for a specified duration.
Blocking delays can be problematic in more complex systems because they pause all execution during that time.
For the moment it is not an issue since no other tasks are running concurrently.

### Main loop

Inside the loop, “Hello, World!” is printed using different log levels.
A delay of 500 ms is then applied before repeating.

```rust
loop {
    debug!("Hello, World!");
    info!("Hello, World!");
    warn!("Hello, World!");
    error!("Hello, World!");
    delay.delay_millis(500);
}
```

## Monitor output

The serial monitor should display the “Hello, World!” message repeatedly with different log levels, separated by 500 ms.

```bash
DEBUG - Hello, World!
INFO - Hello, World!
WARN - Hello, World!
ERROR - Hello, World!
```
