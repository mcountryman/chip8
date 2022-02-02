//! Terminal events.

use crossterm::event::{self, Event, KeyEvent};
use std::{
  ops::Deref,
  sync::mpsc,
  thread,
  time::{Duration, Instant},
};

/// A terminal events iterator.
pub struct UiKeys {
  rx: mpsc::Receiver<KeyEventWithState>,
  _tx: mpsc::Sender<KeyEventWithState>,
}

#[derive(Debug, Clone, Copy)]
pub struct KeyEventWithState {
  event: KeyEvent,
  pub is_pressed: bool,
}

impl KeyEventWithState {
  /// Create a [KeyEventWithState] from [KeyEvent] and `is_pressed` state.
  pub fn new(event: KeyEvent, is_pressed: bool) -> Self {
    Self { event, is_pressed }
  }
}

impl Deref for KeyEventWithState {
  type Target = KeyEvent;

  fn deref(&self) -> &Self::Target {
    &self.event
  }
}

impl UiKeys {
  /// Create a new [UiEvents].
  ///
  /// Creates a background thread that polls terminal events in an infinite loop, avoid
  /// instantiating multiple instances of this struct.
  ///
  /// If this was more than a pet chip8 emulator project I would probably consider using
  /// a static mutex to have a single thread spawn and dispatch events to each instance of
  /// [UiEvents].
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    let (tx, rx) = mpsc::channel();
    let _tx = tx.clone();

    thread::spawn(move || {
      // let mut keys = HashMap::new();
      let mut last_key: Option<KeyEvent> = None;
      let mut last_key_pressed = Instant::now();

      loop {
        if let Ok(true) = event::poll(Duration::from_millis(50)) {
          if let Ok(Event::Key(key)) = event::read() {
            if let Some(last) = last_key {
              if last.code != key.code {
                tx.send(KeyEventWithState::new(last, false)).ok();
                tx.send(KeyEventWithState::new(key, true)).ok();
              }
            } else {
              tx.send(KeyEventWithState::new(key, true)).ok();
            }

            last_key = Some(key);
            last_key_pressed = Instant::now();
          }
        }

        if let Some(last) = last_key {
          if last_key_pressed.elapsed().as_millis() > 500 {
            tx.send(KeyEventWithState::new(last, false)).ok();

            last_key = None;
            last_key_pressed = Instant::now();
          }
        }
      }
    });

    Self { rx, _tx }
  }
}

impl Iterator for UiKeys {
  type Item = KeyEventWithState;

  fn next(&mut self) -> Option<Self::Item> {
    self.rx.try_recv().ok()
  }
}
