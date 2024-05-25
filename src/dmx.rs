use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::gpio::{AnyIOPin, PinDriver};
use esp_idf_hal::io::{EspIOError, ReadExactError};
use esp_idf_hal::prelude::*;
use esp_idf_hal::sys::EspError;
use esp_idf_hal::uart::config::Config;
use esp_idf_hal::uart::UartDriver;

const DMX_PACKET_SIZE: usize = 513;

pub struct DmxReader<'d> {
    uart: UartDriver<'d>,
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

impl<'d> DmxReader<'d> {
    pub fn new() -> Result<Self, EspError> {
        let peripherals = Peripherals::take()?;

        // This pin, labelled 485/EN on the ESP32 Thing Plus DMX to LED Shield schematic, is used
        // to set read mode (low), or write mode (high).
        let enable_pin = peripherals.pins.gpio3;
        PinDriver::output(enable_pin)?.set_low()?;

        // Configure the UART for the DMX protocol
        let tx_pin = peripherals.pins.gpio34;
        let rx_pin = peripherals.pins.gpio33;
        let uart_config = Config::default()
            .baudrate(9600.Hz())
            .data_bits(esp_idf_hal::uart::config::DataBits::DataBits8)
            .parity_none()
            .stop_bits(esp_idf_hal::uart::config::StopBits::STOP1);
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
            data: [0; DMX_PACKET_SIZE],
            data_ready: false,
        })
    }

    pub fn update(&mut self) -> Result<(), DmxError> {
        self.uart.read(&mut self.data, 100)?;
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
