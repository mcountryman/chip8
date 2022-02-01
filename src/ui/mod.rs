//! Chip-8 terminal user interface.

pub mod debug;
pub mod game;

use self::game::Game;
use crate::vm::{flags::VmKey, Vm};
use crossterm::{
  cursor::Show,
  event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
  execute,
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use eyre::Result;
use std::{
  io::{self, Stdout},
  panic,
  time::Duration,
};
use tui::{
  backend::CrosstermBackend,
  layout::{Constraint, Direction, Layout},
  Terminal,
};

pub struct Ui {
  pub step: bool,
  pub paused: bool,

  terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Ui {
  /// Create a [Ui].
  pub fn new() -> Result<Self> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    let ui = Self {
      step: false,
      paused: false,
      terminal,
    };

    panic::set_hook(Box::new(|err| {
      cleanup();
      eprintln!("{}", err);
    }));

    Ok(ui)
  }

  pub fn update(&mut self, vm: &mut Vm) -> Result<()> {
    self.update_ui(vm)?;
    self.update_keys(vm)?;

    Ok(())
  }

  fn update_ui(&mut self, vm: &mut Vm) -> Result<()> {
    self.terminal.draw(|ui| {
      let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(ui.size());

      ui.render_widget(Game::new(&vm.vram), chunks[0]);

      let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Percentage(100)].as_ref())
        .split(chunks[1]);

      let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(48), Constraint::Min(0)].as_ref())
        .split(chunks[0]);

      debug::registers(vm, top[0], ui);
      debug::keys(vm, top[1], ui);

      let bottom = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(20)].as_ref())
        .split(chunks[1]);

      debug::disassembly(vm, bottom[0], ui);
      debug::stack(vm, bottom[1], ui);
    })?;

    Ok(())
  }

  fn update_keys(&mut self, vm: &mut Vm) -> Result<()> {
    if self.step {
      self.paused = true;
    }

    if event::poll(Duration::from_millis(5))? {
      if let Event::Key(key) = event::read()? {
        let is_c = key.code == KeyCode::Char('c') || key.code == KeyCode::Char('C');
        let is_ctrl_c = is_c && key.modifiers.contains(KeyModifiers::CONTROL);

        match key.code {
          KeyCode::Char('0') => vm.signal_key_down(VmKey::KEY_0),
          KeyCode::Char('1') => vm.signal_key_down(VmKey::KEY_1),
          KeyCode::Char('2') => vm.signal_key_down(VmKey::KEY_2),
          KeyCode::Char('3') => vm.signal_key_down(VmKey::KEY_3),
          KeyCode::Char('4') => vm.signal_key_down(VmKey::KEY_4),
          KeyCode::Char('5') => vm.signal_key_down(VmKey::KEY_5),
          KeyCode::Char('6') => vm.signal_key_down(VmKey::KEY_6),
          KeyCode::Char('7') => vm.signal_key_down(VmKey::KEY_7),
          KeyCode::Char('8') => vm.signal_key_down(VmKey::KEY_8),
          KeyCode::Char('9') => vm.signal_key_down(VmKey::KEY_9),
          KeyCode::Char('q') => vm.signal_key_down(VmKey::KEY_A),
          KeyCode::Char('w') => vm.signal_key_down(VmKey::KEY_B),
          KeyCode::Char('e') => vm.signal_key_down(VmKey::KEY_C),
          KeyCode::Char('a') => vm.signal_key_down(VmKey::KEY_D),
          KeyCode::Char('s') => vm.signal_key_down(VmKey::KEY_E),
          KeyCode::Char('d') => vm.signal_key_down(VmKey::KEY_F),

          KeyCode::Char(' ') => {
            self.step = false;
            self.paused = !self.paused;
          }
          KeyCode::Enter => {
            self.step = true;
            self.paused = false;
          }

          code if code == KeyCode::Esc || is_ctrl_c => {
            cleanup();
            std::process::exit(0);
          }
          _ => {}
        }
      }
    }

    Ok(())
  }
}

fn cleanup() {
  let mut stdout = io::stdout();
  terminal::disable_raw_mode().ok();
  execute!(stdout, LeaveAlternateScreen, DisableMouseCapture, Show).ok();
}
