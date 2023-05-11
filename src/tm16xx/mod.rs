use anyhow::Result;
use hal::{blocking::delay::DelayUs, digital::v2::OutputPin};

pub mod defaults;
mod driver;
mod state;

pub use driver::TM16xxDriver;
pub use state::State;

const TM16XX_CMD_DATA_AUTO: u8 = 0x40;
const TM16XX_CMD_DATA_FIXED: u8 = 0x44;
const TM16XX_CMD_DISPLAY: u8 = 0x80;
const TM16XX_CMD_ADDRESS: u8 = 0xC0;

pub trait TM16xx<D, DIO, CLK, STB>
where
  Self: Sized,
  D: DelayUs<u32>,
  DIO: OutputPin,
  CLK: OutputPin,
  STB: OutputPin,
{
  const BIT_DELAY_US: u32;
  const MAX_DISPLAYS: u8;
  const MAX_SEGMENTS: u8;

  fn setup(&mut self, active: bool, intensity: u8) -> Result<()>;
  fn clear(&mut self) -> Result<()>;
  fn set_segments(&mut self, display: u8, data: u8) -> Result<()>;
}

pub trait TM16xx16<D, DIO, CLK, STB>
where
  Self: Sized,
  D: DelayUs<u32>,
  DIO: OutputPin,
  CLK: OutputPin,
  STB: OutputPin,
{
  fn set_segments_16(&mut self, display: u8, data: u16) -> Result<()>;
}

#[derive(Debug)]
pub enum Error {
  PinError,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::PinError => write!(f, ""),
    }
  }
}
