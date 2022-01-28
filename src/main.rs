use std::time::Duration;
use vm::{support::Terminal, Vm};

pub mod insn;
pub mod vm;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let terminal = Terminal::default();
  let mut vm = Vm::new(terminal);

  let program = std::env::args().next().expect("Expected at least one arg.");
  let program = std::fs::read(program)?;

  vm.load_program(&program)?;

  loop {
    vm.update()?;
    vm.support().render()?;
    vm.support_mut().update(Duration::from_millis(150))?;
  }

  Ok(())
}
