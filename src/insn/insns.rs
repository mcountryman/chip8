//! Instruction definitions.

/// An instruction that should clear the display.
pub struct ClsInsn;

/// An instruction that should return from a subroutine.
pub struct RetInsn;

/// An instruction that should jump to a given address.
pub enum JmpInsn {
  /// Jump to address.
  Addr(u16),
  /// Jump to address offset by value in supplied register.
  AddrReg(u8, u16),
}

/// An instruction that should skip the next instruction if values are equal.
pub enum SkpEqInsn {
  /// Skip next instruction if value in register and constant are equal.
  RegByte(u8, u8),
  /// Skips next instruction if value in first register and second register are equal.
  RegReg(u8, u8),
}

/// An instruction that should skip the next instruction if values are not equal.
pub enum SkpNeInsn {
  /// Skip next instruction if value in register and constant are not equal.
  RegByte(u8, u8),
  /// Skips next instruction if value in first register and second register are not equal.
  RegReg(u8, u8),
}
