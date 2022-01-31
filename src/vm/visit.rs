//! Virtual machine instruction visitor.

use super::{error::VmError, support::Support, Vm};
use crate::insn::visit::InsnVisit;

/// A virtual machine step.
pub enum Step {
  /// Next cycle should execute next instruction in program.
  Next,
  /// Next cycle should execute the instruction following the next instruction in program.
  Skip,
  /// Next cycle should execute the instruction at supplied offset in program.
  Jump(u16),
}

impl<S> Vm<S> {
  /// Gets value of register `reg`.
  fn get_reg(&self, reg: u8) -> Result<u8, VmError> {
    self
      .state
      .reg8
      .get(reg as usize)
      .copied()
      .ok_or(VmError::BadReg(reg))
  }

  /// Sets value of register `reg`.
  fn set_reg(&mut self, reg: u8, val: u8) -> Result<(), VmError> {
    *self
      .state
      .reg8
      .get_mut(reg as usize)
      .ok_or(VmError::BadReg(reg))? = val;

    Ok(())
  }

  // Pops value from top of stack.
  fn stack_pop(&mut self) -> Result<u16, VmError> {
    if self.state.reg_sp == 0 {
      return Err(VmError::StackUnderflow);
    }

    let val = self.state.stack[self.state.reg_sp as usize];
    self.state.reg_sp -= 1;

    Ok(val)
  }

  // Pushes value to top of stack.
  fn stack_push(&mut self, val: u16) -> Result<(), VmError> {
    if self.state.reg_sp as usize == self.state.stack.len() {
      return Err(VmError::StackOverflow);
    }

    self.state.reg_sp += 1;
    self.state.stack[self.state.reg_sp as usize] = val;

    Ok(())
  }
}

