// Yes

#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_uno::prelude::*;
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::hal::port::portb::PB4;
use arduino_uno::hal::port::portb::PB0;
use arduino_uno::hal::port::mode::Output;

fn stutter_blink(led: &mut PB5<Output>, times: usize) {
    (0..times).map(|i| i * 10).for_each(|i| {
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(i as u16);
    });
}

fn normal_blink1(led: &mut PB5<Output>, delay: usize) {
    led.toggle().void_unwrap();
    arduino_uno::delay_ms(delay as u16);
    led.toggle().void_unwrap();
    arduino_uno::delay_ms(delay as u16);
}

fn normal_blink2(led: &mut PB4<Output>, delay: usize) {
    led.toggle().void_unwrap();
    arduino_uno::delay_ms(delay as u16);
    led.toggle().void_unwrap();
    arduino_uno::delay_ms(delay as u16);
}

fn normal_blink3(led: &mut PB0<Output>, delay: usize) {
    led.toggle().void_unwrap();
    arduino_uno::delay_ms(delay as u16);
    led.toggle().void_unwrap();
    arduino_uno::delay_ms(delay as u16);
}


#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(
        peripherals.PORTB,
        peripherals.PORTC,
        peripherals.PORTD,
    );

    let mut led_green = pins.d13.into_output(&mut pins.ddr);
    let mut led_red = pins.d12.into_output(&mut pins.ddr);
    let mut led_yellow = pins.d8.into_output(&mut pins.ddr);

    loop {
        //stutter_blink(&mut led, 25);
        normal_blink1(&mut led_green, 200);
        normal_blink2(&mut led_red, 200);
        normal_blink3(&mut led_yellow, 200);
        stutter_blink(&mut led_green, 26);
    }
}
