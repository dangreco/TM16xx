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

  fn get_state(&self) -> &State {
    &self.0.state
  }
}

impl<D, DIO, CLK, STB> TM16xx16<D, DIO, CLK, STB> for TM1628<D, DIO, CLK, STB>
where
  D: DelayUs<u32>,
  DIO: OutputPin,
  CLK: OutputPin,
  STB: OutputPin,
{
  fn set_segments_16(&mut self, display: u8, data: u16) -> anyhow::Result<()> {
    if display >= Self::MAX_DISPLAYS {
      // DO SOMETHING
    }

    let (a0, a1) = (display << 1, (display << 1) | 1);
    let (b0, b1) = ((data as u8) & 0xFF, ((data >> 8) as u8) & 0x30);

    self.0.send_data(a0, b0)?;
    self.0.send_data(a1, b1)?;

    Ok(())
  }
}
