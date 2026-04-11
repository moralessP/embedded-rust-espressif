#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;

use esp_backtrace as _;
use esp_hal::{Config, timer::timg::TimerGroup};
use log::info;

esp_bootloader_esp_idf::esp_app_desc!();

#[embassy_executor::task]
async fn hello_world() {
    info!("Starting hello_world task...");

    loop {
        info!("Hello, World!");
        Timer::after_millis(500).await;
    }
}

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();
    let peripherals = esp_hal::init(Config::default());

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    spawner.spawn(hello_world()).ok();

    info!("Starting main task...");
    loop {
        info!("BING!");
        Timer::after_secs(1).await;
    }
}
