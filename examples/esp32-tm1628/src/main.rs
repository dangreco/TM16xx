extern crate esp_idf_hal as hal;

use anyhow::*;
use esp_idf_sys as _;
use hal::{delay, gpio::*, peripherals::Peripherals};
use tm16xx::{models::TM1628, tm16xx::*};

const FONT: &[u8; 16] = &[
  0b00111111, // 0
  0b00000110, // 1
  0b01011011, // 2
  0b01001111, // 3
  0b01100110, // 4
  0b01101101, // 5
  0b01111100, // 6
  0b00000111, // 7
  0b01111111, // 8
  0b01101111, // 9
  0b01110111, // A
  0b01111100, // B
  0b00111001, // C
  0b01011110, // D
  0b01111001, // E
  0b01110001, // F
];

fn main() -> Result<()> {
  esp_idf_sys::link_patches();

  let peripherals = Peripherals::take().unwrap();
  let pins = peripherals.pins;

  let dio = PinDriver::output(pins.gpio16)?;
  let clk = PinDriver::output(pins.gpio17)?;
  let stb = PinDriver::output(pins.gpio18)?;

  let mut display = TM1628::new(delay::Ets, dio, clk, stb);
  display.setup(true, 7)?;
  display.clear()?;

  let mut counter: u32 = 0;

  loop {
    // Check for overflow
    if counter > 0xFFFF {
      counter = 0;
    }

    // Display counter
    let digits: [u8; 4] = [
      ((counter >> 12) & 0xF) as u8,
      ((counter >> 8) & 0xF) as u8,
      ((counter >> 4) & 0xF) as u8,
      ((counter >> 0) & 0xF) as u8,
    ];

    display.set_segments(0, FONT[digits[0] as usize])?;
    display.set_segments(1, FONT[digits[1] as usize])?;
    display.set_segments(2, FONT[digits[2] as usize])?;
    display.set_segments(3, FONT[digits[3] as usize])?;

    // Increment counter
    counter += 1;

    // Sleep
    delay::FreeRtos::delay_ms(50);
  }
}
