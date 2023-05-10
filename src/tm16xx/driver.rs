use anyhow::{anyhow, Result};
use hal::{blocking::delay::DelayUs, digital::v2::OutputPin};
use std::marker::PhantomData;

use super::{state::State, Error, TM16xx};

pub struct TM16xxDriver<T, D, DIO, CLK, STB>
where
  T: TM16xx<D, DIO, CLK, STB>,
  D: DelayUs<u32>,
  DIO: OutputPin,
  CLK: OutputPin,
  STB: OutputPin,
{
  _p: PhantomData<T>,
  delay: D,
  dio: DIO,
  clk: CLK,
  stb: STB,
  pub state: State,
}

impl<T, D, DIO, CLK, STB> TM16xxDriver<T, D, DIO, CLK, STB>
where
  T: TM16xx<D, DIO, CLK, STB>,
  D: DelayUs<u32>,
  DIO: OutputPin,
  CLK: OutputPin,
  STB: OutputPin,
{
  pub fn new(delay: D, dio: DIO, clk: CLK, stb: STB) -> Self {
    TM16xxDriver {
      _p: PhantomData,
      delay,
      dio,
      clk,
      stb,
      state: State::new(T::MAX_DISPLAYS as usize),
    }
  }

  fn bit_delay(&mut self) {
    self.delay.delay_us(T::BIT_DELAY_US)
  }

  pub fn start(&mut self) -> Result<()> {
    self.stb.set_low().map_err(|_| anyhow!(Error::PinError))?;
    self.bit_delay();
    Ok(())
  }

  pub fn stop(&mut self) -> Result<()> {
    self.stb.set_high().map_err(|_| anyhow!(Error::PinError))?;
    self.bit_delay();
    Ok(())
  }

  pub fn send(&mut self, data: u8) -> Result<()> {
    let mut byte = data;

    for _ in 0..8 {
      self.clk.set_low().map_err(|_| anyhow!(Error::PinError))?;
      self.bit_delay();
      if byte & 1 == 1 {
        self.dio.set_high().map_err(|_| anyhow!(Error::PinError))?;
      } else {
        self.dio.set_low().map_err(|_| anyhow!(Error::PinError))?;
      }
      self.bit_delay();
      byte >>= 1;
      self.clk.set_high().map_err(|_| anyhow!(Error::PinError))?;
      self.bit_delay();
    }

    self.bit_delay();

    Ok(())
  }

  pub fn send_command(&mut self, cmd: u8) -> Result<()> {
    self.start()?;
    self.send(cmd)?;
    self.stop()
  }

  pub fn send_data(&mut self, address: u8, data: u8) -> Result<()> {
    self.send_command(super::TM16XX_CMD_DATA_FIXED)?;
    self.start()?;
    self.send(super::TM16XX_CMD_ADDRESS | address)?;
    self.send(data)?;
    self.stop()
  }

  pub fn clear(&mut self) -> Result<()> {
    self.send_command(super::TM16XX_CMD_DATA_AUTO)?;
    self.start()?;
    self.send_command(super::TM16XX_CMD_ADDRESS)?;

    for i in 0..T::MAX_DISPLAYS {
      self.send(0x00)?;
      if T::MAX_SEGMENTS > 8 {
        self.send(0x00)?;
      }
      self.state.displays[i as usize] = 0x00;
    }

    self.stop()
  }
}
