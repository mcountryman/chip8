
pub struct Ipu {}

impl Ipu {
  pub fn new() -> Self {
    Self {}
  }

  pub fn wait_key(&self) -> u8 {
    return 0;
  }

  pub fn is_key_pressed(&self, key: u8) -> bool {
    return false;
  }
}
