//! Virtual machine errors.

/// A virtual machine error.
pub enum VmError {
  /// An io error occurred.
  Io(std::io::Error),
  /// An instruction attempted to access an invalid register.
  BadReg(u8),
  /// An instruction invalid instruction was found.
  BadInsn(u16),
  /// An instruction attempted to overflow the stack.
  StackOverflow,
  /// An instruction attempted to underflow the stack.
  StackUnderflow,
  /// A program is too large to fit in memory.
  BadProgramTooLarge(usize, usize),
}

impl From<std::io::Error> for VmError {
  fn from(err: std::io::Error) -> Self {
    Self::Io(err)
  }
}

impl std::fmt::Debug for VmError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Io(err) => write!(f, "{}", err),
      Self::BadReg(reg) => write!(f, "Bad register `{reg}`"),
      Self::BadInsn(insn) => write!(f, "Bad instruction `{insn:#x}`"),
      Self::StackOverflow => write!(f, "Stack overflow"),
      Self::StackUnderflow => write!(f, "Stack underflow"),

      Self::BadProgramTooLarge(act, exp) => {
        write!(f, "Program size `{act}` > `{exp}`")
      }
    }
  }
}

impl std::fmt::Display for VmError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl std::error::Error for VmError {}
