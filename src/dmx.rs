use esp_idf_hal::delay::{FreeRtos, BLOCK};
use esp_idf_hal::gpio::{AnyIOPin, Gpio3, Output, OutputPin, Pin, PinDriver};
use esp_idf_hal::io::{EspIOError, Read, ReadExactError};
use esp_idf_hal::prelude::*;
use esp_idf_hal::sys::EspError;
use esp_idf_hal::uart::config::Config;
use esp_idf_hal::uart::UartDriver;

const DMX_PACKET_SIZE: usize = 513;

pub struct DmxReader<'d> {
    uart: UartDriver<'d>,
    enable_pin_driver: PinDriver<'d, Gpio3, Output>,
    data: [u8; DMX_PACKET_SIZE],
    data_ready: bool,
}

#[derive(Debug)]
pub enum DmxError {
    EspError(EspError),
    ReadError(ReadExactError<EspIOError>),
}

impl From<EspError> for DmxError {
    fn from(error: EspError) -> Self {
        DmxError::EspError(error)
    }
}

impl From<ReadExactError<EspIOError>> for DmxError {
    fn from(error: ReadExactError<EspIOError>) -> Self {
        DmxError::ReadError(error)
    }
}
#[allow(dead_code)]
impl<'d> DmxReader<'d> {
    pub fn new() -> Result<Self, EspError> {
        let peripherals = Peripherals::take()?;

        // This pin, labelled 485/EN on the ESP32 Thing Plus DMX to LED Shield schematic, is used
        // to set read mode (low), or write mode (high).
        let enable_pin = peripherals.pins.gpio3;
        let mut enable_pin_driver = PinDriver::output(enable_pin)?;
        enable_pin_driver.set_low()?;
        FreeRtos::delay_ms(500);

        // Configure the UART for the DMX protocol
        let tx_pin = peripherals.pins.gpio34;
        let rx_pin = peripherals.pins.gpio33;
        let uart_config = Config::default()
            .baudrate(250000.Hz())
            .data_bits(esp_idf_hal::uart::config::DataBits::DataBits8)
            .parity_none()
            .stop_bits(esp_idf_hal::uart::config::StopBits::STOP2)
            .flow_control(esp_idf_hal::uart::config::FlowControl::None)
            .source_clock(esp_idf_hal::uart::config::SourceClock::APB);
        let uart = UartDriver::new(
            peripherals.uart1,
            tx_pin,
            rx_pin,
            Option::<AnyIOPin>::None,
            Option::<AnyIOPin>::None,
            &uart_config,
        )?;

        Ok(Self {
            uart,
            enable_pin_driver,
            data: [0; DMX_PACKET_SIZE],
            data_ready: false,
        })
    }

    pub fn update(&mut self) -> Result<(), DmxError> {
        self.uart.read_exact(&mut self.data)?;
        println!("{:?}", self.data);
        self.data_ready = self.data[0] == 0x00;
        Ok(())
    }

    pub fn channel_value(&self, channel: u16) -> Option<u8> {
        if self.data_ready && (1..=512).contains(&channel) {
            Some(self.data[channel as usize])
        } else {
            None
        }
    }
}
