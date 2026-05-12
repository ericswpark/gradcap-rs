#![no_std]
#![no_main]
#![feature(generic_const_exprs)]

use attiny_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use panic_halt as _;
use ws2812_avr::{GRB, WS2812};

const LED_COUNT: usize = 48;

#[attiny_hal::entry]
fn main() -> ! {
    let dp = attiny_hal::Peripherals::take().unwrap();
    let pins = attiny_hal::pins!(dp);

    let led_pin = pins.pb0.into_output();
    let mut driver = WS2812::new(led_pin);

    let reed_switch = pins.pb2.into_pull_up_input();

    let off_buf: [GRB; LED_COUNT] = [GRB::default(); LED_COUNT];
    let on_buf: [GRB; LED_COUNT] = [GRB { g: u8::MAX, r: u8::MAX, b: u8::MAX }; LED_COUNT];
    let mut next_off = false;
    let mut delay = attiny_hal::delay::Delay::<attiny_hal::clock::MHz16>::new();

    loop {
        if reed_switch.is_low() {
            driver.write(&off_buf);
        } else {
            if next_off {
                driver.write(&off_buf);
            } else {
                driver.write(&on_buf);
            }
            next_off = !next_off;
        }
        
        delay.delay_ms(25u16);
    }
}
