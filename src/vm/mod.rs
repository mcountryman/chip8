//! Virtual machine.

use self::{error::VmError, flags::VmKey, visit::Step};
use crate::insn::visit::InsnVisit;

pub mod error;
pub mod flags;
pub mod sprites;
pub mod visit;

/// Offset of program space in memory.
pub const PROG_OFF: usize = 0x200;
/// Size of an instruction in bytes.
pub const INSN_SIZE: u16 = 2;
/// Width of vram buffer.
pub const VRAM_WIDTH: usize = std::mem::size_of::<u64>();
/// Height of vram buffer.
pub const VRAM_HEIGHT: usize = 32;

/// A chip-8 virtual machine.
pub struct Vm {
  pub mem: Vec<u8>,
  pub reg8: [u8; 16],
  pub reg_i: u16,
  pub reg_dt: u8,
  pub reg_st: u8,
  pub reg_pc: u16,
  pub reg_sp: u8,
  pub stack: [u16; 16],
  pub vram: [u64; VRAM_HEIGHT],

  pub keys: VmKey,
  wait_key: bool,
  wait_key_reg: u8,
}

impl Vm {
  /// Create a [Vm].
  pub fn new() -> Self {
    Self::default()
  }

  /// Loads a program into memory and resets registers, stack, memory, and vram.
  pub fn load_program(&mut self, program: &[u8]) -> Result<(), VmError> {
    let max_len = self.mem.len() - PROG_OFF;
    if max_len < program.len() {
      return Err(VmError::BadProgramTooLarge(program.len(), max_len));
    }

    self.mem = vec![0; self.mem.len()];

    // Copy sprites to memory.
    sprites::copy_to(&mut self.mem[..]);

    // Copy program to memory.
    let prog_beg = PROG_OFF;
    let prog_end = prog_beg + program.len();
    self.mem[prog_beg..prog_end].copy_from_slice(program);

    // Reset registers and stack.
    self.reg8 = [0; 16];
    self.reg_i = 0;
    self.reg_dt = 0;
    self.reg_st = 0;
    self.reg_pc = PROG_OFF as u16;
    self.reg_sp = 0;
    self.stack = [0; 16];
    self.vram = [0; VRAM_HEIGHT];
    self.wait_key = false;
    self.wait_key_reg = 0;

    Ok(())
  }

  /// Updates the virtual machine.
  ///
  /// Ideally this should be executed at 500Hz.
  pub fn update(&mut self) -> Result<(), VmError> {
    if self.wait_key {
      return Ok(());
    }

    let (hi, lo) = self.get_insn_bytes().unwrap_or((0, 0));
    let step = self.visit_insn(hi, lo)?;

    match step {
      Step::Next => self.reg_pc += INSN_SIZE,
      Step::Skip => self.reg_pc += INSN_SIZE * 2,
      Step::Jump(offset) => self.reg_pc = offset,
      Step::WaitKey(wait_key_reg) => {
        self.wait_key = true;
        self.wait_key_reg = wait_key_reg;
      }
    }

    Ok(())
  }

  /// Updates the virtual machine timers.
  ///
  /// Ideally this should be executed at 60Hz.
  pub fn update_timers(&mut self) {
    self.reg_dt = self.reg_dt.saturating_sub(1);
    self.reg_st = self.reg_st.saturating_sub(1);
  }

  /// Sends a signal to the virtual machine that a key has been released.
  pub fn signal_key_up(&mut self, key: VmKey) {
    self.keys.remove(key);
  }

  /// Sends a signal to the virtual machine that a key has been pressed.
  pub fn signal_key_down(&mut self, key: VmKey) {
    self.keys.insert(key);

    if self.wait_key {
      self.wait_key = false;

      // todo: We should probably provide a mechanism for returning a result indicating
      // that the source instruction is attempting to access a register that doesn't exist
      // if register out of bounds.
      self.reg8[self.wait_key_reg as usize] = key.into();
    }
  }

  #[inline]
  fn get_insn_bytes(&self) -> Option<(u8, u8)> {
    self.get_insn_bytes_at(self.reg_pc as usize)
  }

  #[inline]
  pub fn get_insn_bytes_at(&self, i: usize) -> Option<(u8, u8)> {
    let hi = *self.mem.get(i)?;
    let lo = *self.mem.get(i + 1)?;

    Some((hi, lo))
  }
}

impl Default for Vm {
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
      vram: [0; VRAM_HEIGHT],

      keys: VmKey::empty(),
      wait_key: false,
      wait_key_reg: 0,
    }
  }
}
