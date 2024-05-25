mod dmx;
mod servo;
mod utils;

use dmx::DmxReader;
use esp_idf_hal::{delay::FreeRtos, sys::EspError};
use servo::{DataPort, Servo};

fn main() {
    if let Err(error) = run_app() {
        log::error!("App failed: {}", error);
    }
}

fn run_app() -> Result<(), EspError> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    let mut dmx = DmxReader::new()?;

    loop {
        match dmx.update() {
            Ok(_) => {
                println!("{:?}", dmx.channel_value(1));
            }
            Err(error) => log::error!("Error reading DMX data: {:?}", error),
        }
        FreeRtos::delay_ms(500);
    }

    /*
        let mut servo = Servo::new(DataPort::D0)?;

        servo.set_angle(0)?;
        FreeRtos::delay_ms(500);

        loop {
            for angle in 0..180 {
                println!("Current Angle {} Degrees", angle);
                servo.set_angle(angle)?;
                FreeRtos::delay_ms(12);
            }

            for angle in (0..180).rev() {
                println!("Current Angle {} Degrees", angle);
                servo.set_angle(angle)?;
                FreeRtos::delay_ms(12);
            }
        }
    */
}
