#[macro_use]
extern crate imgui;

use std::io::Result;
use crate::support::App;
use imgui::{Window, Condition};

mod cpu;
mod ram;
mod support;

fn main() -> Result<()> {
  let mut app = App::create()?;

  app.on_ui(|ui| {
    Window::new(im_str!("chip8"))
      .size([300.0, 100.0], Condition::FirstUseEver)
      .build(ui, || {
        ui.text(im_str!("Emulator here"));
      });
  });

  app.run()?;

  Ok(())
}
