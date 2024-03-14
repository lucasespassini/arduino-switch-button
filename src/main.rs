#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);

    let mut led = pins.d2.into_output();
    let mut led0 = pins.d0.into_output();
    let mut led1 = pins.d1.into_output();
    let mut led13 = pins.d13.into_output();
    let button = pins.d3.into_pull_up_input();

    let mut ligado = false;
    let mut pressionado = false;

    loop {
        if button.is_high() && pressionado == false {
            pressionado = true;
            if ligado == false {
                ligado = true;
            } else {
                ligado = false;
            }
        }

        if button.is_low() && pressionado == true {
            pressionado = false;
            led.set_low();
        }

        if ligado == true {
            led.toggle();
            arduino_hal::delay_ms(200);
            led13.toggle();
            arduino_hal::delay_ms(200);
            led0.toggle();
            arduino_hal::delay_ms(200);
            led1.toggle();
            arduino_hal::delay_ms(200);
        }
    }
}
