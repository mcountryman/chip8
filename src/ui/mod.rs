//! Chip-8 terminal user interface.

pub mod debug;
pub mod game;
pub mod keys;

use self::{game::Game, keys::UiKeys};
use crate::vm::{flags::VmKey, Vm};
use crossterm::{
  cursor::Show,
  event::{DisableMouseCapture, EnableMouseCapture, KeyCode, KeyModifiers},
  execute,
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use eyre::Result;
use std::{
  io::{self, Stdout},
  panic,
};
use tui::{
  backend::CrosstermBackend,
  layout::{Constraint, Direction, Layout},
  Terminal,
};

pub struct Ui {
  pub step: bool,
  pub paused: bool,

  keys: UiKeys,
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

    panic::set_hook(Box::new(|err| {
      cleanup();
      eprintln!("{}", err);
    }));

    Ok(Self {
      step: false,
      paused: false,
      keys: UiKeys::new(),
      terminal,
    })
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

    for key in &mut self.keys {
      let is_c = key.code == KeyCode::Char('c') || key.code == KeyCode::Char('C');
      let is_ctrl_c = is_c && key.modifiers.contains(KeyModifiers::CONTROL);

      match key.code {
        KeyCode::Char('0') => {
          if key.is_pressed {
            vm.signal_key_down(VmKey::KEY_0)
          } else {
            vm.signal_key_up(VmKey::KEY_0)
          }
        }
        KeyCode::Char('1') => {
          if key.is_pressed {
            vm.signal_key_down(VmKey::KEY_1)
          } else {
            vm.signal_key_up(VmKey::KEY_1)
          }
        }
        KeyCode::Char('2') => {
          if key.is_pressed {
            vm.signal_key_down(VmKey::KEY_2)
          } else {
            vm.signal_key_up(VmKey::KEY_2)
          }
        }
        KeyCode::Char('3') => {
          if key.is_pressed {
            vm.signal_key_down(VmKey::KEY_3)
          } else {
            vm.signal_key_up(VmKey::KEY_3)
          }
        }
        KeyCode::Char('4') => {
          if key.is_pressed {
            vm.signal_key_down(VmKey::KEY_4)
          } else {
            vm.signal_key_up(VmKey::KEY_4)
          }
        }
        KeyCode::Char('5') => {
          if key.is_pressed {
            vm.signal_key_down(VmKey::KEY_5)
          } else {
            vm.signal_key_up(VmKey::KEY_5)
          }
        }
        KeyCode::Char('6') => {
          if key.is_pressed {
            vm.signal_key_down(VmKey::KEY_6)
          } else {
            vm.signal_key_up(VmKey::KEY_6)
          }
        }
        KeyCode::Char('7') => {
          if key.is_pressed {
            vm.signal_key_down(VmKey::KEY_7)
          } else {
            vm.signal_key_up(VmKey::KEY_7)
          }
        }
        KeyCode::Char('8') => {
          if key.is_pressed {
            vm.signal_key_down(VmKey::KEY_8)
          } else {
            vm.signal_key_up(VmKey::KEY_8)
          }
        }
        KeyCode::Char('9') => {
          if key.is_pressed {
            vm.signal_key_down(VmKey::KEY_9)
          } else {
            vm.signal_key_up(VmKey::KEY_9)
          }
        }
        KeyCode::Char('q') => {
          if key.is_pressed {
            vm.signal_key_down(VmKey::KEY_A)
          } else {
            vm.signal_key_up(VmKey::KEY_A)
          }
        }
        KeyCode::Char('w') => {
          if key.is_pressed {
            vm.signal_key_down(VmKey::KEY_B)
          } else {
            vm.signal_key_up(VmKey::KEY_B)
          }
        }
        KeyCode::Char('e') => {
          if key.is_pressed {
            vm.signal_key_down(VmKey::KEY_C)
          } else {
            vm.signal_key_up(VmKey::KEY_C)
          }
        }
        KeyCode::Char('a') => {
          if key.is_pressed {
            vm.signal_key_down(VmKey::KEY_D)
          } else {
            vm.signal_key_up(VmKey::KEY_D)
          }
        }
        KeyCode::Char('s') => {
          if key.is_pressed {
            vm.signal_key_down(VmKey::KEY_E)
          } else {
            vm.signal_key_up(VmKey::KEY_E)
          }
        }
        KeyCode::Char('d') => {
          if key.is_pressed {
            vm.signal_key_down(VmKey::KEY_F)
          } else {
            vm.signal_key_up(VmKey::KEY_F)
          }
        }

        KeyCode::Char(' ') if key.is_pressed => {
          self.step = false;
          self.paused = !self.paused;
        }
        KeyCode::Enter if key.is_pressed => {
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

    Ok(())
  }
}

fn cleanup() {
  let mut stdout = io::stdout();
  terminal::disable_raw_mode().ok();
  execute!(stdout, LeaveAlternateScreen, DisableMouseCapture, Show).ok();
}
