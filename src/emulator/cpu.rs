use std::cell::RefCell;
use std::io::{Read, Result};
use std::rc::Rc;
use std::time::Instant;

use rand::{Rng, thread_rng};
use rand::prelude::ThreadRng;

use crate::emulator::ipu::Ipu;
use crate::emulator::ram::{MEMORY_PROGRAM, Ram};

pub const V0: usize = 0x00;
pub const V1: usize = 0x01;
pub const V2: usize = 0x02;
pub const V3: usize = 0x03;
pub const V4: usize = 0x04;
pub const V5: usize = 0x05;
pub const V6: usize = 0x06;
pub const V7: usize = 0x07;
pub const V8: usize = 0x08;
pub const V9: usize = 0x09;
pub const VA: usize = 0x0A;
pub const VB: usize = 0x0B;
pub const VC: usize = 0x0C;
pub const VD: usize = 0x0D;
pub const VE: usize = 0x0E;
pub const VF: usize = 0x0F;

pub const ST: usize = 0x10;
pub const DT: usize = 0x11;

pub struct Cpu {
  ram: Rc<RefCell<Ram>>,
  ipu: Rc<RefCell<Ipu>>,
  rng: ThreadRng,

  reg8: [u8; 0x11],
  reg16: u16,
  stack: [u16; 16],
  clock: Instant,

  stack_pointer: usize,
  program_counter: usize,
}

enum Step {
  Next,
  Skip,
  Jump(usize),
}

impl Cpu {

  pub fn new(ram: Rc<RefCell<Ram>>, ipu: Rc<RefCell<Ipu>>) -> Self {
    Self {
      ram,
      ipu,
      rng: thread_rng(),

      reg8: [0u8; 0x11],
      reg16: 0,
      stack: [0u16; 0x10],
      clock: Instant::now(),

      stack_pointer: 0,
      program_counter: 0,
    }
  }

  pub fn tick(&mut self) -> Result<()> {
    // Tick timers
    if self.clock.elapsed().as_millis() > 16 {
      if self.reg8[DT] > 0 {
        self.reg8[DT] -= 1;
      }

      if self.reg8[ST] > 0 {
        self.reg8[ST] -= 1;
      }

      self.clock = Instant::now()
    }

    // Run instruction
    {
      let instruction = self.get_instruction();
      let step = self.run_instruction(instruction);

      match step {
        Step::Next => self.program_counter += 1,
        Step::Skip => self.program_counter += 2,
        Step::Jump(addr) => self.program_counter = addr,
      }
    }

    Ok(())
  }

  pub fn reset(&mut self) {
    // Reset reg16 & counter
    self.reg16 = 0;
    self.stack_pointer = 0;
    self.program_counter = 0;

    // Reset registers
    for i in 0..DT {
      self.reg8[i] = 0;
    }

    // Reset stack
    for i in 0..16 {
      self.stack[i] = 0;
    }
  }

  fn get_instruction(&self) -> u16 {
    let mut ram = self.ram.borrow_mut();
    let bite1 = ram.memory[MEMORY_PROGRAM + self.program_counter];
    let bite2 = ram.memory[MEMORY_PROGRAM + self.program_counter + 1];

    (bite1 as u16) << 8 | bite2 as u16
  }

