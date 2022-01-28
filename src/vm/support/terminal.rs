//! Terminal graphics and keyboard support.

use super::Support;
use crate::vm::error::VmError;
use crossterm::{
  cursor::{Hide, MoveRight, MoveTo, MoveUp, Show},
  event::{self, Event, KeyCode, KeyModifiers},
  queue, style,
  terminal::{self, Clear, ClearType},
};
use std::{
  collections::HashMap,
  error::Error,
  io::{stdout, Write},
  process,
  time::Duration,
};

/// Terminal graphics and keyboard support.
#[derive(Clone)]
pub struct Terminal {
  keys: [bool; 0xf],
  key_map: HashMap<KeyCode, u8>,
  // todo: use a [u16; 32] to be a cool kid.
  vram: Vec<bool>,
  vram_width: usize,
  vram_height: usize,

  term_width: usize,
  term_height: usize,
}

impl Terminal {
  pub fn render(&self) -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    let (width, height) = terminal::size()?;
    let width = width as usize;
    let height = height as usize;

    queue!(stdout, Hide)?;

    if self.term_width != width || self.term_height != height {
      queue!(stdout, Clear(ClearType::FromCursorDown))?;
    }

    for y in 0..self.vram_height {
      for x in 0..self.vram_width {
        queue!(stdout, MoveTo(x as u16, y as u16))?;

        if self.vram[y * self.vram_width + x] {
          queue!(stdout, style::Print("█"))?;
        } else {
          queue!(stdout, style::Print(" "))?;
        }
      }
    }

    queue!(
      stdout,
      MoveUp(self.vram_width as u16),
      MoveRight(self.term_width as u16),
      Show
    )?;

    stdout.flush()?;

    Ok(())
  }

  pub fn update(&mut self, timeout: Duration) -> Result<(), Box<dyn Error>> {
    self.keys = [false; 0xf];

    if event::poll(timeout)? {
      if let Event::Key(key) = event::read()? {
        if key.code == KeyCode::Esc
          || key.code == KeyCode::Char('c')
            && key.modifiers.contains(KeyModifiers::CONTROL)
        {
          process::exit(0);
        }

        if let Some(code) = self.key_map.get(&key.code) {
          self.keys[*code as usize] = true;
        }
      }
    }

    Ok(())
  }
}

impl Default for Terminal {
  fn default() -> Self {
    let mut key_map = HashMap::new();

    key_map.insert(KeyCode::Char('0'), 0x0);
    key_map.insert(KeyCode::Char('1'), 0x1);
    key_map.insert(KeyCode::Char('2'), 0x2);
    key_map.insert(KeyCode::Char('3'), 0x3);
    key_map.insert(KeyCode::Char('4'), 0x4);
    key_map.insert(KeyCode::Char('5'), 0x5);
    key_map.insert(KeyCode::Char('6'), 0x6);
    key_map.insert(KeyCode::Char('7'), 0x7);
    key_map.insert(KeyCode::Char('8'), 0x8);
    key_map.insert(KeyCode::Char('9'), 0x9);
    key_map.insert(KeyCode::Char('a'), 0xa);
    key_map.insert(KeyCode::Char('b'), 0xb);
    key_map.insert(KeyCode::Char('c'), 0xc);
    key_map.insert(KeyCode::Char('d'), 0xd);
    key_map.insert(KeyCode::Char('e'), 0xe);
    key_map.insert(KeyCode::Char('f'), 0xf);

    Self {
      keys: [false; 0xf],
      key_map,
      vram: vec![false; 64 * 32],
      vram_width: 64,
      vram_height: 32,

      term_width: 0,
      term_height: 0,
    }
  }
}

impl Support for Terminal {
  fn clear(&mut self) -> Result<(), VmError> {
    self.vram.clear();
    Ok(())
  }

  fn draw(&mut self, sprite: &[u8], x: u8, y: u8) -> Result<bool, VmError> {
    let mut collided = false;

    for (j, row) in sprite.iter().enumerate() {
      for i in 0..8 {
        let bit = (row >> (7 - i)) & 1 == 1;
        let x = (x as usize + i as usize) % self.vram_width;
        let y = (y as usize + j as usize) % self.vram_height;
        let i = x + y * self.vram_width;
        if self.vram[i] != bit {
          collided = true;
        }

        self.vram[i] = bit;
      }
    }

    Ok(collided)
  }

  fn is_key_pressed(&self, key: u8) -> Result<bool, VmError> {
    Ok(self.keys[key as usize])
  }

  fn wait_key_pressed(&mut self) -> Result<u8, VmError> {
    loop {
      if let Event::Key(key) = event::read()? {
        if let Some(code) = self.key_map.get(&key.code) {
          return Ok(*code);
        }
      }
    }
  }
}
