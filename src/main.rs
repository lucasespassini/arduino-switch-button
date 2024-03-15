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

    led.set_low();
    led13.set_low();
    led0.set_high();
    led1.set_high();

    loop {
        if button.is_high() && !pressionado {
            pressionado = true;
            ligado = !ligado;
        }

        if button.is_low() && pressionado {
            pressionado = false;
            led.set_low();
            led13.set_low();
            led0.set_high();
            led1.set_high();
        }

        if ligado && !pressionado {
            led.toggle();
            arduino_hal::delay_ms(200);
            led.toggle();
            arduino_hal::delay_ms(200);

            led1.toggle();
            arduino_hal::delay_ms(200);
            led1.toggle();
            arduino_hal::delay_ms(200);

            led0.toggle();
            arduino_hal::delay_ms(200);
            led0.toggle();
            arduino_hal::delay_ms(200);

            led13.toggle();
            arduino_hal::delay_ms(200);
            led13.toggle();
            arduino_hal::delay_ms(200);
        }
    }
}