impl<S> InsnVisit for Vm<S>
where
  S: Support,
{
  type Result = Result<Step, VmError>;

  fn nop(&mut self) -> Self::Result {
    Ok(Step::Next)
  }

  fn cls(&mut self) -> Self::Result {
    self.support.clear()?;
    Ok(Step::Next)
  }

  fn ret(&mut self) -> Self::Result {
    Ok(Step::Jump(self.stack_pop()?))
  }

  fn sys_nnn(&mut self, nnn: u16) -> Self::Result {
    todo!(
      r"Jump to a machine code routine at nnn.
    
      This instruction is only used on the old computers on which Chip-8 was originally 
      implemented. It is ignored by modern interpreters."
    )
  }

  fn jp_nnn(&mut self, nnn: u16) -> Self::Result {
    Ok(Step::Jump(nnn))
  }

  fn call_nnn(&mut self, nnn: u16) -> Self::Result {
    self.stack_push(self.state.reg_pc)?;
    Ok(Step::Jump(nnn))
  }

  fn se_x_kk(&mut self, x: u8, kk: u8) -> Self::Result {
    if self.get_reg(x)? == kk {
      Ok(Step::Skip)
    } else {
      Ok(Step::Next)
    }
  }

  fn sne_x_kk(&mut self, x: u8, kk: u8) -> Self::Result {
    if self.get_reg(x)? != kk {
      Ok(Step::Skip)
    } else {
      Ok(Step::Next)
    }
  }

  fn se_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    if self.get_reg(x)? == self.get_reg(y)? {
      Ok(Step::Skip)
    } else {
      Ok(Step::Next)
    }
  }

  fn ld_x_kk(&mut self, x: u8, kk: u8) -> Self::Result {
    self.set_reg(x, kk)?;
    Ok(Step::Next)
  }

  fn add_x_kk(&mut self, x: u8, kk: u8) -> Self::Result {
    self.set_reg(x, self.get_reg(x)?.wrapping_add(kk))?;
    Ok(Step::Next)
  }

  fn ld_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    self.set_reg(x, self.get_reg(y)?)?;
    Ok(Step::Next)
  }

  fn or_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    self.set_reg(x, self.get_reg(x)? | self.get_reg(y)?)?;
    Ok(Step::Next)
  }

  fn and_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    self.set_reg(x, self.get_reg(x)? & self.get_reg(y)?)?;
    Ok(Step::Next)
  }

  fn xor_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    self.set_reg(x, self.get_reg(x)? ^ self.get_reg(y)?)?;
    Ok(Step::Next)
  }

  fn add_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    let (val, carry) = self.get_reg(x)?.overflowing_add(self.get_reg(y)?);
    self.set_reg(x, val)?;
    self.set_reg(0xf, carry as u8)?;
    Ok(Step::Next)
  }

  fn sub_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    let (val, carry) = self.get_reg(x)?.overflowing_sub(self.get_reg(y)?);
    self.set_reg(x, val)?;
    self.set_reg(0xf, !carry as u8)?;
    Ok(Step::Next)
  }

  fn shr_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    let val = self.get_reg(x)?;
    let carry = val & 0x1;
    self.set_reg(x, val >> 1)?;
    self.set_reg(0xf, carry)?;
    Ok(Step::Next)
  }

  fn subn_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    let (val, carry) = self.get_reg(y)?.overflowing_sub(self.get_reg(x)?);
    self.set_reg(x, val)?;
    self.set_reg(0xf, !carry as u8)?;
    Ok(Step::Next)
  }

  fn shl_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    let val = self.get_reg(x)?;
    let carry = val & 0x80;
    self.set_reg(x, val << 1)?;
    self.set_reg(0xf, carry)?;
    Ok(Step::Next)
  }

  fn sne_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    if self.get_reg(x)? != self.get_reg(y)? {
      Ok(Step::Skip)
    } else {
      Ok(Step::Next)
    }
  }

  fn ld_i_nnn(&mut self, nnn: u16) -> Self::Result {
    self.state.reg_i = nnn;
    Ok(Step::Next)
  }

  fn jp_0_nnn(&mut self, nnn: u16) -> Self::Result {
    Ok(Step::Jump(self.get_reg(0)? as u16 + nnn))
  }

  fn rnd(&mut self, x: u8, kk: u8) -> Self::Result {
    let rnd = rand::random::<u8>();
    let rnd = rnd & kk;

    self.set_reg(x, rnd)?;

    Ok(Step::Next)
  }

  fn drw(&mut self, x: u8, y: u8, n: u8) -> Self::Result {
    let x = self.get_reg(x)?;
    let y = self.get_reg(y)?;
    let beg = self.state.reg_i as usize;
    let end = beg + n as usize;
    let sprite = &self.state.mem[beg..end];
    self.support.draw(sprite, x, y)?;
    Ok(Step::Next)
  }

  fn skp_x(&mut self, x: u8) -> Self::Result {
    let key = self.get_reg(x)?;
    if self.support.is_key_pressed(key)? {
      Ok(Step::Skip)
    } else {
      Ok(Step::Next)
    }
  }

  fn sknp_x(&mut self, x: u8) -> Self::Result {
    let key = self.get_reg(x)?;
    if !self.support.is_key_pressed(key)? {
      Ok(Step::Skip)
    } else {
      Ok(Step::Next)
    }
  }

  fn ld_x_dt(&mut self, x: u8) -> Self::Result {
    self.set_reg(x, self.state.reg_dt)?;
    Ok(Step::Next)
  }

  fn ld_x_k(&mut self, x: u8) -> Self::Result {
    let key = self.support.wait_key_pressed()?;
    self.set_reg(x, key)?;
    Ok(Step::Next)
  }

  fn ld_dt_x(&mut self, x: u8) -> Self::Result {
    self.state.reg_dt = self.get_reg(x)?;
    Ok(Step::Next)
  }

  fn ld_st_x(&mut self, x: u8) -> Self::Result {
    self.state.reg_st = self.get_reg(x)?;
    Ok(Step::Next)
  }

  fn add_i_x(&mut self, x: u8) -> Self::Result {
    self.state.reg_i = self.state.reg_i.wrapping_add(self.get_reg(x)? as u16);
    Ok(Step::Next)
  }

  fn ld_f_x(&mut self, x: u8) -> Self::Result {
    self.state.reg_i = self.get_reg(x)? as u16 * 5;

    Ok(Step::Next)
  }

  fn ld_b_x(&mut self, x: u8) -> Self::Result {
    let x = self.get_reg(x)?;

    self.state.mem[self.state.reg_i as usize] = x / 100;
    self.state.mem[self.state.reg_i as usize + 1] = (x % 100) / 10;
    self.state.mem[self.state.reg_i as usize + 2] = x % 10;

    Ok(Step::Next)
  }

  fn ld_deref_i_x(&mut self, x: u8) -> Self::Result {
    for i in 0..x {
      self.state.mem[self.state.reg_i as usize + i as usize] = self.get_reg(i)?;
    }

    Ok(Step::Next)
  }

  fn ld_x_deref_i(&mut self, x: u8) -> Self::Result {
    for i in 0..x {
      self.set_reg(i, self.state.mem[self.state.reg_i as usize + i as usize])?;
    }

    Ok(Step::Next)
  }

  fn invalid(&mut self, hi: u8, lo: u8) -> Self::Result {
    Err(VmError::BadInsn((hi as u16) << 8 | lo as u16))
  }
}
