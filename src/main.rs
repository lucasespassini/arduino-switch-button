#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);

    let mut led = pins.d0.into_output();
    let button = pins.d1.into_pull_up_input();

    loop {
        if button.is_high() {
            led.set_high();

            if button.is_low() {
                for _i in 0..3 {
                    led.set_low();
                    arduino_hal::delay_ms(200);
                    led.set_high();
                    arduino_hal::delay_ms(200);
                }
            }
        } else {
            led.set_low();
        }
    }
}
