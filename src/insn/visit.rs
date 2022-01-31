//! Instruction visiting.
//!
//! # Operands
//! nnn - A 12-bit value, the lowest 12 bits of the instruction
//! n - A 4-bit value, the lowest 4 bits of the instruction
//! x - A 4-bit value, the lower 4 bits of the high byte of the instruction
//! y - A 4-bit value, the upper 4 bits of the low byte of the instruction
//! kk - An 8-bit value, the lowest 8 bits of the instruction
//!
//! # Instructions
//! * `00E0` - CLS
//! * `00EE` - RET
//! * `0nnn` - SYS addr
//! * `1nnn` - JP addr
//! * `2nnn` - CALL addr
//! * `3xkk` - SE Vx, byte
//! * `4xkk` - SNE Vx, byte
//! * `5xy0` - SE Vx, Vy
//! * `6xkk` - LD Vx, byte
//! * `7xkk` - ADD Vx, byte
//! * `8xy0` - LD Vx, Vy
//! * `8xy1` - OR Vx, Vy
//! * `8xy2` - AND Vx, Vy
//! * `8xy3` - XOR Vx, Vy
//! * `8xy4` - ADD Vx, Vy
//! * `8xy5` - SUB Vx, Vy
//! * `8xy6` - SHR Vx {, Vy}
//! * `8xy7` - SUBN Vx, Vy
//! * `8xyE` - SHL Vx {, Vy}
//! * `9xy0` - SNE Vx, Vy
//! * `Annn` - LD I, addr
//! * `Bnnn` - JP V0, addr
//! * `Cxkk` - RND Vx, byte
//! * `Dxyn` - DRW Vx, Vy, nibble
//! * `Ex9E` - SKP Vx
//! * `ExA1` - SKNP Vx
//! * `Fx07` - LD Vx, DT
//! * `Fx0A` - LD Vx, K
//! * `Fx15` - LD DT, Vx
//! * `Fx18` - LD ST, Vx
//! * `Fx1E` - ADD I, Vx
//! * `Fx29` - LD F, Vx
//! * `Fx33` - LD B, Vx
//! * `Fx55` - LD [I], Vx
//! * `Fx65` - LD Vx, [I]

/// Describes a type that can visit chip-8 instructions.
pub trait InsnVisit {
  type Result;

  fn nop(&mut self) -> Self::Result;
  fn cls(&mut self) -> Self::Result;
  fn ret(&mut self) -> Self::Result;
  fn sys_nnn(&mut self, nnn: u16) -> Self::Result;
  fn jp_nnn(&mut self, nnn: u16) -> Self::Result;
  fn call_nnn(&mut self, nnn: u16) -> Self::Result;
  fn se_x_kk(&mut self, x: u8, kk: u8) -> Self::Result;
  fn sne_x_kk(&mut self, x: u8, kk: u8) -> Self::Result;
  fn se_x_y(&mut self, x: u8, y: u8) -> Self::Result;
  fn ld_x_kk(&mut self, x: u8, kk: u8) -> Self::Result;
  fn add_x_kk(&mut self, x: u8, kk: u8) -> Self::Result;
  fn ld_x_y(&mut self, x: u8, y: u8) -> Self::Result;
  fn or_x_y(&mut self, x: u8, y: u8) -> Self::Result;
  fn and_x_y(&mut self, x: u8, y: u8) -> Self::Result;
  fn xor_x_y(&mut self, x: u8, y: u8) -> Self::Result;
  fn add_x_y(&mut self, x: u8, y: u8) -> Self::Result;
  fn sub_x_y(&mut self, x: u8, y: u8) -> Self::Result;
  fn shr_x_y(&mut self, x: u8, y: u8) -> Self::Result;
  fn subn_x_y(&mut self, x: u8, y: u8) -> Self::Result;
  fn shl_x_y(&mut self, x: u8, y: u8) -> Self::Result;
  fn sne_x_y(&mut self, x: u8, y: u8) -> Self::Result;
  fn ld_i_nnn(&mut self, nnn: u16) -> Self::Result;
  fn jp_0_nnn(&mut self, nnn: u16) -> Self::Result;
  fn rnd(&mut self, x: u8, kk: u8) -> Self::Result;
  fn drw(&mut self, x: u8, y: u8, n: u8) -> Self::Result;
  fn skp_x(&mut self, x: u8) -> Self::Result;
  fn sknp_x(&mut self, x: u8) -> Self::Result;
  fn ld_x_dt(&mut self, x: u8) -> Self::Result;
  fn ld_x_k(&mut self, x: u8) -> Self::Result;
  fn ld_dt_x(&mut self, x: u8) -> Self::Result;
  fn ld_st_x(&mut self, x: u8) -> Self::Result;
  fn add_i_x(&mut self, x: u8) -> Self::Result;
  fn ld_f_x(&mut self, x: u8) -> Self::Result;
  fn ld_b_x(&mut self, x: u8) -> Self::Result;
  fn ld_deref_i_x(&mut self, x: u8) -> Self::Result;
  fn ld_x_deref_i(&mut self, x: u8) -> Self::Result;

