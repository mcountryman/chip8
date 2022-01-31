//! Conversion trait implementations for instructions.

use super::{insns::*, Insn};

impl From<ClsInsn> for Insn {
  fn from(insn: ClsInsn) -> Self {
    Self::Cls(insn)
  }
}

impl From<RetInsn> for Insn {
  fn from(insn: RetInsn) -> Self {
    Self::Ret(insn)
  }
}

impl From<SysInsn> for Insn {
  fn from(insn: SysInsn) -> Self {
    Self::Sys(insn)
  }
}

impl From<JpInsn> for Insn {
  fn from(insn: JpInsn) -> Self {
    Self::Jp(insn)
  }
}

impl From<CallInsn> for Insn {
  fn from(insn: CallInsn) -> Self {
    Self::Call(insn)
  }
}

impl From<SeInsn> for Insn {
  fn from(insn: SeInsn) -> Self {
    Self::Se(insn)
  }
}

impl From<SneInsn> for Insn {
  fn from(insn: SneInsn) -> Self {
    Self::SNe(insn)
  }
}

impl From<LdInsn> for Insn {
  fn from(insn: LdInsn) -> Self {
    Self::Ld(insn)
  }
}

impl From<AddInsn> for Insn {
  fn from(insn: AddInsn) -> Self {
    Self::Add(insn)
  }
}

impl From<OrInsn> for Insn {
  fn from(insn: OrInsn) -> Self {
    Self::Or(insn)
  }
}

impl From<AndInsn> for Insn {
  fn from(insn: AndInsn) -> Self {
    Self::And(insn)
  }
}

impl From<XorInsn> for Insn {
  fn from(insn: XorInsn) -> Self {
    Self::Xor(insn)
  }
}

impl From<SubInsn> for Insn {
  fn from(insn: SubInsn) -> Self {
    Self::Sub(insn)
  }
}

impl From<ShrInsn> for Insn {
  fn from(insn: ShrInsn) -> Self {
    Self::Shr(insn)
  }
}

impl From<SubNInsn> for Insn {
  fn from(insn: SubNInsn) -> Self {
    Self::SubN(insn)
  }
}

impl From<ShlInsn> for Insn {
  fn from(insn: ShlInsn) -> Self {
    Self::Shl(insn)
  }
}

impl From<RndInsn> for Insn {
  fn from(insn: RndInsn) -> Self {
    Self::Rnd(insn)
  }
}

impl From<DrwInsn> for Insn {
  fn from(insn: DrwInsn) -> Self {
    Self::Drw(insn)
  }
}

impl From<SkpInsn> for Insn {
  fn from(insn: SkpInsn) -> Self {
    Self::Skp(insn)
  }
}

impl From<SkpNpInsn> for Insn {
  fn from(insn: SkpNpInsn) -> Self {
    Self::SkpNp(insn)
  }
}
