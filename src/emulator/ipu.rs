use std::collections::HashMap;
use std::iter::Map;

use glium::backend::glutin::glutin::{KeyboardInput, VirtualKeyCode, ElementState};
use std::thread::sleep;
use std::time::Duration;

pub struct Ipu {
  keys: HashMap<u8, bool>,

  last_key: u8,
  is_waiting: bool,
}

impl Ipu {
  pub fn new() -> Self {
    Self {
      keys: HashMap::new(),
      last_key: 0,
      is_waiting: false,
    }
  }

  pub fn wait_key(&mut self) -> u8 {
    self.is_waiting = true;

    while self.is_waiting {
      sleep(Duration::from_micros(10));
    }

    self.last_key
  }

  pub fn is_key_pressed(&self, key: u8) -> bool {
    return self.keys.contains_key(&key) && self.keys[&key];
  }

  pub fn process_input(&mut self, input: &KeyboardInput) {
    let key = match input.virtual_keycode {
      Some(VirtualKeyCode::Numpad0) => Some(0x0),
      Some(VirtualKeyCode::Numpad1) => Some(0x1),
      Some(VirtualKeyCode::Numpad2) => Some(0x2),
      Some(VirtualKeyCode::Numpad3) => Some(0x3),
      Some(VirtualKeyCode::Numpad4) => Some(0x4),
      Some(VirtualKeyCode::Numpad5) => Some(0x5),
      Some(VirtualKeyCode::Numpad6) => Some(0x6),
      Some(VirtualKeyCode::Numpad7) => Some(0x7),
      Some(VirtualKeyCode::Numpad8) => Some(0x8),
      Some(VirtualKeyCode::Numpad9) => Some(0x9),
      Some(VirtualKeyCode::Key0) => Some(0x0),
      Some(VirtualKeyCode::Key1) => Some(0x1),
      Some(VirtualKeyCode::Key2) => Some(0x2),
      Some(VirtualKeyCode::Key3) => Some(0x3),
      Some(VirtualKeyCode::Key4) => Some(0x4),
      Some(VirtualKeyCode::Key5) => Some(0x5),
      Some(VirtualKeyCode::Key6) => Some(0x6),
      Some(VirtualKeyCode::Key7) => Some(0x7),
      Some(VirtualKeyCode::Key8) => Some(0x8),
      Some(VirtualKeyCode::Key9) => Some(0x9),
      Some(VirtualKeyCode::A) => Some(0xA),
      Some(VirtualKeyCode::B) => Some(0xB),
      Some(VirtualKeyCode::C) => Some(0xC),
      Some(VirtualKeyCode::D) => Some(0xD),
      Some(VirtualKeyCode::E) => Some(0xE),
      Some(VirtualKeyCode::F) => Some(0xF),
      _ => None,
    };

    if let Some(key) = key {
      let state = self.keys.entry(key).or_insert(false);
      *state = input.state == ElementState::Pressed;

      if input.state == ElementState::Pressed {
        self.last_key = key;
        self.is_waiting = false;
      }
    }
  }
}
