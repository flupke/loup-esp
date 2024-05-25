use esp_idf_hal::ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver, Resolution};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::sys::EspError;

#[allow(dead_code)]
pub enum DataPort {
    D0,
    D1,
    D2,
}

pub struct Servo {
    driver: LedcDriver<'static>,
    min_limit: u32,
    max_limit: u32,
}

impl Servo {
    pub fn new(port: DataPort) -> Result<Self, EspError> {
        let peripherals = Peripherals::take()?;
        let timer_driver = LedcTimerDriver::new(
            peripherals.ledc.timer0,
            &TimerConfig::default()
                .frequency(50.Hz())
                .resolution(Resolution::Bits14),
        )?;
        let driver = match port {
            DataPort::D0 => LedcDriver::new(
                peripherals.ledc.channel0,
                timer_driver,
                peripherals.pins.gpio37,
            )?,
            DataPort::D1 => LedcDriver::new(
                peripherals.ledc.channel1,
                timer_driver,
                peripherals.pins.gpio35,
            )?,
            DataPort::D2 => LedcDriver::new(
                peripherals.ledc.channel2,
                timer_driver,
                peripherals.pins.gpio11,
            )?,
        };
        let max_duty = driver.get_max_duty();
        let min_limit = max_duty * 25 / 1000;
        let max_limit = max_duty * 125 / 1000;
        Ok(Self {
            driver,
            min_limit,
            max_limit,
        })
    }

    pub fn set_angle(&mut self, angle: u32) -> Result<(), EspError> {
        self.driver
            .set_duty(map(angle, 0, 180, self.min_limit, self.max_limit))
    }
}

fn map(x: u32, in_min: u32, in_max: u32, out_min: u32, out_max: u32) -> u32 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
