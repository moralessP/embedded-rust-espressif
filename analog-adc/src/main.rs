#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{Config, analog::adc::*, delay::Delay};
use log::info;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal::main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    let peripherals = esp_hal::init(Config::default());

    let mut adc1_config = AdcConfig::new();
    let mut pin =
        adc1_config.enable_pin_with_cal::<_, AdcCalBasic<_>>(peripherals.GPIO1, Attenuation::_11dB);
    let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);

    let delay = Delay::new();

    loop {
        if let Ok(value) = adc1.read_oneshot(&mut pin) {
            info!("potentiometer value: {value}");
        }
        delay.delay_millis(250);
    }
}
