#![no_std]
#![no_main]
#![feature(generic_const_exprs)]

use attiny_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use panic_halt as _;
use smart_leds::hsv::{Hsv, hsv2rgb};
use ws2812_avr::{GRB, WS2812};

const LED_COUNT: usize = 48;

#[attiny_hal::entry]
fn main() -> ! {
    let dp = attiny_hal::Peripherals::take().unwrap();
    let pins = attiny_hal::pins!(dp);

    let led_pin = pins.pb0.into_output();
    let mut driver = WS2812::new(led_pin);

    let reed_switch = pins.pb2.into_pull_up_input();

    let mut buf: [GRB; LED_COUNT] = [GRB::default(); LED_COUNT];
    let mut delay = attiny_hal::delay::Delay::<attiny_hal::clock::MHz16>::new();

    let mut cur_offset = 0;
    let mut hsv = Hsv {
        hue: 0,
        sat: 255,
        val: 255,
    };

    loop {
        if reed_switch.is_low() {
            let off_buf: [GRB; LED_COUNT] = [GRB::default(); LED_COUNT];
            driver.write(&off_buf);
        } else {
            let rgb = hsv2rgb(hsv);
            buf[cur_offset] = GRB {
                g: rgb.g,
                r: rgb.r,
                b: rgb.b,
            };
            driver.write(&buf);
            cur_offset = (cur_offset + 1) % LED_COUNT;
            hsv.hue += 1;
        }
        
        delay.delay_ms(25u16);
    }
}
