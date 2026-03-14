#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Ticker};
use esp_backtrace as _;
use esp_hal::{Config, timer::timg::TimerGroup};
use log::*;

esp_bootloader_esp_idf::esp_app_desc!();

#[embassy_executor::task]
async fn hello_world_info() {
    let mut ticker = Ticker::every(Duration::from_secs(1));
    loop {
        info!("Hello, World!");
        ticker.next().await;
    }
}

#[embassy_executor::task]
async fn hello_world_warn() {
    let mut ticker = Ticker::every(Duration::from_millis(500));
    loop {
        warn!("Hello, World!");
        ticker.next().await;
    }
}

#[embassy_executor::task]
async fn hello_world_error() {
    let mut ticker = Ticker::every(Duration::from_millis(250));
    loop {
        error!("Hello, World!");
        ticker.next().await;
    }
}

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();
    let peripherals = esp_hal::init(Config::default());

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    info!("Starting tasks...");
    spawner.spawn(hello_world_info()).ok();
    spawner.spawn(hello_world_warn()).ok();
    spawner.spawn(hello_world_error()).ok();

    core::future::pending::<()>().await;
}
