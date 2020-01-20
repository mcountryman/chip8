use std::cell::RefCell;
use std::io::Result;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{Events, EventSettings};
use piston::input::{Button, ButtonEvent, RenderEvent, UpdateEvent, Key, ButtonState};
use piston::window::WindowSettings;

use crate::emulator::Emulator;
use crate::emulator::gpu::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use graphics::clear;
use nfd::Response;
use std::env;

mod emulator;

fn main() -> Result<()> {
  let opengl = OpenGL::V3_2;
  let args: Vec<String> = env::args().collect();
  let mut emulator = Emulator::new();
  let mut window: GlutinWindow = WindowSettings::new("chip8", [550, 250])
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .expect("Failed to create window");

  let mut gl = GlGraphics::new(opengl);
  let mut events = Events::new(EventSettings::new());

  if args.len() > 1 {
    emulator.load(args[1].clone())?;
  }

  while let Some(event) = events.next(&mut window) {
    if let Some(args) = event.render_args() {
      gl.draw(args.viewport(), |context, gfx| {
        clear([0.0, 0.0, 0.0, 0.0], gfx);

        if let Some(ref gpu) = *emulator.bus.borrow().gpu.borrow() {
          gpu.draw(context, gfx);
        }
      });
      // app.render(&args)
    }

    if let Some(args) = event.button_args() {
      if let Button::Keyboard(key) = args.button {
        if let Some(ref mut ipu) = *emulator.bus.borrow().ipu.borrow_mut() {
          ipu.process_input(key, args.state);
        }

        if key == Key::O && args.state == ButtonState::Press {
          let result = nfd::open_file_dialog(None, None)
            .unwrap();

          match result {
            Response::Okay(path) => {
              emulator.load(path).unwrap();
            },
            _ => {},
          }
        }
        // key input
      }
    }

    if let Some(args) = event.update_args() {
      if let Some(ref mut cpu) = *emulator.bus.borrow().cpu.borrow_mut() {
        cpu.tick()?;
      }
    }
  }

  Ok(())
}
