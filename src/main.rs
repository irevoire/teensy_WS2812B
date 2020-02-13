#![feature(stdsimd)]
#![no_std]
#![no_main]

use teensy::*;

define_panic! {empty}

#[inline]
fn write_byte(pin: &mut port::Gpio, data: u8) {
    let mut bitmask: u8 = 0x80;
    while bitmask != 0 {
        pin.high();
        sleep::delay(3);

        if data & bitmask != 0 {
            sleep::delay(10);
            pin.low();
        } else {
            pin.low();
            sleep::delay(3);
        }
        bitmask >>= 1;
    }
    sleep::delay(8);
}

fn write_color(led: &mut port::Gpio, r: u8, g: u8, b: u8) {
    write_byte(led, g);
    write_byte(led, r);
    write_byte(led, b);
}

#[no_mangle]
fn main() {
    let mut led = unsafe { make_pin!(led).make_gpio().with_output() };

    led.low();
    sleep::sleep_us(100);

    let mut r: u8 = 80;
    let mut g: u8 = 160;
    let mut b: u8 = 240;
    let mut rb: bool = true;
    let mut gb: bool = true;
    let mut bb: bool = true;

    loop {
        write_color(&mut led, r / 64, g / 64, b / 64);
        if rb {
            r += 1;
            if r == 255 {
                rb = false;
            }
        } else {
            r -= 1;
            if r == 0 {
                rb = true;
            }
        }

        if gb {
            g += 1;
            if g == 255 {
                gb = false;
            }
        } else {
            g -= 1;
            if g == 0 {
                gb = true;
            }
        }

        if bb {
            b += 1;
            if b == 255 {
                bb = false;
            }
        } else {
            b -= 1;
            if b == 0 {
                bb = true;
            }
        }

        sleep::sleep_ms(20);
    }
}
