#![no_std]
#![no_main]

extern crate embedded_hal as hal;

use core::panic::PanicInfo;
use hal::prelude::*;
use hal::Pwm;

#[panic_handler]
fn panic(_info: & PanicInfo) -> ! {
    loop {

    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Important because this sets the bit in the DDR register!
    pins.d9.into_output();
    // - TC1 runs off a 250kHz clock, with 5000 counts per overflow => 50 Hz signal.
    // - Each count increases the duty-cycle by 4us.
    // - Use OC1A which is connected to D9 of the Arduino Uno.
    let tc1 = dp.TC1;
    let mut pwm: Pwm = {};

    // tc1.icr1.write(|w| unsafe { w.bits(4999) });
    // tc1.tccr1a
    //     .write(|w| w.wgm1().bits(0b10).com1a().match_clear());
    // tc1.tccr1b
    //     .write(|w| w.wgm1().bits(0b11).cs1().prescale_64());

    loop {
        // 100 counts => 0.4ms
        // 700 counts => 2.8ms
        pwm.set_duty((), 15 as u8);
        arduino_hal::delay_ms(20);
        pwm.set_duty((), 24 as u8);
        arduino_hal::delay_ms(20);
        pwm.set_duty((), 31 as u8);
        arduino_hal::delay_ms(20);
    }
}