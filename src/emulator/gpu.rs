use std::io::{Result, Error};

use graphics::{Context, Graphics};
use std::rc::Rc;
use std::cell::RefCell;
use crate::emulator::bus::Bus;

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub type GpuRef = Rc<RefCell<Gpu>>;

pub struct Gpu {
  bus: Rc<RefCell<Bus>>,
  pub display: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
}

impl Gpu {
  pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
    Self {
      bus,
      display: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    }
  }

  pub fn clear(&mut self) {
    self.display = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
  }

  pub fn draw<G: Graphics>(&self, context: Context, gfx: &mut G) {
    use graphics::*;

    let mut pixel_size = 0f64;
    let mut pixel_viewport_x = 0f64;
    let mut pixel_viewport_y = 0f64;
    let mut pixel_viewport_width = 0f64;
    let mut pixel_viewport_height = 0f64;

    if let Some(viewport) = context.viewport {
      let window_width = viewport.window_size[0];
      let window_height = viewport.window_size[1];
      let display_width = (DISPLAY_WIDTH - 1) as f64;
      let display_height = (DISPLAY_HEIGHT - 1) as f64;

      pixel_size = f64::min(
        window_width / display_width,
        window_height / display_height,
      );

      pixel_viewport_width = pixel_size * display_width;
      pixel_viewport_height = pixel_size * display_height;
      pixel_viewport_x = (window_width - pixel_viewport_width) / 2.0;
      pixel_viewport_y = (window_height - pixel_viewport_height) / 2.0;
    } else {
      panic!("No viewport");
      return;
    }

    rectangle(
      [0.1, 0.1, 0.1, 1.0],
      [
        pixel_viewport_x,
        pixel_viewport_y,
        pixel_viewport_width,
        pixel_viewport_height
      ],
      context.transform,
      gfx
    );

    for y in 0..DISPLAY_HEIGHT - 1 {
      for x in 0..DISPLAY_WIDTH - 1 {
        let pixel_x = pixel_viewport_x + pixel_size * (x as f64);
        let pixel_y = pixel_viewport_y + pixel_size * (y as f64);

        if !self.display[y][x] {
          continue;
        }

        rectangle(
          [0.0, 1.0, 0.0, 0.7],
          [
            pixel_viewport_x + (pixel_size * x as f64),
            pixel_viewport_y + (pixel_size * y as f64),
            pixel_size,
            pixel_size,
          ],
          context.transform,
          gfx,
        )
      }
    }
  }
}
