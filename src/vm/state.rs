//! Virtual machine state.

use crate::insn::Insn;

use super::{error::VmError, sprites, OFF_PROG};

#[derive(Debug, Clone)]
pub struct VmState {
  pub mem: Vec<u8>,
  pub reg8: [u8; 16],
  pub reg_i: u16,
  pub reg_dt: u8,
  pub reg_st: u8,
  pub reg_pc: u16,
  pub reg_sp: u8,
  pub stack: [u16; 16],
}

impl VmState {
  /// Loads a program into memory and resets all registers and stack.
  pub fn load_program(&mut self, program: &[u8]) -> Result<(), VmError> {
    let max_len = self.mem.len() - OFF_PROG;
    if max_len < program.len() {
      return Err(VmError::BadProgramTooLarge(program.len(), max_len));
    }

    self.mem = vec![0; self.mem.len()];

    // Copy sprites to memory.
    sprites::copy_to(&mut self.mem[..]);

    // Copy program to memory.
    let prog_beg = OFF_PROG;
    let prog_end = prog_beg + program.len();
    self.mem[prog_beg..prog_end].copy_from_slice(program);

    // Reset registers and stack.
    self.reg8 = [0; 16];
    self.reg_i = 0;
    self.reg_dt = 0;
    self.reg_st = 0;
    self.reg_pc = OFF_PROG as u16;
    self.reg_sp = 0;
    self.stack = [0; 16];

    Ok(())
  }

  pub fn get_insn_bytes(&self) -> Option<(u8, u8)> {
    self.get_insn_bytes_at(self.reg_pc as usize)
  }

  pub fn get_insn_at(&self, i: usize) -> Option<Insn> {
    let (hi, lo) = self.get_insn_bytes_at(i)?;

    Insn::from_bytes(hi, lo)
  }

  pub fn get_insn_bytes_at(&self, i: usize) -> Option<(u8, u8)> {
    let hi = *self.mem.get(i)?;
    let lo = *self.mem.get(i + 1)?;

    Some((hi, lo))
  }
}

impl Default for VmState {
  fn default() -> Self {
    Self {
      mem: vec![0; 4096],
      reg8: [0; 16],
      reg_i: 0,
      reg_dt: 0,
      reg_st: 0,
      reg_pc: 0,
      reg_sp: 0,
      stack: [0; 16],
    }
  }
}
