//! Terminal debug ui.

use std::collections::HashMap;

use crate::{
  insn::{
    insns::{CallInsn, JpInsn},
    Insn,
  },
  vm::state::VmState,
};
use tui::{
  backend::Backend,
  layout::{Alignment, Constraint, Direction, Layout, Rect},
  style::{Color, Style},
  text::{Span, Spans},
  widgets::{Block, Borders, Paragraph},
  Frame,
};

pub fn render_debug_widgets<B: Backend>(state: &VmState, area: Rect, ui: &mut Frame<B>) {
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(7), Constraint::Min(0)].as_ref())
    .split(area);

  render_registers(state, chunks[0], ui);

  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Min(10), Constraint::Length(18)].as_ref())
    .split(chunks[1]);

  render_disassembly(state, chunks[0], ui);
  render_stack(state, chunks[1], ui);
}

fn render_registers<B: Backend>(state: &VmState, area: Rect, ui: &mut Frame<B>) {
  let mut registers = String::new();

  for line in 0..4 {
    for i in line * 4..line * 4 + 4 {
      registers.push_str(&format!("V{i:x}: {:#04x} ", state.reg8[i]));
    }

    registers.push('\n');
  }

  registers.push_str(&format!("SP: {:#04x} ", state.reg_sp));
  registers.push_str(&format!("PC: {:#04x} ", state.reg_pc));
  registers.push_str(&format!("I: {:#04x} ", state.reg_i));
  registers.push_str(&format!("DT: {:#04x} ", state.reg_dt));
  registers.push_str(&format!("ST: {:#04x} ", state.reg_st));

  ui.render_widget(
    Paragraph::new(registers).block(
      Block::default()
        .title(" registers ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL),
    ),
    area,
  );
}

fn render_stack<B: Backend>(state: &VmState, area: Rect, ui: &mut Frame<B>) {
  let mut spans = Vec::new();

  for i in 0..16 {
    let mut line = Vec::new();

    if state.reg_sp == i as u8 {
      line.push(Span::styled("> ", Style::default().fg(Color::Blue)));
    } else {
      line.push(Span::from("  "));
    }

    line.push(Span::styled(
      format!("{i:#04x}"),
      Style::default().fg(Color::Gray),
    ));
    line.push(Span::from(format!(": {:#06x}  ", state.stack[i])));
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

fn render_disassembly<B: Backend>(state: &VmState, area: Rect, ui: &mut Frame<B>) {
  let mut spans = Vec::new();
  let beg = state.reg_pc as usize;
  let beg = beg - area.height as usize;
  let end = beg + area.height as usize;

  let mut insns = Vec::new();
  let mut jp_to = HashMap::new();
  let mut jp_from = HashMap::new();

  for i in (beg..end * 2).step_by(2) {
    let insn = match state.get_insn_at(i) {
      Some(insn) => insn,
      None => break,
    };

    match insn {
      Insn::Jp(JpInsn::Addr(addr)) => {
        jp_from.insert(addr, i);
      }
      Insn::Jp(JpInsn::AddrReg(addr)) => {}
      Insn::Call(CallInsn { addr }) => {}
      _ if jp_from.get(&i).is_some() => {}
      _ => {}
    }

    insns.push(insn);
  }

  for i in (beg..end * 2).step_by(2) {
    let mut line = Vec::new();
    let (hi, lo) = match state.get_insn_bytes_at(i) {
      Some(insn) => insn,
      None => break,
    };

    let insn = match Insn::from_bytes(hi, lo) {
      Some(insn) => insn,
      None => break,
    };

    let color = if i == state.reg_pc as usize {
      Color::Blue
    } else {
      Color::Gray
    };

    match insn {
      Insn::Jp(JpInsn::Addr(addr)) => {
        jp_from.insert(addr as usize, 1);
        line.push(Span::styled(" ╭--", Style::default().fg(color)));
      }
      Insn::Jp(JpInsn::AddrReg(addr)) => {
        jp_from.insert(addr as usize + state.reg8[0] as usize, 1);
        line.push(Span::styled(" ╭--", Style::default().fg(color)));
      }
      Insn::Call(CallInsn { addr }) => {
        jp_from.insert(addr as usize, 1);
        line.push(Span::styled(" ╭--", Style::default().fg(color)));
      }
      _ if jp_from.get(&i).is_some() => {
        line.push(Span::styled(" ╰->", Style::default().fg(color)));
      }
      _ => {
        line.push(Span::styled(" |  ", Style::default().fg(color)));
      }
    }

    line.push(Span::styled(
      format!("{i:#06x} "),
      Style::default().fg(color),
    ));
    line.push(Span::styled(
      format!("{:x}{:x}{:x}{:x} ", hi >> 4, hi & 0x0f, lo >> 4, lo & 0x0f),
      Style::default().fg(Color::LightBlue),
    ));
    line.push(Span::from(format!("{}", insn)));
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
