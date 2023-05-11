pub struct State {
  pub active: bool,
  pub intensity: u8,
  pub displays: Vec<u16>,
}

impl State {
  pub fn new(max_displays: usize) -> Self {
    State {
      active: false,
      intensity: 0,
      displays: vec![0; max_displays],
    }
  }
}
