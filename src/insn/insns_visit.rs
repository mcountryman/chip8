//! Visitor trait implementation for [Insn].

use super::{insns::*, visit::InsnVisit, Insn};

/// A [Insn] visitor.
pub struct InsnVisitor;

impl InsnVisit for InsnVisitor {
  type Result = Option<Insn>;

  fn nop(&mut self) -> Self::Result {
    Some(Insn::Nop)
  }

  fn cls(&mut self) -> Self::Result {
    Some(ClsInsn.into())
  }

  fn ret(&mut self) -> Self::Result {
    Some(RetInsn.into())
  }

  fn sys_nnn(&mut self, nnn: u16) -> Self::Result {
    Some(SysInsn { addr: nnn }.into())
  }

  fn jp_nnn(&mut self, nnn: u16) -> Self::Result {
    Some(JpInsn::Addr(nnn).into())
  }

  fn call_nnn(&mut self, nnn: u16) -> Self::Result {
    Some(CallInsn { addr: nnn }.into())
  }

  fn se_x_kk(&mut self, x: u8, kk: u8) -> Self::Result {
    Some(SeInsn::RegVal(x, kk).into())
  }

  fn sne_x_kk(&mut self, x: u8, kk: u8) -> Self::Result {
    Some(SneInsn::RegVal(x, kk).into())
  }

  fn se_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    Some(SeInsn::RegReg(x, y).into())
  }

  fn ld_x_kk(&mut self, x: u8, kk: u8) -> Self::Result {
    Some(LdInsn::RegVal(x, kk).into())
  }

  fn add_x_kk(&mut self, x: u8, kk: u8) -> Self::Result {
    Some(AddInsn::RegVal(x, kk).into())
  }

  fn ld_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    Some(LdInsn::RegReg(x, y).into())
  }

  fn or_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    Some(OrInsn { x, y }.into())
  }

  fn and_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    Some(AndInsn { x, y }.into())
  }

  fn xor_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    Some(XorInsn { x, y }.into())
  }

  fn add_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    Some(AddInsn::RegReg(x, y).into())
  }

  fn sub_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    Some(SubInsn { x, y }.into())
  }

  fn shr_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    Some(ShrInsn { x, y }.into())
  }

  fn subn_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    Some(SubNInsn { x, y }.into())
  }

  fn shl_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    Some(ShlInsn { x, y }.into())
  }

  fn sne_x_y(&mut self, x: u8, y: u8) -> Self::Result {
    Some(SneInsn::RegReg(x, y).into())
  }

  fn ld_i_nnn(&mut self, nnn: u16) -> Self::Result {
    Some(LdInsn::MemAddr(nnn).into())
  }

  fn jp_0_nnn(&mut self, nnn: u16) -> Self::Result {
    Some(JpInsn::AddrReg(nnn).into())
  }

  fn rnd(&mut self, x: u8, kk: u8) -> Self::Result {
    Some(RndInsn { x, val: kk }.into())
  }

  fn drw(&mut self, x: u8, y: u8, n: u8) -> Self::Result {
    Some(DrwInsn { x, y, n }.into())
  }

  fn skp_x(&mut self, x: u8) -> Self::Result {
    Some(SkpInsn { x }.into())
  }

  fn sknp_x(&mut self, x: u8) -> Self::Result {
    Some(SkpNpInsn { x }.into())
  }

  fn ld_x_dt(&mut self, x: u8) -> Self::Result {
    Some(LdInsn::RegDt(x).into())
  }

  fn ld_x_k(&mut self, x: u8) -> Self::Result {
    Some(LdInsn::RegKey(x).into())
  }

  fn ld_dt_x(&mut self, x: u8) -> Self::Result {
    Some(LdInsn::DtReg(x).into())
  }

  fn ld_st_x(&mut self, x: u8) -> Self::Result {
    Some(LdInsn::StReg(x).into())
  }

  fn add_i_x(&mut self, x: u8) -> Self::Result {
    Some(AddInsn::MemReg(x).into())
  }

  fn ld_f_x(&mut self, x: u8) -> Self::Result {
    Some(LdInsn::SpriteReg(x).into())
  }

  fn ld_b_x(&mut self, x: u8) -> Self::Result {
    Some(LdInsn::BcdReg(x).into())
  }

  fn ld_deref_i_x(&mut self, x: u8) -> Self::Result {
    Some(LdInsn::PtrReg(x).into())
  }

  fn ld_x_deref_i(&mut self, x: u8) -> Self::Result {
    Some(LdInsn::RegPtr(x).into())
  }

  fn invalid(&mut self, _: u8, _: u8) -> Self::Result {
    None
  }
}
