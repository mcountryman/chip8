//! Virtual machine instruction visitor.

use super::{error::VmError, Vm};
use crate::insn::visit::InsnVisit;

/// A virtual machine step.
pub enum Step {
  /// Next cycle should execute next instruction in program.
  Next,
  /// Next cycle should execute the instruction following the next instruction in program.
  Skip,
  /// Next cycle should execute the instruction at supplied offset in program.
  Jump(u16),
  /// Next cycle should be executed after a key is pressed.
  WaitKey(u8),
}

impl Step {
  /// Creates a [Step] that indicates the next instruction should be skipped if supplied
  /// condition is true.
  #[inline]
  pub fn skip_if(condition: bool) -> Self {
    if condition {
      Step::Skip
    } else {
      Step::Next
    }
  }
}

impl Vm {
  /// Gets value of register `reg`.
  #[inline]
  fn get_reg(&self, reg: u8) -> Result<u8, VmError> {
    self
      .reg8
      .get(reg as usize)
      .copied()
      .ok_or(VmError::BadReg(reg))
  }

  /// Sets value of register `reg`.
  #[inline]
  fn set_reg(&mut self, reg: u8, val: u8) -> Result<(), VmError> {
    *self
      .reg8
      .get_mut(reg as usize)
      .ok_or(VmError::BadReg(reg))? = val;

    Ok(())
  }

  // Pops value from top of stack.
  #[inline]
  fn stack_pop(&mut self) -> Result<u16, VmError> {
    if self.reg_sp == 0 {
      return Err(VmError::StackUnderflow);
    }

    let val = self.stack[self.reg_sp as usize];
    self.reg_sp -= 1;

    Ok(val)
  }

  // Pushes value to top of stack.
  #[inline]
  fn stack_push(&mut self, val: u16) -> Result<(), VmError> {
    if self.reg_sp as usize == self.stack.len() {
      return Err(VmError::StackOverflow);
    }

    self.reg_sp += 1;
    self.stack[self.reg_sp as usize] = val;

    Ok(())
  }
}

impl InsnVisit for Vm {
  type Result = Result<Step, VmError>;

  #[inline]
  fn nop(&mut self) -> Self::Result {
    Ok(Step::Next)
  }

  #[inline]
  fn cls(&mut self) -> Self::Result {
    self.vram = [0; 32];
    Ok(Step::Next)
  }

  #[inline]
  fn ret(&mut self) -> Self::Result {
    Ok(Step::Jump(self.stack_pop()?))
  }

  #[inline]
  fn sys_nnn(&mut self, _: u16) -> Self::Result {
    todo!(
      r"Jump to a machine code routine at nnn.
    
      This instruction is only used on the old computers on which Chip-8 was originally 
      implemented. It is ignored by modern interpreters."
    )
  }

  #[inline]
  fn jp_nnn(&mut self, nnn: u16) -> Self::Result {
    Ok(Step::Jump(nnn))
  }

  #[inline]
  fn call_nnn(&mut self, nnn: u16) -> Self::Result {
    self.stack_push(self.reg_pc + 2)?;
    Ok(Step::Jump(nnn))
  }

  #[inline]
  fn se_x_kk(&mut self, x: u8, kk: u8) -> Self::Result {
    Ok(Step::skip_if(self.get_reg(x)? == kk))
  }

  #[inline]
  fn sne_x_kk(&mut self, x: u8, kk: u8) -> Self::Result {
    Ok(Step::skip_if(self.get_reg(x)? != kk))
  }

