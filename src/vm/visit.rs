//! Virtual machine instruction visitor.

use super::{error::VmError, support::Support, Vm};
use crate::insn::visit::InsnVisitor;

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
      .reg8
      .get(reg as usize)
      .copied()
      .ok_or(VmError::BadReg(reg))
  }

  /// Sets value of register `reg`.
  fn set_reg(&mut self, reg: u8, val: u8) -> Result<(), VmError> {
    *self
      .reg8
      .get_mut(reg as usize)
      .ok_or(VmError::BadReg(reg))? = val;

    Ok(())
  }

  // Pops value from top of stack.
  fn stack_pop(&mut self) -> Result<u16, VmError> {
    if self.reg_sp == 0 {
      return Err(VmError::StackUnderflow);
    }

    let val = self.stack[self.reg_sp as usize];
    self.reg_sp -= 1;

    Ok(val)
  }

  // Pushes value to top of stack.
  fn stack_push(&mut self, val: u16) -> Result<(), VmError> {
    if self.reg_sp as usize == self.stack.len() {
      return Err(VmError::StackOverflow);
    }

    self.reg_sp += 1;
    self.stack[self.reg_sp as usize] = val;

    Ok(())
  }
}

impl<S> InsnVisitor for Vm<S>
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
    self.stack_push(self.reg_pc)?;
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
    self.set_reg(x, self.get_reg(x)? + kk)?;
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

  fn ld_i_nnn(&mut self, x: u8, nnn: u16) -> Self::Result {
    self.reg_i = nnn;
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
    let sprite = &self.mem[self.reg_i as usize..n as usize];
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
    self.set_reg(x, self.reg_dt)?;
    Ok(Step::Next)
  }

  fn ld_x_k(&mut self, x: u8) -> Self::Result {
    let key = self.support.wait_key_pressed()?;
    self.set_reg(x, key)?;
    Ok(Step::Next)
  }

  fn ld_dt_x(&mut self, x: u8) -> Self::Result {
    self.reg_dt = self.get_reg(x)?;
    Ok(Step::Next)
  }

  fn ld_st_x(&mut self, x: u8) -> Self::Result {
    self.reg_st = self.get_reg(x)?;
    Ok(Step::Next)
  }

  fn add_i_x(&mut self, x: u8) -> Self::Result {
    self.reg_i += self.get_reg(x)? as u16;
    Ok(Step::Next)
  }

  fn ld_f_x(&mut self, x: u8) -> Self::Result {
    todo!(
      r"Set I = location of sprite for digit Vx.
      
      The value of I is set to the location for the hexadecimal sprite corresponding to 
      the value of Vx. See section 2.4, Display, for more information on the Chip-8 
      hexadecimal font."
    )
  }

  fn ld_b_x(&mut self, x: u8) -> Self::Result {
    todo!(
      r"Store BCD representation of Vx in memory locations I, I+1, and I+2.
      
      The interpreter takes the decimal value of Vx, and places the hundreds digit in 
      memory at location in I, the tens digit at location I+1, and the ones digit at 
      location I+2."
    )
  }

  fn ld_deref_i_x(&mut self, x: u8) -> Self::Result {
    todo!(
      r"Store registers V0 through Vx in memory starting at location I.
      
      The interpreter copies the values of registers V0 through Vx into memory, starting 
      at the address in I."
    )
  }

  fn ld_x_deref_i(&mut self, x: u8) -> Self::Result {
    todo!(
      r"Read registers V0 through Vx from memory starting at location I.
    
      The interpreter reads values from memory starting at location I into registers V0 
      through Vx."
    )
  }

  fn invalid(&mut self, hi: u8, lo: u8) -> Self::Result {
    Err(VmError::BadInsn(hi as u16 | (lo as u16) << 8))
  }
}
