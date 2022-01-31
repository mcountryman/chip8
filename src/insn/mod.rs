//! Instruction parsing.

use self::{
  insns::{
    AddInsn, AndInsn, CallInsn, ClsInsn, DrwInsn, JpInsn, LdInsn, OrInsn, RetInsn,
    RndInsn, SeInsn, ShlInsn, ShrInsn, SkpInsn, SkpNpInsn, SneInsn, SubInsn, SubNInsn,
    SysInsn, XorInsn,
  },
  insns_visit::InsnVisitor,
  visit::InsnVisit,
};
use std::fmt::Display;

pub mod insns;
pub mod insns_display;
pub mod insns_into;
pub mod insns_visit;
pub mod visit;

/// An instruction.
#[derive(Clone, Copy)]
pub enum Insn {
  Nop,
  Cls(ClsInsn),
  Ret(RetInsn),
  Sys(SysInsn),
  Jp(JpInsn),
  Call(CallInsn),
  Se(SeInsn),
  SNe(SneInsn),
  Ld(LdInsn),
  Add(AddInsn),
  Or(OrInsn),
  And(AndInsn),
  Xor(XorInsn),
  Sub(SubInsn),
  Shr(ShrInsn),
  SubN(SubNInsn),
  Shl(ShlInsn),
  Rnd(RndInsn),
  Drw(DrwInsn),
  Skp(SkpInsn),
  SkpNp(SkpNpInsn),
}

impl Insn {
  /// Creates an instruction from bytes.
  pub fn from_bytes(hi: u8, lo: u8) -> Option<Self> {
    InsnVisitor.visit_insn(hi, lo)
  }
}

impl Display for Insn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Nop => write!(f, "NOP"),
      Self::Cls(insn) => write!(f, "{insn}"),
      Self::Ret(insn) => write!(f, "{insn}"),
      Self::Sys(insn) => write!(f, "{insn}"),
      Self::Jp(insn) => write!(f, "{insn}"),
      Self::Call(insn) => write!(f, "{insn}"),
      Self::Se(insn) => write!(f, "{insn}"),
      Self::SNe(insn) => write!(f, "{insn}"),
      Self::Ld(insn) => write!(f, "{insn}"),
      Self::Add(insn) => write!(f, "{insn}"),
      Self::Or(insn) => write!(f, "{insn}"),
      Self::And(insn) => write!(f, "{insn}"),
      Self::Xor(insn) => write!(f, "{insn}"),
      Self::Sub(insn) => write!(f, "{insn}"),
      Self::Shr(insn) => write!(f, "{insn}"),
      Self::SubN(insn) => write!(f, "{insn}"),
      Self::Shl(insn) => write!(f, "{insn}"),
      Self::Rnd(insn) => write!(f, "{insn}"),
      Self::Drw(insn) => write!(f, "{insn}"),
      Self::Skp(insn) => write!(f, "{insn}"),
      Self::SkpNp(insn) => write!(f, "{insn}"),
    }
  }
}
