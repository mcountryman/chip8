//! Graphics and keyboard support.

pub mod terminal;
pub mod terminal_debug;
pub mod terminal_game;
pub use terminal::*;

use super::error::VmError;

/// Describes a virtual machine graphics and keyboard support type.
pub trait Support {
  /// Clears the graphics display.
  fn clear(&mut self) -> Result<(), VmError>;

  /// Draws a sprite at the specified coordinates and gets a value indicating whether the
  /// sprite collided with already drawn pixels.
  fn draw(&mut self, sprite: &[u8], x: u8, y: u8) -> Result<bool, VmError>;

  /// Gets a value indicating if the given hexadecimal key is currently pressed.
  fn is_key_pressed(&self, key: u8) -> Result<bool, VmError>;

  /// Waits until a key is pressed and returns the hexadecimal key code.
  fn wait_key_pressed(&mut self) -> Result<u8, VmError>;
}
