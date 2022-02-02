use eyre::Result;
use std::{
  env, fs,
  time::{Duration, Instant},
};
use ui::Ui;
use vm::Vm;

pub mod insn;
pub mod ui;
pub mod vm;

fn main() -> Result<()> {
  let mut vm = Vm::new();
  let mut ui = Ui::new()?;

  let program = env::args().nth(1).expect("Expected program path");
  let program = fs::read(program)?;

  vm.load_program(&program)?;

  ui.paused = env::args()
    .nth(2)
    .map(|arg| match arg.split_once("--is-paused=") {
      Some((_, flag)) => flag.starts_with('t') || flag.starts_with('T'),
      None => false,
    })
    .unwrap_or_default();

  let rate_cpu = Duration::from_secs_f64(1. / 500.);
  let rate_timers = Duration::from_secs_f64(1. / 60.);
  let mut clock_cpu = Instant::now();
  let mut clock_timers = Instant::now();

  loop {
    if !ui.paused {
      if clock_cpu.elapsed() >= rate_cpu {
        clock_cpu = Instant::now();
        vm.update()?;
      }

      if clock_timers.elapsed() >= rate_timers {
        clock_timers = Instant::now();
        vm.update_timers();
      }
    }

    ui.update(&mut vm)?;
  }
}
