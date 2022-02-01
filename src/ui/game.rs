//! Game widget.

use tui::{
  buffer::Buffer,
  layout::Rect,
  style::{Color, Style},
  widgets::Widget,
};

pub struct Game<'vram> {
  vram: &'vram [u64],
}

impl<'vram> Game<'vram> {
  /// Creates a [Game].
  pub fn new(vram: &'vram [u64]) -> Self {
    Self { vram }
  }

  /// Gets vram width.
  pub fn vram_width(&self) -> usize {
    u64::BITS as _
  }

  /// Gets vram height.
  pub fn vram_height(&self) -> usize {
    self.vram.len()
  }

  fn render_blocks(&self, area: Rect, buf: &mut Buffer) {
    let vram_mid_x = self.vram_width() / 2;
    let vram_mid_y = self.vram_height() / 2;
    let game_mid_x = area.width / 2;
    let game_mid_y = area.height / 2;

    let mid_x = game_mid_x - vram_mid_x as u16;
    let mid_y = game_mid_y - vram_mid_y as u16;

    for y in 0..self.vram_height() {
      let byte = self.vram[y];

      for x in 0..self.vram_width() {
        let pixel = byte >> (63 - x) & 0x1 != 0;
        let color = if pixel { Color::Green } else { Color::Black };

        let x = mid_x + x as u16;
        let y = mid_y + y as u16;

        buf.get_mut(x, y).set_bg(color);
      }
    }
  }

  fn render_quarters(&self, area: Rect, buf: &mut Buffer) {
    let vram_mid_x = self.vram_width() / 4;
    let vram_mid_y = self.vram_height() / 4;
    let game_mid_x = area.width / 2;
    let game_mid_y = area.height / 2;

    let mid_x = game_mid_x - vram_mid_x as u16;
    let mid_y = game_mid_y - vram_mid_y as u16;

    for y in (0..self.vram_height()).step_by(2) {
      let n1 = self.vram[y];
      let n2 = self.vram[y + 1];

      for x in (0..self.vram_width()).step_by(2) {
        // shifts two `n1` bits at x into higher bits of u4.
        let n1 = n1 >> 60u64.saturating_sub(x as u64);
        let n1 = n1 as u8 & 0b1100;

        // shifts two `n2` bits at x into lower bits of u4.
        let n2 = n2 >> 62u64.saturating_sub(x as u64);
        let n2 = n2 as u8 & 0b0011;

        // Collect 2x2 bits in u8 where trailing bits correspond to tl, tr, bl, br
        let bits = n1 | n2;

        let x = mid_x + (x / 2) as u16;
        let y = mid_y + (y / 2) as u16;

        match get_quarter_for_bits(bits) {
          Some(quarter) => buf
            .get_mut(x, y)
            .set_char(quarter)
            .set_fg(Color::Green)
            .set_bg(Color::Black),
          None => buf.get_mut(x, y).set_bg(Color::Green),
        };
      }
    }
  }

  fn render_braille(&self, area: Rect, buf: &mut Buffer) {
    let vram_mid_x = self.vram_width() / 4;
    let vram_mid_y = self.vram_height() / 8;
    let game_mid_x = area.width / 2;
    let game_mid_y = area.height / 2;

    let mid_x = game_mid_x - vram_mid_x as u16;
    let mid_y = game_mid_y - vram_mid_y as u16;

    for y in (0..self.vram_height()).step_by(4) {
      let n1 = self.vram[y];
      let n2 = self.vram[y + 1];
      let n3 = self.vram[y + 2];
      let n4 = self.vram[y + 3];

      for x in (0..self.vram_width()).step_by(2) {
        // 14_25_36_78

        // shifts two `n1` bits at x into bits of u8.
        let n1 = n1 >> 56u64.saturating_sub(x as u64);
        let n1_hi = n1 as u8 & 0b10_00_00_00;
        let n1_lo = n1 as u8 & 0b01_00_00_00;
        let n1 = n1_hi >> 7 | n1_lo >> 4;

        // shifts two `n2` bits at x into bits of u8.
        let n2 = n2 >> 58u64.saturating_sub(x as u64);
        let n2_hi = n2 as u8 & 0b00_10_00_00;
        let n2_lo = n2 as u8 & 0b00_01_00_00;
        let n2 = n2_hi >> 4 | n2_lo << 1;

        // shifts two `n1` bits at x into bits of u8.
        let n3 = n3 >> 60u64.saturating_sub(x as u64);
        let n3_hi = n3 as u8 & 0b00_00_10_00;
        let n3_lo = n3 as u8 & 0b00_00_01_00;
        let n3 = n3_hi >> 1 | n3_lo << 3;

        // shifts two `n2` bits at x into bits of u8.
        let n4 = n4 >> 62u64.saturating_sub(x as u64);
        let n4_hi = n4 as u8 & 0b00_00_00_10; // 7, 8
        let n4_lo = n4 as u8 & 0b00_00_00_01; // 7, 8
        let n4 = n4_hi << 5 | n4_lo << 7;

        // Collect 2x2 bits in u8 where trailing bits correspond to tl, tr, bl, br
        let bits = n1 | n2 | n3 | n4;

        let x = mid_x + (x / 2) as u16;
        let y = mid_y + (y / 4) as u16;

        match get_braille_for_bits(bits) {
          Some(quarter) => buf
            .get_mut(x, y)
            .set_char(quarter)
            .set_fg(Color::Green)
            .set_bg(Color::Black),
          None => buf.get_mut(x, y).set_bg(Color::Black),
        };
      }
    }
  }
}

impl<'vram> Widget for Game<'vram> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let width = self.vram_width() as u16;
    let height = self.vram_height() as u16;

    if area.width >= width && area.height >= height {
      return self.render_blocks(area, buf);
    }

    if area.width >= width / 2 && area.height >= height / 2 {
      return self.render_quarters(area, buf);
    }

    if area.width >= width / 2 && area.height >= height / 4 {
      return self.render_braille(area, buf);
    }

    buf.set_string(
      0,
      0,
      "Too small! Try resizing your terminal.",
      Style::default().fg(Color::Red),
    );
  }
}

fn get_quarter_for_bits(bits: u8) -> Option<char> {
  match bits {
    0b0000 => Some(' '),
    0b0001 => Some('▗'),
    0b0010 => Some('▖'),
    0b0011 => Some('▄'),
    0b0100 => Some('▝'),
    0b0101 => Some('▐'),
    0b0110 => Some('▞'),
    0b0111 => Some('▟'),
    0b1000 => Some('▘'),
    0b1001 => Some('▚'),
    0b1010 => Some('▌'),
    0b1011 => Some('▙'),
    0b1100 => Some('▀'),
    0b1101 => Some('▜'),
    0b1110 => Some('▛'),
    0b1111 => Some('█'),

    _ => None,
  }
}

fn get_braille_for_bits(bits: u8) -> Option<char> {
  if bits == 0 {
    Some(' ')
  } else {
    char::from_u32(0x28u32 << 8 | bits as u32)
  }
}
