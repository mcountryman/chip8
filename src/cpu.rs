use std::io::{Read, Result};
use crate::ram::{Ram, MEMORY_PROGRAM};
use std::time::Instant;

pub const V0: usize = 0x00;
pub const V1: usize = 0x01;
pub const V2: usize = 0x02;
pub const V3: usize = 0x03;
pub const V4: usize = 0x04;
pub const V5: usize = 0x05;
pub const V6: usize = 0x06;
pub const V7: usize = 0x07;
pub const V8: usize = 0x08;
pub const V9: usize = 0x09;
pub const VA: usize = 0x0A;
pub const VB: usize = 0x0B;
pub const VC: usize = 0x0C;
pub const VD: usize = 0x0D;
pub const VE: usize = 0x0E;
pub const VF: usize = 0x0F;

pub const ST: usize = 0x10;
pub const DT: usize = 0x11;

pub struct Cpu {
  ram: Ram,

  reg8: [u8; 0x11],
  reg16: u16,
  stack: [u16; 16],
  clock: Instant,

  stack_pointer: usize,
  program_counter: usize,
}

enum Step {
  Next,
  Skip,
  Jump(usize),
}

impl Cpu {

  pub fn new() -> Self {
    Self {
      ram: Ram::new(),

      reg8: [0u8; 0x11],
      reg16: 0,
      stack: [0u16; 0x10],
      clock: Instant::now(),

      stack_pointer: 0,
      program_counter: 0,
    }
  }

  pub fn load<R: Read>(&mut self, read: &mut R) -> Result<usize> {
    self.ram.load(read)
  }

  pub fn tick(&mut self) -> Result<()> {
    // Tick timers
    if self.clock.elapsed().as_millis() > 16 {
      if self.reg8[DT] > 0 {
        self.reg8[DT] -= 1;
      }

      if self.reg8[ST] > 0 {
        self.reg8[ST] -= 1;
      }

      self.clock = Instant::now()
    }

    // Run instruction
    {
      let instruction = self.get_instruction();
      let step = self.run_instruction(instruction)?;

      match step {
        Step::Next => self.program_counter += 1,
        Step::Skip => self.program_counter += 2,
        Step::Jump(addr) => self.program_counter = addr,
      }
    }

    Ok(())
  }

  pub fn reset(&mut self) {
    // Reset reg16 & counter
    self.reg16 = 0;
    self.stack_pointer = 0;
    self.program_counter = 0;

    // Reset registers
    for i in 0..DT {
      self.reg8[i] = 0;
    }

    // Reset stack
    for i in 0..16 {
      self.stack[i] = 0;
    }

    self.ram.reset();
  }

  fn get_instruction(&self) -> u16 {
    let bite1 = self.ram.memory[MEMORY_PROGRAM + self.program_counter];
    let bite2 = self.ram.memory[MEMORY_PROGRAM + self.program_counter + 1];

    (bite1 as u16) << 8 | bite2 as u16
  }

  fn run_instruction(&mut self, instruction: u16) -> Result<Step> {
    let nibbles = (
      (instruction & 0xF000 >> 12) as u8,
      (instruction & 0x0F00 >> 8) as u8,
      (instruction & 0x00F0 >> 4) as u8,
      (instruction & 0x000F) as u8,
    );

    let nnn = (instruction & 0x0FFF) as usize;
    let kk = (instruction & 0x00FF) as u8;
    let x = nibbles.1 as usize;
    let y = nibbles.2 as usize;
    let n = nibbles.3 as usize;

    match nibbles {
      (0x00, 0x00, 0x0e, 0x00) => {},
      (0x00, 0x00, 0x0e, 0x0e) => {},
      _ => {},
    }

    Ok(Step::Next)
  }

}
