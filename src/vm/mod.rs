//! Virtual machine.

use self::{error::VmError, support::Support};
use crate::{insn::visit::InsnVisitor, vm::visit::Step};

pub mod error;
pub mod sprites;
pub mod support;
pub mod visit;

const MEM_OFF_BEG_PROG: usize = 0x200;

/// A chip-8 virtual machine.
pub struct Vm<S> {
  mem: Vec<u8>,
  reg8: [u8; 16],
  reg_i: u16,
  reg_dt: u8,
  reg_st: u8,
  reg_pc: u16,
  reg_sp: u8,
  stack: [u16; 16],

  support: S,
}

impl<S> Vm<S> {
  /// Create a [Vm] with the supplied graphics and keyboard.
  pub fn new(support: S) -> Self {
    let mut mem = vec![0; 4096];

    sprites::copy_to(&mut mem[..]);

    Vm {
      mem,
      reg8: [0; 16],
      reg_i: 0,
      reg_dt: 0,
      reg_st: 0,
      reg_pc: 0,
      reg_sp: 0,
      stack: [0; 16],

      support,
    }
  }

  /// Loads a program into memory and resets all registers and stack.
  pub fn load_program(&mut self, program: &[u8]) -> Result<(), VmError> {
    if program.len() > self.mem.len() - MEM_OFF_BEG_PROG {
      return Err(VmError::BadInsn(0));
    }

    let mut mem = vec![0; 4096];

    sprites::copy_to(&mut mem[..]);
    mem[MEM_OFF_BEG_PROG..program.len()].copy_from_slice(program);

    self.mem = mem;
    self.reg8 = [0; 16];
    self.reg_i = 0;
    self.reg_dt = 0;
    self.reg_st = 0;
    self.reg_pc = 0;
    self.reg_sp = 0;
    self.stack = [0; 16];

    Ok(())
  }

  /// Gets reference to support.
  pub fn support(&self) -> &S {
    &self.support
  }

  /// Gets mutable reference to support.
  pub fn support_mut(&mut self) -> &mut S {
    &mut self.support
  }
}

impl<S> Vm<S>
where
  S: Support,
{
  /// Steps the virtual machine.
  pub fn update(&mut self) -> Result<(), VmError> {
    let prog = self.reg_pc as usize * 2 + MEM_OFF_BEG_PROG;
    let hi = self.mem[prog];
    let lo = self.mem[prog + 1];
    let step = self.visit_insn(hi, lo)?;
    match step {
      Step::Next => self.reg_pc += 1,
      Step::Skip => self.reg_pc += 2,
      Step::Jump(offset) => self.reg_pc = offset,
    }

    Ok(())
  }
}
