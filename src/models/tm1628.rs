use hal::{blocking::delay::DelayUs, digital::v2::OutputPin};

use crate::tm16xx::*;

pub struct TM1628<D, DIO, CLK, STB>(TM16xxDriver<Self, D, DIO, CLK, STB>)
where
  D: DelayUs<u32>,
  DIO: OutputPin,
  CLK: OutputPin,
  STB: OutputPin;

impl<D, DIO, CLK, STB> TM1628<D, DIO, CLK, STB>
where
  D: DelayUs<u32>,
  DIO: OutputPin,
  CLK: OutputPin,
  STB: OutputPin,
{
  pub fn new(delay: D, dio: DIO, clk: CLK, stb: STB) -> Self {
    Self(TM16xxDriver::new(delay, dio, clk, stb))
  }
}

impl<D, DIO, CLK, STB> TM16xx<D, DIO, CLK, STB> for TM1628<D, DIO, CLK, STB>
where
  D: DelayUs<u32>,
  DIO: OutputPin,
  CLK: OutputPin,
  STB: OutputPin,
{
  const BIT_DELAY_US: u32 = 5;
  const MAX_DISPLAYS: u8 = 7;
  const MAX_SEGMENTS: u8 = 13;

  fn setup(&mut self, active: bool, intensity: u8) -> anyhow::Result<()> {
    defaults::setup(&mut self.0, active, intensity)
  }

  fn clear(&mut self) -> anyhow::Result<()> {
    defaults::clear(&mut self.0)
  }

  fn set_segments(&mut self, display: u8, data: u8) -> anyhow::Result<()> {
    defaults::set_segments(&mut self.0, display << 1, data)
  }
}
