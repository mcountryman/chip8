use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::Map;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;
use piston::input::{Key, ButtonState};
use std::ops::Deref;
use crate::emulator::bus::Bus;

pub struct Ipu {
  bus: Rc<RefCell<Bus>>,
  keys: HashMap<u8, bool>,
  key_map: HashMap<Key, u8>,

  last_key: u8,
  pub is_waiting: bool,
}

impl Ipu {
  pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
    Self {
      bus,
      keys: HashMap::new(),
      key_map: [
        (Key::D1, 0x1),
        (Key::D2, 0x2),
        (Key::D3, 0x3),
        (Key::D4, 0xC),
        (Key::Q, 0x4),
        (Key::W, 0x5),
        (Key::E, 0x6),
        (Key::R, 0xD),
        (Key::A, 0x7),
        (Key::S, 0x8),
        (Key::D, 0x9),
        (Key::F, 0xE),
        (Key::Z, 0xA),
        (Key::X, 0x0),
        (Key::C, 0xB),
        (Key::V, 0xF),
      ].iter().cloned().collect(),

      last_key: 0,
      is_waiting: false,
    }
  }

  pub fn wait_key(&mut self) {
    if let Some(ref mut cpu) = *self.bus.borrow().cpu.borrow_mut() {
      cpu.key_waiting = true;
    }
  }

  pub fn is_key_pressed(&self, key: u8) -> bool {
    let is_down = *self.keys.get(&key).unwrap_or(&false);
    if is_down {
      println!("is_key_pressed: true");
    }

    is_down
  }

  pub fn process_input(&mut self, key: Key, state: ButtonState) {
    if let Some(key_code) = self.key_map.get(&key) {
      let is_down = state == ButtonState::Press;
      let is_down_entry = self.keys.entry(*key_code).or_insert(false);

      if *is_down_entry != is_down {
        *is_down_entry = is_down;

        if is_down {
          self.last_key = *key_code;

          if let Some(ref mut cpu) = *self.bus.borrow().cpu.borrow_mut() {
            cpu.key_code = *key_code;
            cpu.key_waiting = false;
          }

          println!("Pressed key '{:#0x}'", key_code);
        } else {
          println!("Released key '{:#0x}'", key_code);
        }
      }
    }
  }
}
