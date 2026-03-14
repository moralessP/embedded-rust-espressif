#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{Config, delay::Delay, rmt::Rmt, time::Rate};
use esp_hal_smartled::{SmartLedsAdapter, smart_led_buffer};
use smart_leds::{SmartLedsWrite, brightness, gamma, hsv};

esp_bootloader_esp_idf::esp_app_desc!();

const BRIGHTNESS: u8 = 10;

#[esp_hal::main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    let peripherals = esp_hal::init(Config::default());

    let rmt = {
        let rate = Rate::from_mhz(80);
        Rmt::new(peripherals.RMT, rate)
    }
    .expect("Failed to initialize RMT");

    let mut rmt_buffer = smart_led_buffer!(1);
    let mut led = SmartLedsAdapter::new(rmt.channel0, peripherals.GPIO38, &mut rmt_buffer);

    let delay = Delay::new();
    let mut data: smart_leds::RGB8;
    let mut color = hsv::Hsv {
        hue: 0,
        sat: 255,
        val: 255,
    };

    loop {
        for hue in 0..=255 {
            color.hue = hue;
            data = hsv::hsv2rgb(color);

            led.write(brightness(gamma([data].into_iter()), BRIGHTNESS))
                .expect("Failed to write data");

            delay.delay_millis(50);
        }
    }
}
