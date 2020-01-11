#[macro_use]
extern crate imgui;

use std::io::Result;

use glium::glutin::{Event, WindowEvent};
use imgui::{Condition, Window};
use imgui::NavInput::Menu;

use crate::emulator::Emulator;
use crate::support::App;

mod support;
mod emulator;

fn main() -> Result<()> {
  let mut emulator = Emulator::new();
  let mut app = App::create(
    "chip8",
    640.0,
    380.0
  )?;

  let mut show_metrics = false;

  app
    .on_ui(|ui| {
      // Menu end w/margin = 30


    })
    .on_event(|event| {
      if let Event::WindowEvent { event, .. } = event {
        if let WindowEvent::KeyboardInput { input, .. } = event {
          // do shit
        }
      }
    });

  app.run()?;

  Ok(())
}