  fn run_instruction(&mut self, instruction: u16) -> Step {
    let nibbles = (
      (instruction & 0xF000 >> 12) as u8,
      (instruction & 0x0F00 >> 8) as u8,
      (instruction & 0x00F0 >> 4) as u8,
      (instruction & 0x000F) as u8,
    );

    let addr = (instruction & 0x0FFF) as usize;
    let kk = (instruction & 0x00FF) as u8;
    let x = nibbles.1 as usize;
    let y = nibbles.2 as usize;
    let n = nibbles.3;

    match nibbles {
      (0x0, 0x0, 0xC, _) => Step::Next, // SCD nibble
      (0x0, 0x0, 0xE, 0x0) => self.run_cls(), // CLS
      (0x0, 0x0, 0xE, 0xE) => self.run_ret(), // RET
      (0x0, 0x0, 0xF, 0xB) => Step::Next, // SCR
      (0x0, 0x0, 0xF, 0xC) => Step::Next, // SCL
      (0x0, 0x0, 0xF, 0xD) => Step::Next, // EXIT
      (0x0, 0x0, 0xF, 0xE) => Step::Next, // LOW
      (0x0, 0x0, 0xF, 0xF) => Step::Next, // HIGH
      (0x0, ..) => self.run_sys(), // SYS addr
      (0x1, ..) => self.run_jp(addr), // JP addr
      (0x2, ..) => self.run_call(addr), // CALL addr
      (0x3, ..) => self.run_se_byte(x, kk), // SE Vx, byte
      (0x4, ..) => self.run_sne_byte(x, kk), // SNE Vx, byte
      (0x5, .., 0x0) => self.run_se_reg8(x, y), // SE Vx, Vy
      (0x6, ..) => self.run_ld_byte(x, kk), // LD Vx, byte
      (0x7, ..) => self.run_add_byte(x, kk), // ADD Vx, byte
      (0x8, .., 0x0) => self.run_ld_byte(x, kk), // LD Vx, byte
      (0x8, .., 0x1) => self.run_or_reg8(x, y), // OR Vx, Vy
      (0x8, .., 0x2) => self.run_and_reg8(x, y), // AND Vx, Vy
      (0x8, .., 0x3) => self.run_xor_reg8(x, y), // XOR Vx, Vy
      (0x8, .., 0x4) => self.run_add_reg8(x, y), // ADD Vx, Vy
      (0x8, .., 0x5) => self.run_sub_reg8(x, y), // SUB Vx, Vy
      (0x8, .., 0x6) => self.run_shr_reg8(x, y), // SHR Vx {, Vy}
      (0x8, .., 0x7) => self.run_subn_reg8(x, y), // SUBN Vx, Vy
      (0x8, .., 0xE) => self.run_shl_reg8(x, y), // SHL Vx, {, Vy}
      (0x9, .., 0x0) => self.run_sne_reg8(x, y), //SNE, Vx, Vy
      (0xA, ..) => self.run_ld_addr(addr), // LD I, addr
      (0xB, ..) => self.run_jp_relative(addr), // JP V0, addr
      (0xC, ..) => self.run_rnd(x, kk), // RND Vx, byte
      (0xD, ..) => self.run_drw(x, y, n), // DRW Vx, Vy, nibble
      (0xE, _, 0x9, 0xE) => self.run_skp(x), // SKP Vx
      (0xE, _, 0xA, 0xA) => self.run_sknp(x), // SKNP Vx
      (0xF, _, 0x0, 0x7) => self.run_ld_dt(x), // LD Vx, DT
      (0xF, _, 0x0, 0xA) => self.run_ld_key(x), // LD Vx, K
      (0xF, _, 0x1, 0x5) => self.run_ld_dt_vx(x), // LD DT, Vx
      (0xF, _, 0x1, 0x8) => self.run_ld_st_vx(x), // LD ST, Vx
      (0xF, _, 0x1, 0xE) => self.run_add_reg16(x), // ADD I, Vx
      (0xF, _, 0x2, 0x9) => self.run_ld_sprite(x), // LD F, Vx
      (0xF, _, 0x3, 0x0) => Step::Next, // LD HF, Vx
      (0xF, _, 0x3, 0x3) => self.run_ld_bcd(x), // LD B, Vx
      (0xF, _, 0x5, 0x5) => self.run_ld_reg16(x), // LD [I], Vx
      (0xF, _, 0x6, 0x5) => self.run_ld_mem(x), // Ld Vx, [I]
      (0xF, _, 0x7, 0x5) => Step::Next, // LD R, Vx
      (0xF, _, 0x8, 0x5) => Step::Next, // LD Vx, R

      _ => Step::Next,
    }
  }

  /// CLS
  ///
  /// Clear screen
  ///
  fn run_cls(&mut self) -> Step {
    // TODO: Implement this
    Step::Next
  }

  /// RET
  ///
  /// Return from a subroutine
  ///
  fn run_ret(&mut self) -> Step {
    Step::Jump(self.stack[0] as usize - 1)
  }

  /// SYS
  ///
  /// Ignored
  ///
  fn run_sys(&mut self) -> Step {
    Step::Next // ignore
  }

  /// JMP addr
  fn run_jp(&mut self, addr: usize) -> Step {
    Step::Jump(addr)
  }

  /// CALL addr
  fn run_call(&mut self, addr: usize) -> Step {
    self.stack_pointer += 1;
    self.stack[0] = self.program_counter as u16;

    Step::Jump(addr)
  }

