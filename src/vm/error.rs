//! Virtual machine errors.

/// A virtual machine error.
#[derive(Debug)]
pub enum VmError {
  /// An io error occurred.
  Io(std::io::Error),
  /// An instruction attempted to access an invalid register.
  BadReg(u8),
  /// An instruction invalid instruction was found.
  BadInsn(u16),
  /// An instruction attempted to underflow the stack.
  StackUnderflow,
  /// An instruction attempted to overflow the stack.
  StackOverflow,
}

impl From<std::io::Error> for VmError {
  fn from(err: std::io::Error) -> Self {
    Self::Io(err)
  }
}

impl std::fmt::Display for VmError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl std::error::Error for VmError {}