  fn invalid(&mut self, hi: u8, lo: u8) -> Self::Result;

  /// Parses an instruction from hi and lo bytes.
  #[inline]
  fn visit_insn(&mut self, hi: u8, lo: u8) -> Self::Result {
    let nnnn = (hi >> 4, hi & 0x0f, lo >> 4, lo & 0x0f);
    let nnn = ((hi & 0x0f) as u16) << 8 | (lo as u16);
    let kk = lo;
    let n = nnnn.3;
    let x = nnnn.1;
    let y = nnnn.2;

    match nnnn {
      (0x0, 0x0, 0x0, 0x0) => self.nop(),
      (0x0, 0x0, 0xE, 0x0) => self.cls(),
      (0x0, 0x0, 0xE, 0xE) => self.ret(),
      (0x0, _, _, _) => self.sys_nnn(nnn),
      (0x1, _, _, _) => self.jp_nnn(nnn),
      (0x2, _, _, _) => self.call_nnn(nnn),
      (0x3, _, _, _) => self.se_x_kk(x, kk),
      (0x4, _, _, _) => self.sne_x_kk(x, kk),
      (0x5, _, _, 0x0) => self.se_x_y(x, y),
      (0x6, _, _, _) => self.ld_x_kk(x, kk),
      (0x7, _, _, _) => self.add_x_kk(x, kk),
      (0x8, _, _, 0x0) => self.ld_x_y(x, y),
      (0x8, _, _, 0x1) => self.or_x_y(x, y),
      (0x8, _, _, 0x2) => self.and_x_y(x, y),
      (0x8, _, _, 0x3) => self.xor_x_y(x, y),
      (0x8, _, _, 0x4) => self.add_x_y(x, y),
      (0x8, _, _, 0x5) => self.sub_x_y(x, y),
      (0x8, _, _, 0x6) => self.shr_x_y(x, y),
      (0x8, _, _, 0x7) => self.subn_x_y(x, y),
      (0x8, _, _, 0xE) => self.shl_x_y(x, y),
      (0x9, _, _, 0x0) => self.sne_x_y(x, y),
      (0xA, _, _, _) => self.ld_i_nnn(nnn),
      (0xB, _, _, _) => self.jp_0_nnn(nnn),
      (0xC, _, _, _) => self.rnd(x, kk),
      (0xD, _, _, _) => self.drw(x, y, n),
      (0xE, _, 0x9, 0xE) => self.skp_x(x),
      (0xE, _, 0xA, 0x1) => self.sknp_x(x),
      (0xF, _, 0x0, 0x7) => self.ld_x_dt(x),
      (0xF, _, 0x0, 0xA) => self.ld_x_k(x),
      (0xF, _, 0x1, 0x5) => self.ld_dt_x(x),
      (0xF, _, 0x1, 0x8) => self.ld_st_x(x),
      (0xF, _, 0x1, 0xE) => self.add_i_x(x),
      (0xF, _, 0x2, 0x9) => self.ld_f_x(x),
      (0xF, _, 0x3, 0x3) => self.ld_b_x(x),
      (0xF, _, 0x5, 0x5) => self.ld_deref_i_x(x),
      (0xF, _, 0x6, 0x5) => self.ld_x_deref_i(x),
      _ => self.invalid(hi, lo),
    }
  }
}
