use anyhow::Result;
use hal::{blocking::delay::DelayUs, digital::v2::OutputPin};

use super::{TM16xx, TM16xxDriver};

pub fn setup<T, D, DIO, CLK, STB>(
  driver: &mut TM16xxDriver<T, D, DIO, CLK, STB>,
  active: bool,
  intensity: u8,
) -> Result<()>
where
  T: TM16xx<D, DIO, CLK, STB>,
  D: DelayUs<u32>,
  DIO: OutputPin,
  CLK: OutputPin,
  STB: OutputPin,
{
  let active = if active { 8 } else { 0 };
  let intensity = intensity.min(7);

  driver.send_command(super::TM16XX_CMD_DISPLAY | active | intensity)?;
  driver.state.intensity = intensity;
  driver.state.active = active > 0;

  Ok(())
}

pub fn clear<T, D, DIO, CLK, STB>(driver: &mut TM16xxDriver<T, D, DIO, CLK, STB>) -> Result<()>
where
  T: TM16xx<D, DIO, CLK, STB>,
  D: DelayUs<u32>,
  DIO: OutputPin,
  CLK: OutputPin,
  STB: OutputPin,
{
  driver.send_command(super::TM16XX_CMD_DATA_AUTO)?;
  driver.start()?;
  driver.send(super::TM16XX_CMD_ADDRESS)?;

  for i in 0..(T::MAX_DISPLAYS as usize) {
    driver.send(0x00)?;
    if T::MAX_SEGMENTS > 8 {
      driver.send(0x00)?;
    }
    driver.state.displays[i] = 0x00;
  }

  driver.stop()
}

pub fn set_segments<T, D, DIO, CLK, STB>(
  driver: &mut TM16xxDriver<T, D, DIO, CLK, STB>,
  display: u8,
  data: u8,
) -> Result<()>
where
  T: TM16xx<D, DIO, CLK, STB>,
  D: DelayUs<u32>,
  DIO: OutputPin,
  CLK: OutputPin,
  STB: OutputPin,
{
  if display >= T::MAX_DISPLAYS {
    // DO SOMETHING
  }

  driver.send_data(display, data)?;
  driver.state.displays[display as usize] = data;

  Ok(())
}
