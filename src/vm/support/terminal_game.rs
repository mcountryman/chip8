//! Terminal game widget.

use tui::{
  buffer::Buffer,
  layout::Rect,
  style::{Color, Style},
  widgets::{Block, Widget},
};

pub struct TerminalGameWidget<'vram, 'block> {
  block: Option<Block<'block>>,
  vram: &'vram [bool],
  vram_width: usize,
  vram_height: usize,
}

impl<'vram, 'block> TerminalGameWidget<'vram, 'block> {
  /// Creates a [TerminalGameWidget].
  pub fn new(width: usize, height: usize, vram: &'vram [bool]) -> Self {
    Self {
      vram,
      vram_width: width,
      vram_height: height,

      block: None,
    }
  }

  /// Sets parent block.
  pub fn block(mut self, block: Block<'block>) -> Self {
    self.block = Some(block);
    self
  }

  fn draw_full_blocks(&self, area: Rect, buf: &mut Buffer) {
    let vram_mid_x = self.vram_width / 2;
    let vram_mid_y = self.vram_height / 2;
    let game_mid_x = area.width / 2;
    let game_mid_y = area.height / 2;

    let mid_x = game_mid_x - vram_mid_x as u16;
    let mid_y = game_mid_y - vram_mid_y as u16;

    for y in 0..self.vram_height {
      for x in 0..self.vram_width {
        let color = if self.vram[y * self.vram_width + x] {
          Color::Green
        } else {
          Color::Black
        };

        let x = mid_x + x as u16;
        let y = mid_y + y as u16;

        buf.get_mut(x, y).set_bg(color);
      }
    }
  }

  fn draw_quarter_blocks(&self, area: Rect, buf: &mut Buffer) {
    let vram_mid_x = self.vram_width / 4;
    let vram_mid_y = self.vram_height / 4;
    let game_mid_x = area.width / 2;
    let game_mid_y = area.height / 2;

    let mid_x = game_mid_x - vram_mid_x as u16;
    let mid_y = game_mid_y - vram_mid_y as u16;

    for x in (0..self.vram_width).step_by(2) {
      for y in (0..self.vram_height).step_by(2) {
        let tl = y * self.vram_width + x;
        let tl = self.vram.get(tl).copied().unwrap_or(false);

        let tr = y * self.vram_width + x + 1;
        let tr = self.vram.get(tr).copied().unwrap_or(false);

        let bl = (y + 1) * self.vram_width + x;
        let bl = self.vram.get(bl).copied().unwrap_or(false);

        let br = (y + 1) * self.vram_width + x + 1;
        let br = self.vram.get(br).copied().unwrap_or(false);

        let x = mid_x + (x / 2) as u16;
        let y = mid_y + (y / 2) as u16;

        match (tl, tr, bl, br) {
          (true, true, true, true) => {
            buf.get_mut(x, y).set_bg(Color::Green);
          }
          (true, true, true, false) => {
            buf
              .get_mut(x, y)
              .set_char('▛')
              .set_fg(Color::Green)
              .set_bg(Color::Black);
          }
          (true, true, false, true) => {
            buf
              .get_mut(x, y)
              .set_char('▜')
              .set_fg(Color::Green)
              .set_bg(Color::Black);
          }
          (true, true, false, false) => {
            buf
              .get_mut(x, y)
              .set_char('▀')
              .set_fg(Color::Green)
              .set_bg(Color::Black);
          }
          (true, false, true, true) => {
            buf
              .get_mut(x, y)
              .set_char('▙')
              .set_fg(Color::Green)
              .set_bg(Color::Black);
          }
          (true, false, true, false) => {
            buf
              .get_mut(x, y)
              .set_char('▌')
              .set_fg(Color::Green)
              .set_bg(Color::Black);
          }
          (true, false, false, true) => {
            buf
              .get_mut(x, y)
              .set_char('▚')
              .set_fg(Color::Green)
              .set_bg(Color::Black);
          }
          (true, false, false, false) => {
            buf
              .get_mut(x, y)
              .set_char('▘')
              .set_fg(Color::Green)
              .set_bg(Color::Black);
          }
          (false, true, true, true) => {
            buf
              .get_mut(x, y)
              .set_char('▟')
              .set_fg(Color::Green)
              .set_bg(Color::Black);
          }
          (false, true, true, false) => {
            buf
              .get_mut(x, y)
              .set_char('▞')
              .set_fg(Color::Green)
              .set_bg(Color::Black);
          }
          (false, true, false, true) => {
            buf
              .get_mut(x, y)
              .set_char('▐')
              .set_fg(Color::Green)
              .set_bg(Color::Black);
          }
          (false, true, false, false) => {
            buf
              .get_mut(x, y)
              .set_char('▝')
              .set_fg(Color::Green)
              .set_bg(Color::Black);
          }
          (false, false, true, true) => {
            buf
              .get_mut(x, y)
              .set_char('▄')
              .set_fg(Color::Green)
              .set_bg(Color::Black);
          }
          (false, false, true, false) => {
            buf
              .get_mut(x, y)
              .set_char('▖')
              .set_fg(Color::Green)
              .set_bg(Color::Black);
          }
          (false, false, false, true) => {
            buf
              .get_mut(x, y)
              .set_char('▗')
              .set_fg(Color::Green)
              .set_bg(Color::Black);
          }
          (false, false, false, false) => {
            buf
              .get_mut(x, y)
              .set_char(' ')
              .set_fg(Color::Green)
              .set_bg(Color::Black);
          }
        }
      }
    }
  }
}

impl<'vram, 'block> Widget for TerminalGameWidget<'vram, 'block> {
  fn render(mut self, area: Rect, buf: &mut Buffer) {
    let area = match self.block.take() {
      Some(block) => {
        let inner = block.inner(area);
        let block = block.title(format!("chip8 - {}x{}", inner.width, inner.height));
        block.render(area, buf);
        inner
      }
      None => area,
    };

    let vram_width = self.vram_width as u16;
    let vram_height = self.vram_height as u16;

    if area.width >= vram_width && area.height >= vram_height {
      return self.draw_full_blocks(area, buf);
    }

    if area.width >= vram_width / 2 && area.height >= vram_height / 2 {
      return self.draw_quarter_blocks(area, buf);
    }

    buf.set_string(
      0,
      0,
      "Too small! Try resizing your terminal.",
      Style::default().fg(Color::Red),
    );
  }
}
