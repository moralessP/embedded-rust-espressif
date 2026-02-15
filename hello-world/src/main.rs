#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{Config, delay::Delay};
use log::{debug, info, warn, error};

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal::main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    let _peripherals = esp_hal::init(Config::default());
    let delay = Delay::new();

    loop {
        debug!("Hello, World!");
        info!("Hello, World!");
        warn!("Hello, World!");
        error!("Hello, World!");
        delay.delay_millis(500);
    }
}