  /// SE Vx, byte
  fn run_se_byte(&mut self, vx: usize, byte: u8) -> Step {
    if self.reg8[vx] == byte {
      Step::Skip
    } else {
      Step::Next
    }
  }

  /// SNE Vx, byte
  fn run_sne_byte(&mut self, vx: usize, byte: u8) -> Step {
    if self.reg8[vx] != byte {
      Step::Skip
    } else {
      Step::Next
    }
  }

  /// SE Vx, byte
  fn run_se_reg8(&mut self, vx: usize, vy: usize) -> Step {
    if self.reg8[vx] == self.reg8[vy] {
      Step::Skip
    } else {
      Step::Next
    }
  }

  /// LD Vx, byte
  fn run_ld_byte(&mut self, vx: usize, byte: u8) -> Step {
    self.reg8[vx] = byte;
    Step::Next
  }

  /// ADD Vx, byte
  fn run_add_byte(&mut self, vx: usize, byte: u8) -> Step {
    self.reg8[vx] += byte;
    Step::Next
  }

  /// LD Vx, Vy
  fn run_ld_reg8(&mut self, vx: usize, vy: usize) -> Step {
    self.reg8[vx] = self.reg8[vy];
    Step::Next
  }

  /// OR Vx, Vy
  fn run_or_reg8(&mut self, vx: usize, vy: usize) -> Step {
    self.reg8[vx] |= self.reg8[vy];
    Step::Next
  }

  /// AND Vx, Vy
  fn run_and_reg8(&mut self, vx: usize, vy: usize) -> Step {
    self.reg8[vx] &= self.reg8[vy];
    Step::Next
  }

  /// XOR Vx, Vy
  fn run_xor_reg8(&mut self, vx: usize, vy: usize) -> Step {
    self.reg8[vx] |= self.reg8[vy];
    Step::Next
  }

  /// ADD Vx, Vy
  fn run_add_reg8(&mut self, vx: usize, vy: usize) -> Step {
    self.reg8[vx] += self.reg8[vy];
    Step::Next
  }

  /// SUB Vx, Vy
  fn run_sub_reg8(&mut self, vx: usize, vy: usize) -> Step {
    self.reg8[vx] -= self.reg8[vy];
    Step::Next
  }

  /// SHR Vx, {, Vy }
  ///
  /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx
  /// is divided by 2.
  ///
  fn run_shr_reg8(&mut self, vx: usize, vy: usize) -> Step {
    if self.reg8[vx] << 1 == 0x1 {
      self.reg8[VF] = 1
    } else {
      self.reg8[VF] = 0
    }

    self.reg8[vx] /= 2;
    Step::Next
  }

  /// SUBN Vx, Vy
  ///
  /// Set Vx = Vy - Vx, set VF = NOT borrow.
  ///
  fn run_subn_reg8(&mut self, vx: usize, vy: usize) -> Step {
    let vx_value = self.reg8[vx];
    let vy_value = self.reg8[vy];

    if vx_value < vy_value {
      self.reg8[VF] = 1
    } else {
      self.reg8[VF] = 0
    }

    self.reg8[vx] = vy_value - vx_value;
    Step::Next
  }

  /// SHL Vx, {, Vy}
  ///
  /// Set Vx = Vx SHL 1.
  ///
  fn run_shl_reg8(&mut self, vx: usize, vy: usize) -> Step {
    if self.reg8[vx] & 0xF0 == 1 {
      self.reg8[VF] = 1;
    } else {
      self.reg8[VF] = 0;
    }

    self.reg8[vx] *= 2;
    Step::Next
  }

  /// SNE Vx, Vy
  ///
  /// Skip next instruction if Vx != Vy
  ///
  fn run_sne_reg8(&mut self, vx: usize, vy: usize) -> Step {
    if self.reg8[vx] != self.reg8[vy] {
      Step::Skip
    } else {
      Step::Next
    }
  }

  /// LD I, addr
  ///
  /// The value of register I is set to nnn
  ///
  fn run_ld_addr(&mut self, addr: usize) -> Step {
    self.reg16 = addr as u16;
    Step::Next
  }

  /// JP V0, addr
  ///
  /// The program counter is set to nnn plus the value of V0
  ///
  fn run_jp_relative(&mut self, addr: usize) -> Step {
    Step::Jump(self.reg8[V0] as usize + addr)
  }

