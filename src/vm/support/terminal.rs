//! Terminal graphics and keyboard support.

use super::{
  terminal_debug::render_debug_widgets, terminal_game::TerminalGameWidget, Support,
};
use crate::vm::{error::VmError, state::VmState, Vm};
use crossterm::{
  cursor::Show,
  event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
  execute,
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
  collections::HashMap,
  error::Error,
  io::{stdout, Stdout},
  process,
  time::{Duration, Instant},
};
use tui::{
  backend::CrosstermBackend,
  layout::{Constraint, Direction, Layout},
};

/// Terminal graphics and keyboard support.
pub struct Terminal {
  keys: [bool; 0x10],
  key_map: HashMap<KeyCode, u8>,
  // todo: use a [u16; 32] to be a cool kid.
  vram: Vec<bool>,
  vram_width: usize,
  vram_height: usize,

  terminal: tui::Terminal<CrosstermBackend<Stdout>>,
}

impl Terminal {
  /// Create a [Terminal].
  pub fn new() -> Result<Self, Box<dyn Error>> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = tui::Terminal::new(backend)?;
    let terminal = Self::from_terminal(terminal);

    std::panic::set_hook(Box::new(|err| {
      let mut stdout = std::io::stdout();
      terminal::disable_raw_mode().ok();
      execute!(stdout, LeaveAlternateScreen, DisableMouseCapture, Show).ok();

      eprintln!("{}", err);
    }));

    Ok(terminal)
  }

  /// Create a [Terminal] from [tui::Terminal].
  pub fn from_terminal(terminal: tui::Terminal<CrosstermBackend<Stdout>>) -> Self {
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
    key_map.insert(KeyCode::Char('q'), 0xa);
    key_map.insert(KeyCode::Char('w'), 0xb);
    key_map.insert(KeyCode::Char('e'), 0xc);
    key_map.insert(KeyCode::Char('a'), 0xd);
    key_map.insert(KeyCode::Char('s'), 0xe);
    key_map.insert(KeyCode::Char('d'), 0xf);

    Self {
      keys: [false; 0x10],
      key_map,
      vram: vec![false; 64 * 32],
      vram_width: 64,
      vram_height: 32,

      terminal,
    }
  }

  pub fn update(
    &mut self,
    state: &VmState,
    mut timeout: Duration,
  ) -> Result<(), Box<dyn Error>> {
    self.terminal.draw(|ui| {
      let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(ui.size());

      ui.render_widget(
        TerminalGameWidget::new(self.vram_width, self.vram_height, &self.vram),
        chunks[0],
      );

      render_debug_widgets(state, chunks[1], ui);
    })?;

    self.keys = [false; 0x10];

    let mut paused = false;

    while paused || !timeout.is_zero() {
      let time = Instant::now();
      let poll = Duration::from_millis(150.max(timeout.as_millis() as _));

      if event::poll(poll)? {
        if let Event::Key(key) = event::read()? {
          if key.code == KeyCode::Esc
            || key.code == KeyCode::Char('c')
              && key.modifiers.contains(KeyModifiers::CONTROL)
          {
            self.reset();
            process::exit(0);
          }

          if key.code == KeyCode::Enter {
            paused = !paused;
          }

          if let Some(code) = self.key_map.get(&key.code) {
            self.keys[*code as usize] = true;
          }
        }
      }

      timeout = Duration::from_millis(
        timeout
          .as_millis()
          .saturating_sub(time.elapsed().as_millis()) as _,
      );
    }

    Ok(())
  }

  /// Resets terminal to default state.
  pub fn reset(&mut self) {
    terminal::disable_raw_mode().ok();
    execute!(
      self.terminal.backend_mut(),
      LeaveAlternateScreen,
      DisableMouseCapture
    )
    .ok();

    self.terminal.show_cursor().ok();
  }
}

impl Support for Terminal {
  fn clear(&mut self) -> Result<(), VmError> {
    self.vram = vec![false; self.vram.len()];
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
