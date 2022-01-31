//! Instruction definitions.

/// Instruction that clears the display.
#[derive(Clone, Copy)]
pub struct ClsInsn;

/// Instruction that returns from a subroutine.
#[derive(Clone, Copy)]
pub struct RetInsn;

/// Instruction that jumps to machine code routine at specified address.
#[derive(Clone, Copy)]
pub struct SysInsn {
  pub addr: u16,
}

/// Instruction that jumps to a subroutine routine at specified address.
#[derive(Clone, Copy)]
pub struct CallInsn {
  pub addr: u16,
}

/// Instruction that jumps to an instruction at specified address.
#[derive(Clone, Copy)]
pub enum JpInsn {
  /// Jump to specified address.
  Addr(u16),
  /// Jump to specified address, offset by value in register V0.
  AddrReg(u16),
}

/// Instruction that skips next instruction if values are equal.
#[derive(Clone, Copy)]
pub enum SeInsn {
  /// Skip next instruction if value in register Vx and constant are equal.
  RegVal(u8, u8),
  /// Skips next instruction if value in register Vx and register VY are equal.
  RegReg(u8, u8),
}

/// Instruction that skips next instruction if values are not equal.
#[derive(Clone, Copy)]
pub enum SneInsn {
  /// Skip next instruction if value in register Vx and constant are equal.
  RegVal(u8, u8),
  /// Skips next instruction if value in register Vx and register VY are equal.
  RegReg(u8, u8),
}

/// Instruction that loads values from source.
#[derive(Clone, Copy)]
pub enum LdInsn {
  /// Loads constant into register Vx.
  RegVal(u8, u8),
  /// Loads value from register Vy into register Vx.
  RegReg(u8, u8),
  /// Loads constant address into register I.
  MemAddr(u16),
  /// Loads value from delay timer register into register Vx.
  RegDt(u8),
  /// Waits for key press and stores key code into register Vx.
  RegKey(u8),
  /// Loads value from register into delay timer register.
  DtReg(u8),
  /// Loads value from register into sound timer register.
  StReg(u8),
  /// Loads the location of sprite for digit value in register Vx into I register.
  SpriteReg(u8),
  /// Loads the BCD representation of Vx in memory locations I, I+1, and I+2.
  BcdReg(u8),
  /// Loads values from memory location I into registers V0 through Vx.
  PtrReg(u8),
  /// Loads values from registers V0 through Vx into memory location I.
  RegPtr(u8),
}

/// Instruction that adds values.
#[derive(Clone, Copy)]
pub enum AddInsn {
  /// Adds value in register Vx to constant and stores result in Vx.
  RegVal(u8, u8),
  /// Adds value in register Vx to value in register Vy and stores result in Vx.
  RegReg(u8, u8),
  /// Adds value in register I to value in register Vx and stores result in I.
  MemReg(u8),
}

/// Instruction that performs OR operation on values in registers Vx and Vy.
#[derive(Clone, Copy)]
pub struct OrInsn {
  pub x: u8,
  pub y: u8,
}

/// Instruction that performs AND operation on values in registers Vx and Vy.
#[derive(Clone, Copy)]
pub struct AndInsn {
  pub x: u8,
  pub y: u8,
}

/// Instruction that performs XOR operation on values in registers Vx and Vy.
#[derive(Clone, Copy)]
pub struct XorInsn {
  pub x: u8,
  pub y: u8,
}

/// Instruction that subtracts value in register Vx from value in register Vy and stores
/// result in Vx.
#[derive(Clone, Copy)]
pub struct SubInsn {
  pub x: u8,
  pub y: u8,
}

/// Instruction that shifts value in register Vx left by value in register Vy and stores
/// result in Vx.
#[derive(Clone, Copy)]
pub struct ShrInsn {
  pub x: u8,
  pub y: u8,
}

/// Instruction that subtracts value in register Vy from value in register Vx and stores
/// result in Vx.
#[derive(Clone, Copy)]
pub struct SubNInsn {
  pub x: u8,
  pub y: u8,
}

/// Instruction that shifts value in register Vx right by value in register Vy and stores
/// result in Vx.
#[derive(Clone, Copy)]
pub struct ShlInsn {
  pub x: u8,
  pub y: u8,
}

/// Instruction that performs and AND operation on value in register Vx and random byte
/// and stores result in Vx.
#[derive(Clone, Copy)]
pub struct RndInsn {
  pub x: u8,
  pub val: u8,
}

/// Instruction that displays an n-byte sprite starting at memory location I at (Vx, Vy).
#[derive(Clone, Copy)]
pub struct DrwInsn {
  pub x: u8,
  pub y: u8,
  pub n: u8,
}

/// Instruction that skips next instruction if key with the value of Vx is pressed.
#[derive(Clone, Copy)]
pub struct SkpInsn {
  pub x: u8,
}

/// Instruction that skips next instruction if key with the value of Vx is not pressed.
#[derive(Clone, Copy)]
pub struct SkpNpInsn {
  pub x: u8,
}
