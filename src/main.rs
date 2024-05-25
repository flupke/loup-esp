use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver, Resolution};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::sys::EspError;

fn main() {
    if let Err(error) = run_app() {
        log::error!("App failed: {}", error);
    }
}

#[allow(unreachable_code)]
fn run_app() -> Result<(), EspError> {
    let mut driver = init()?;

    let max_duty = driver.get_max_duty();
    let min_limit = max_duty * 25 / 1000;
    let max_limit = max_duty * 125 / 1000;

    driver.set_duty(map(0, 0, 180, min_limit, max_limit))?;
    FreeRtos::delay_ms(500);

    loop {
        // Sweep from 0 degrees to 180 degrees
        for angle in 0..180 {
            // Print Current Angle for visual verification
            println!("Current Angle {} Degrees", angle);
            // Set the desired duty cycle
            driver
                .set_duty(map(angle, 0, 180, min_limit, max_limit))
                .unwrap();
            // Give servo some time to update
            FreeRtos::delay_ms(12);
        }

        // Sweep from 180 degrees to 0 degrees
        for angle in (0..180).rev() {
            // Print Current Angle for visual verification
            println!("Current Angle {} Degrees", angle);
            // Set the desired duty cycle
            driver
                .set_duty(map(angle, 0, 180, min_limit, max_limit))
                .unwrap();
            // Give servo some time to update
            FreeRtos::delay_ms(12);
        }
    }

    Ok(())
}

fn init() -> Result<LedcDriver<'static>, EspError> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Configuring servo control on GPIO 23");

    // Configure timer to operate the Servo
    let peripherals = Peripherals::take()?;
    let timer_driver = LedcTimerDriver::new(
        peripherals.ledc.timer0,
        &TimerConfig::default()
            .frequency(50.Hz())
            .resolution(Resolution::Bits14),
    )?;
    let driver = LedcDriver::new(
        peripherals.ledc.channel0,
        timer_driver,
        peripherals.pins.gpio32,
    )?;

    log::info!("Init done");

    Ok(driver)
}

fn map(x: u32, in_min: u32, in_max: u32, out_min: u32, out_max: u32) -> u32 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