  /// RND Vx, byte
  ///
  /// Set Vx = random byte AND kk
  ///
  fn run_rnd(&mut self, vx: usize, byte: u8) -> Step {
    self.reg8[vx] = self.rng.gen_range(0, 255) & byte;
    Step::Next
  }

  /// DRW Vx, Vy, nibble
  ///
  /// Display n-byte sprite starting at memory location I at (Vx, Fy), set VF = collision.
  ///
  fn run_drw(&mut self, vx: usize, vy: usize, nubble: u8) -> Step {
    // TODO: Implement DRW
    // The interpreter reads n bytes from memory, starting at the address stored in I.
    // These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
    // Sprites are XORed onto the existing screen. If this causes any pixels to be erased,
    // VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it
    // is outside the coordinates of the display, it wraps around to the opposite side of
    // the screen. See instruction 8xy3 for more information on XOR, and section 2.4,
    // Display, for more information on the Chip-8 screen and sprites.
    Step::Next
  }

  /// SKP Vx
  ///
  /// Skip next instruction if key with the value of Vx is pressed
  ///
  fn run_skp(&mut self, vx: usize) -> Step {
    if self.ipu.borrow().is_key_pressed(self.reg8[vx]) {
      Step::Skip
    } else {
      Step::Next
    }
  }

  /// SKNP Vx
  ///
  /// Skip next instruction if key with the value of Vx is pressed
  ///
  fn run_sknp(&mut self, vx: usize) -> Step {
    if !self.ipu.borrow().is_key_pressed(self.reg8[vx]) {
      Step::Skip
    } else {
      Step::Next
    }
  }

  /// LD Vx, DT
  ///
  /// Set Vx - delay timer value.
  ///
  fn run_ld_dt(&mut self, vx: usize) -> Step {
    self.reg8[vx] = self.reg8[DT];
    Step::Next
  }

  /// LD Vx, K
  ///
  /// Keyboard interrupt, store key in Vx
  ///
  fn run_ld_key(&mut self, vx: usize) -> Step {
    self.reg8[vx] = self.ipu.borrow().wait_key();
    Step::Next
  }

  /// LD DT, Vx
  ///
  /// Set delay timer = Vx
  ///
  fn run_ld_dt_vx(&mut self, vx: usize) -> Step {
    self.reg8[DT] = self.reg8[vx];
    Step::Next
  }

  /// LD ST, VX
  ///
  /// Set sound timer = Vx
  ///
  fn run_ld_st_vx(&mut self, vx: usize) -> Step {
    self.reg8[ST] = self.reg8[vx];
    Step::Next
  }

  /// ADD I, Vx
  ///
  /// Set I = I + Vx
  ///
  fn run_add_reg16(&mut self, vx: usize) -> Step {
    self.reg16 += self.reg8[vx] as u16;
    Step::Next
  }

  /// LD F, Vx
  ///
  /// Set I = location sprite for digit Vx
  ///
  fn run_ld_sprite(&mut self, vx: usize) -> Step {
    // TODO: Implement `LD F, Vx`
    // The value of I is set to the location for the hexadecimal sprite corresponding to
    // the value of Vx. See section 2.4, Display, for more information on the Chip-8
    // hexadecimal font.
    Step::Next
  }

  /// LD B, Vx
  ///
  /// Store BCD representation of Vx in memory locations I, I+1, and I+2
  ///
  fn run_ld_bcd(&mut self, vx: usize) -> Step {
    // TODO: Implemenet `LD B, Vx`
    // The interpreter takes the decimal value of Vx, and places the hundreds digit in
    // memory at location in I, the tens digit at location I+1, and the ones digit at
    // location I+2.

    Step::Next
  }

  /// LD [I], Vx
  ///
  /// Store registers V0 through Vx in memory starting at location I
  ///
  fn run_ld_reg16(&mut self, vx: usize) -> Step {
    let mut ram = self.ram.borrow_mut();
    let offset = self.reg16;

    for i in V0..vx {
      ram.memory[(offset + i as u16) as usize] = self.reg8[i];
    }

    Step::Next
  }

  /// LD Vx, [I]
  ///
  /// Read registers V0 through Vx in memory starting at location I
  ///
  fn run_ld_mem(&mut self, vx: usize) -> Step {
    let ram = self.ram.borrow();
    let offset = self.reg16;

    for i in V0..vx {
      self.reg8[i] = ram.memory[(offset + i as u16) as usize];
    }

    Step::Next
  }

}
