//! Display trait implementations for instructions.

use super::insns::*;
use std::fmt::Display;

impl Display for ClsInsn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "CLS")
  }
}

impl Display for RetInsn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "RET")
  }
}

impl Display for SysInsn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "SYS {:#x}", self.addr)
  }
}

impl Display for JpInsn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Addr(addr) => write!(f, "JP {addr:#x}"),
      Self::AddrReg(addr) => write!(f, "JP V0, {addr:#x}"),
    }
  }
}

impl Display for CallInsn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "CALL {:#x}", self.addr)
  }
}

impl Display for SeInsn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::RegVal(vx, val) => write!(f, "SE V{vx}, {val:#x}"),
      Self::RegReg(vx, vy) => write!(f, "SE V{vx}, V{vy}"),
    }
  }
}

impl Display for SneInsn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::RegVal(vx, val) => write!(f, "SNE V{vx}, {val:#x}"),
      Self::RegReg(vx, vy) => write!(f, "SNE V{vx}, V{vy}"),
    }
  }
}

impl Display for LdInsn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::RegVal(x, val) => write!(f, "LD V{x}, {val:#x}"),
      Self::RegReg(x, y) => write!(f, "LD {x}, {y}"),
      Self::MemAddr(x) => write!(f, "LD I, {x:#x}"),
      Self::RegDt(x) => write!(f, "LD V{x}, DT"),
      Self::RegKey(x) => write!(f, "LD V{x}, K"),
      Self::DtReg(x) => write!(f, "LD DT, V{x}"),
      Self::StReg(x) => write!(f, "LD ST, V{x}"),
      Self::SpriteReg(x) => write!(f, "LD F, V{x}"),
      Self::BcdReg(x) => write!(f, "LD B, V{x}"),
      Self::PtrReg(x) => write!(f, "LD [I], V{x}"),
      Self::RegPtr(x) => write!(f, "LD V{x}, [I]"),
    }
  }
}

impl Display for AddInsn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::RegVal(vx, val) => write!(f, "ADD V{vx}, {val:#x}"),
      Self::RegReg(vx, vy) => write!(f, "ADD V{vx}, V{vy}"),
      Self::MemReg(vx) => write!(f, "ADD I, V{vx}"),
    }
  }
}

impl Display for OrInsn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "OR V{}, V{}", self.x, self.y)
  }
}

impl Display for AndInsn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "AND V{}, V{}", self.x, self.y)
  }
}

impl Display for XorInsn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "XOR V{}, V{}", self.x, self.y)
  }
}

impl Display for SubInsn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "SUB V{}, V{}", self.x, self.y)
  }
}

impl Display for SubNInsn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "SUBN V{}, V{}", self.x, self.y)
  }
}

impl Display for ShrInsn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "SHR V{}, V{}", self.x, self.y)
  }
}

impl Display for ShlInsn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "SHL V{}, V{}", self.x, self.y)
  }
}

impl Display for RndInsn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "RND V{}, {:#x}", self.x, self.val)
  }
}

impl Display for DrwInsn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "DRW V{}, V{}, {}", self.x, self.y, self.n)
  }
}

impl Display for SkpInsn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "SKP V{}", self.x)
  }
}

impl Display for SkpNpInsn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "SKNP V{}", self.x)
  }
}
