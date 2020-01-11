use std::io::{Read, Result};

pub const MEMORY_END: usize = 0xFFF;
pub const MEMORY_BEGIN: usize = 0x000;
pub const MEMORY_PROGRAM: usize = 0x200;

pub struct Ram {
  pub memory: [u8; MEMORY_END],
}

impl Ram {
  pub fn new() -> Self {
    Self{
      memory: [0u8; MEMORY_END],
    }
  }

  pub fn load<R: Read>(&mut self, read: &mut R) -> Result<usize> {
    read.read(&mut self.memory[MEMORY_PROGRAM..])
  }

  pub fn reset(&mut self) {
    // Reset program block
    for i in MEMORY_PROGRAM..MEMORY_END {
      self.memory[i] = 0;
    }
  }
}