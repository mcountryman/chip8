//! Virtual machine.

use self::{
  error::VmError,
  state::VmState,
  support::{Support, Terminal},
};
use crate::{insn::visit::InsnVisit, vm::visit::Step};
use std::time::{Duration, Instant};

pub mod error;
pub mod sprites;
pub mod state;
pub mod support;
pub mod visit;

pub const OFF_PROG: usize = 0x200;
pub const SIZE_INSN: u16 = 2;

/// A chip-8 virtual machine.
pub struct Vm<S> {
  pub state: VmState,

  last: Instant,
  support: S,
}

impl<S> Vm<S> {
  /// Create a [Vm] with the supplied graphics and keyboard.
  pub fn new(support: S) -> Self {
    Vm {
      last: Instant::now(),
      state: VmState::default(),
      support,
    }
  }

  /// Loads a program into memory and resets all registers and stack.
  pub fn load_program(&mut self, program: &[u8]) -> Result<(), VmError> {
    self.state.load_program(program)
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

impl Vm<Terminal> {
  /// Steps the virtual machine.
  pub fn update(&mut self) -> Result<(), VmError> {
    let (hi, lo) = self.state.get_insn_bytes().unwrap_or((0, 0));
    let step = self.visit_insn(hi, lo)?;

    self.state.reg_dt = self.state.reg_dt.saturating_sub(1);
    self.state.reg_st = self.state.reg_st.saturating_sub(1);

    match step {
      Step::Next => self.state.reg_pc += SIZE_INSN,
      Step::Skip => self.state.reg_pc += SIZE_INSN * 2,
      Step::Jump(offset) => self.state.reg_pc = offset,
    }

    let sleep = self.last.elapsed();
    let sleep = Duration::from_millis(500) - sleep;

    self
      .support
      .update(&self.state, sleep)
      .expect("Failed to update terminal.");

    self.last = Instant::now();

    Ok(())
  }
}
