use vm::{support::Terminal, Vm};

pub mod insn;
pub mod vm;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let terminal = Terminal::new()?;
  let mut vm = Vm::new(terminal);

  let program = if std::env::args().count() < 2 {
    include_bytes!("../data/TEST").to_vec()
  } else {
    let program = std::env::args().nth(1).expect("Expected at least one arg.");

    std::fs::read(program)?
  };

  vm.load_program(&program[..])?;

  loop {
    vm.update()?;
  }

  Ok(())
}
