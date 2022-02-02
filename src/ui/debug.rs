//! Debug widgets.

use crate::{insn::Insn, vm::Vm};
use tui::{
  backend::Backend,
  layout::{Alignment, Rect},
  style::{Color, Style},
  text::{Span, Spans},
  widgets::{Block, Borders, Paragraph},
  Frame,
};

pub fn keys<B: Backend>(vm: &Vm, area: Rect, ui: &mut Frame<B>) {
  let mut spans = Vec::new();
  let mut keys = vm.keys.to_vec();

  for _ in 0..4 {
    let mut line = Vec::new();

    for _ in 0..area.width / 3 {
      if keys.is_empty() {
        break;
      }

      match keys.pop() {
        None => break,
        Some(key) => line.push(Span::from(format!("{key:#x} "))),
      };
    }

    spans.push(Spans::from(line));
  }

  ui.render_widget(
    Paragraph::new(spans).block(
      Block::default()
        .title(" keys ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL),
    ),
    area,
  );
}

pub fn registers<B: Backend>(vm: &Vm, area: Rect, ui: &mut Frame<B>) {
  let mut spans = Vec::new();

  for i in 0..4 {
    let mut line = Vec::new();

    for j in i * 4..i * 4 + 4 {
      let reg = format!("V{j:x}: ");
      let val = format!("{:#04x} ", vm.reg8[j]);

      line.push(Span::styled(reg, Style::default().fg(Color::Gray)));
      line.push(Span::from(val));
    }

    let (reg, val) = match i {
      0 => ("PC: ", format!("{:#04x}", vm.reg_pc)),
      1 => (" I: ", format!("{:#04x}", vm.reg_i)),
      2 => ("DT: ", format!("{:#04x}", vm.reg_dt)),
      3 => ("ST: ", format!("{:#04x}", vm.reg_st)),
      _ => unreachable!(),
    };

    line.push(Span::styled(reg, Style::default().fg(Color::Gray)));
    line.push(Span::from(val));
    spans.push(Spans::from(line));
  }

  ui.render_widget(
    Paragraph::new(spans).block(
      Block::default()
        .title(" registers ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL),
    ),
    area,
  );
}

pub fn stack<B: Backend>(vm: &Vm, area: Rect, ui: &mut Frame<B>) {
  let mut spans = Vec::new();

  for i in 0..16 {
    let mut line = Vec::new();

    if vm.reg_sp == i as u8 {
      line.push(Span::styled("> ", Style::default().fg(Color::Blue)));
    } else {
      line.push(Span::from("  "));
    }

    let add = format!("{i:#04x}");
    let val = format!(": {:#06x}  ", vm.stack[i]);

    line.push(Span::styled(add, Style::default().fg(Color::Gray)));
    line.push(Span::from(val));
    spans.push(Spans::from(line));
  }

  ui.render_widget(
    Paragraph::new(spans).block(
      Block::default()
        .title(" stack ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL),
    ),
    area,
  );
}

pub fn disassembly<B: Backend>(vm: &Vm, area: Rect, ui: &mut Frame<B>) {
  let mut spans = Vec::new();
  let beg = (vm.reg_pc as usize).saturating_sub(area.height as usize);
  let beg = if beg % 2 == 0 { beg } else { beg + 1 };
  let end = (vm.reg_pc as usize).saturating_add(area.height as usize);
  let end = if end % 2 == 0 { end } else { end + 1 };

  let insns = (beg..end).step_by(2).filter_map(|i| {
    let (hi, lo) = vm.get_insn_bytes_at(i)?;
    let insn = Insn::from_bytes(hi, lo)?;

    Some((i, (hi as u16) << 8 | lo as u16, insn))
  });

  for (offs, byte, insn) in insns {
    let mut line = Vec::new();

    let offs_color = if offs == vm.reg_pc as usize {
      Color::Blue
    } else {
      Color::Gray
    };

    let offs = format!("{offs:#06x} ");
    let byte = format!("{byte:04x} ");
    let insn = format!("{insn}");

    line.push(Span::styled(offs, Style::default().fg(offs_color)));
    line.push(Span::styled(byte, Style::default().fg(Color::Yellow)));
    line.push(Span::from(insn));
    spans.push(Spans::from(line));
  }

  ui.render_widget(
    Paragraph::new(spans).block(
      Block::default()
        .title(" disassembly ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL),
    ),
    area,
  );
}