  #[inline]
  fn se_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    Ok(Step::skip_if(self.get_reg(x)? == self.get_reg(y)?))
  }

  #[inline]
  fn ld_x_kk(&mut self, x: u8, kk: u8) -> Self::Result {
    self.set_reg(x, kk)?;
    Ok(Step::Next)
  }

  #[inline]
  fn add_x_kk(&mut self, x: u8, kk: u8) -> Self::Result {
    self.set_reg(x, self.get_reg(x)?.wrapping_add(kk))?;
    Ok(Step::Next)
  }

  #[inline]
  fn ld_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    self.set_reg(x, self.get_reg(y)?)?;
    Ok(Step::Next)
  }

  #[inline]
  fn or_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    self.set_reg(x, self.get_reg(x)? | self.get_reg(y)?)?;
    Ok(Step::Next)
  }

  #[inline]
  fn and_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    self.set_reg(x, self.get_reg(x)? & self.get_reg(y)?)?;
    Ok(Step::Next)
  }

  #[inline]
  fn xor_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    self.set_reg(x, self.get_reg(x)? ^ self.get_reg(y)?)?;
    Ok(Step::Next)
  }

  #[inline]
  fn add_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    let (val, carry) = self.get_reg(x)?.overflowing_add(self.get_reg(y)?);
    self.set_reg(x, val)?;
    self.set_reg(0xf, carry as u8)?;
    Ok(Step::Next)
  }

  #[inline]
  fn sub_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    let (val, carry) = self.get_reg(x)?.overflowing_sub(self.get_reg(y)?);
    self.set_reg(x, val)?;
    self.set_reg(0xf, !carry as u8)?;
    Ok(Step::Next)
  }

  #[inline]
  fn shr_x_y(&mut self, x: u8, _: u8) -> Self::Result {
    let val = self.get_reg(x)?;

    self.set_reg(x, val >> 1)?;
    self.set_reg(0xf, val & 0x1)?;

    Ok(Step::Next)
  }

  #[inline]
  fn subn_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    let (val, carry) = self.get_reg(y)?.overflowing_sub(self.get_reg(x)?);
    self.set_reg(x, val)?;
    self.set_reg(0xf, !carry as u8)?;
    Ok(Step::Next)
  }

  #[inline]
  fn shl_x_y(&mut self, x: u8, _: u8) -> Self::Result {
    let val = self.get_reg(x)?;
    self.set_reg(x, val << 1)?;
    self.set_reg(0xf, val >> 7)?;
    Ok(Step::Next)
  }

  #[inline]
  fn sne_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    if self.get_reg(x)? != self.get_reg(y)? {
      Ok(Step::Skip)
    } else {
      Ok(Step::Next)
    }
  }

  #[inline]
  fn ld_i_nnn(&mut self, nnn: u16) -> Self::Result {
    self.reg_i = nnn;
    Ok(Step::Next)
  }

  #[inline]
  fn jp_0_nnn(&mut self, nnn: u16) -> Self::Result {
    Ok(Step::Jump(self.get_reg(0)? as u16 + nnn))
  }

  #[inline]
  fn rnd(&mut self, x: u8, kk: u8) -> Self::Result {
    let rnd = rand::random::<u8>();
    let rnd = rnd & kk;

    self.set_reg(x, rnd)?;

    Ok(Step::Next)
  }

  #[inline]
  fn drw(&mut self, x: u8, y: u8, n: u8) -> Self::Result {
    let n = n as usize;
    let x = self.get_reg(x)? as u32;
    let y = self.get_reg(y)? as usize;

    for i in 0..n {
      let byte = self.mem[self.reg_i as usize + i];
      let vram_y = (y + i) % self.vram.len();
      let vram = self.vram[vram_y];

      let mask = (byte as u64) << (64 - 8);
      let mask = mask.rotate_right(x);

      self.reg8[0xf] |= (mask & vram).rotate_left(x).rotate_right(64 - 8) as u8;
      self.vram[vram_y] ^= mask;
    }

    Ok(Step::Next)
  }

  #[inline]
  fn skp_x(&mut self, x: u8) -> Self::Result {
    let key = self.get_reg(x)?.into();
    if self.keys.contains(key) {
      Ok(Step::Skip)
    } else {
      Ok(Step::Next)
    }
  }

  #[inline]
  fn sknp_x(&mut self, x: u8) -> Self::Result {
    let key = self.get_reg(x)?.into();
    if !self.keys.contains(key) {
      Ok(Step::Skip)
    } else {
      Ok(Step::Next)
    }
  }

  #[inline]
  fn ld_x_dt(&mut self, x: u8) -> Self::Result {
    self.set_reg(x, self.reg_dt)?;
    Ok(Step::Next)
  }

  #[inline]
  fn ld_x_k(&mut self, x: u8) -> Self::Result {
    Ok(Step::WaitKey(x))
  }

  #[inline]
  fn ld_dt_x(&mut self, x: u8) -> Self::Result {
    self.reg_dt = self.get_reg(x)?;
    Ok(Step::Next)
  }

  #[inline]
  fn ld_st_x(&mut self, x: u8) -> Self::Result {
    self.reg_st = self.get_reg(x)?;
    Ok(Step::Next)
  }

  #[inline]
  fn add_i_x(&mut self, x: u8) -> Self::Result {
    self.reg_i = self.reg_i.wrapping_add(self.get_reg(x)? as u16);
    Ok(Step::Next)
  }

  #[inline]
  fn ld_f_x(&mut self, x: u8) -> Self::Result {
    self.reg_i = self.get_reg(x)? as u16 * 5;

    Ok(Step::Next)
  }

  #[inline]
  fn ld_b_x(&mut self, x: u8) -> Self::Result {
    let x = self.get_reg(x)?;

    self.mem[self.reg_i as usize] = x / 100;
    self.mem[self.reg_i as usize + 1] = (x % 100) / 10;
    self.mem[self.reg_i as usize + 2] = x % 10;

    Ok(Step::Next)
  }

  #[inline]
  fn ld_deref_i_x(&mut self, x: u8) -> Self::Result {
    for i in 0..x {
      self.mem[self.reg_i as usize + i as usize] = self.get_reg(i)?;
    }

    Ok(Step::Next)
  }

  #[inline]
  fn ld_x_deref_i(&mut self, x: u8) -> Self::Result {
    for i in 0..x {
      self.set_reg(i, self.mem[self.reg_i as usize + i as usize])?;
    }

    Ok(Step::Next)
  }

  #[inline]
  fn invalid(&mut self, hi: u8, lo: u8) -> Self::Result {
    Err(VmError::BadInsn((hi as u16) << 8 | lo as u16))
  }
}
