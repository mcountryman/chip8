use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::io::{Read, Result};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::time::Instant;

use rand::{Rng, thread_rng};
use rand::prelude::ThreadRng;

use crate::emulator::bus::Bus;
use crate::emulator::gpu::{Gpu, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::emulator::ipu::Ipu;
use crate::emulator::ram::{MEMORY_END, MEMORY_PROGRAM, Ram};
use std::cmp;
use rand::seq::index::IndexVec::USize;

pub const V0: usize = 0x00;
pub const VF: usize = 0x0F;
pub const ST: usize = 0x10;
pub const DT: usize = 0x11;

pub const INTERRUPT_KEY: u8 = 0x2;

pub type CpuRef = Rc<RefCell<Cpu>>;

pub struct Cpu {
  bus: Rc<RefCell<Bus>>,

  reg8: [u8; 0x12],
  reg16: u16,
  stack: [u16; 16],
  clock: Instant,

  // TODO: Maybe implement interrupt / signal interface for debug
  pub key_code: u8,
  pub key_waiting: bool,
  pub key_register: usize,

  stack_pointer: usize,
  program_counter: usize,
}

enum Step {
  Next,
  Skip,
  Jump(usize),
}

impl Cpu {
  pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
    let mut cpu = Self {
      bus,

      reg8: [0u8; 0x12],
      reg16: 0,
      stack: [0u16; 0x10],
      clock: Instant::now(),

      key_code: 0,
      key_waiting: false,
      key_register: 0,

      stack_pointer: 0,
      program_counter: 0,
    };

    cpu.reset();
    cpu
  }

  pub fn tick(&mut self) -> Result<()> {
    if self.key_waiting {
      return Ok(())
    } else {
      if self.key_register != std::usize::MAX {
        self.reg8[self.key_register] = self.key_code;
        self.key_register = std::usize::MAX;
        self.key_code = 0;
      }
    }

    // Tick timers
    if self.reg8[DT] > 0 {
      self.reg8[DT] -= 1;
    }

    if self.reg8[ST] > 0 {
      self.reg8[ST] -= 1;
    }

    // Run instruction
    if self.program_counter < MEMORY_END - 1 {
      let instruction = self.get_instruction();
      let step = self.run_instruction(instruction);

      match step {
        Step::Next => self.program_counter += 2,
        Step::Skip => self.program_counter += 4,
        Step::Jump(addr) => self.program_counter = addr,
      }
    }

    Ok(())
  }

  pub fn reset(&mut self) {
    self.key_code = 0;
    self.key_waiting = false;
    self.key_register = std::usize::MAX;

    self.reg16 = 0;
    self.stack_pointer = 0;
    self.program_counter = MEMORY_PROGRAM;

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
    if let Some(ref ram) = *self.bus.borrow().ram.borrow() {
      let bite1 = ram.memory[self.program_counter];
      let bite2 = ram.memory[self.program_counter + 1];

      (bite1 as u16) << 8 | bite2 as u16
    } else {
      0
    }
  }

  fn run_instruction(&mut self, instruction: u16) -> Step {
    let nibbles = (
      (instruction & 0xF000) >> 12 as u8,
      (instruction & 0x0F00) >> 8 as u8,
      (instruction & 0x00F0) >> 4 as u8,
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
      (0xE, _, 0xA, 0x1) => self.run_sknp(x), // SKNP Vx
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
    if let Some(ref mut gpu) = *self.bus.borrow().gpu.borrow_mut() {
      gpu.clear();
    }

    Step::Next
  }

  /// RET
  ///
  /// Return from a subroutine
  ///
  fn run_ret(&mut self) -> Step {
    self.stack_pointer -= 1;
    Step::Jump(self.stack[self.stack_pointer] as usize)
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
    self.stack[self.stack_pointer] = self.program_counter as u16 + 2;
    self.stack_pointer += 1;

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
    let vx_val = self.reg8[vx] as u16;
    let ret_val = vx_val + byte as u16;
    self.reg8[vx] = ret_val as u8;

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
    self.reg8[vx] ^= self.reg8[vy];
    Step::Next
  }

  /// ADD Vx, Vy
  fn run_add_reg8(&mut self, vx: usize, vy: usize) -> Step {
    // TODO: Find a better way to check if VX+VY > 255

    let vx_val = self.reg8[vx] as u16;
    let vy_val = self.reg8[vy] as u16;
    let ret_val = vx_val + vy_val;

    self.reg8[vx] = ret_val as u8;
    self.reg8[VF] = if ret_val > 0xFF { 1 } else { 0 };

    Step::Next
  }

  /// SUB Vx, Vy
  fn run_sub_reg8(&mut self, vx: usize, vy: usize) -> Step {
    let vx_val = self.reg8[vx];
    let vy_val = self.reg8[vy];

    self.reg8[VF] = if vx_val > vy_val { 1 } else { 0 };
    self.reg8[vx] = vx_val.wrapping_sub(vy_val);

    Step::Next
  }

  /// SHR Vx, {, Vy }
  ///
  /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx
  /// is divided by 2.
  ///
  fn run_shr_reg8(&mut self, vx: usize, vy: usize) -> Step {
    self.reg8[VF] = self.reg8[vx] & 1;
    self.reg8[vx] >>= 1;

    Step::Next
  }

  /// SUBN Vx, Vy
  ///
  /// Set Vx = Vy - Vx, set VF = NOT borrow.
  ///
  fn run_subn_reg8(&mut self, vx: usize, vy: usize) -> Step {
    let vx_value = self.reg8[vx];
    let vy_value = self.reg8[vy];

    self.reg8[VF] = if vy_value > vx_value { 1 } else { 0 };
    self.reg8[vx] = vy_value.wrapping_sub(vx_value);

    Step::Next
  }

  /// SHL Vx, {, Vy}
  ///
  /// Set Vx = Vx SHL 1.
  ///
  fn run_shl_reg8(&mut self, vx: usize, vy: usize) -> Step {
    self.reg8[VF] = self.reg8[vy] & 0x80 >> 7;
    self.reg8[vx] <<= 1;

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
    let mut rng = thread_rng();
    self.reg8[vx] = rng.gen::<u8>() & byte;
    Step::Next
  }

  /// DRW Vx, Vy, nibble
  ///
  /// Display n-byte sprite starting at memory location I at (Vx, Fy), set VF = collision.
  ///
  fn run_drw(&mut self, vx: usize, vy: usize, nibble: u8) -> Step {
    // Reset collision register
    self.reg8[VF] = 0;

    // The interpreter reads n bytes from memory, starting at the address stored in I.
    // These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
    // Sprites are XORed onto the existing screen. If this causes any pixels to be erased,
    // VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it
    // is outside the coordinates of the display, it wraps around to the opposite side of
    // the screen. See instruction 8xy3 for more information on XOR, and section 2.4,
    // Display, for more information on the Chip-8 screen and sprites.
    if let Some(ref ram) = *self.bus.borrow().ram.borrow() {
      if let Some(ref mut gpu) = *self.bus.borrow().gpu.borrow_mut() {
        for byte in 0..nibble as usize {
          let y = (self.reg8[vy] as usize + byte) % DISPLAY_HEIGHT;
          for bit in 0..8 as usize {
            let x = (self.reg8[vx] as usize + bit) % DISPLAY_WIDTH;
            let on = (ram.memory[self.reg16 as usize + byte] >> (7 - bit) as u8) & 1;

            self.reg8[VF] |= on & gpu.display[y][x] as u8;
            gpu.display[y][x] ^= on != 0;
          }
        }
      }
    }

    Step::Next
  }

  /// SKP Vx
  ///
  /// Skip next instruction if key with the value of Vx is pressed
  ///
  fn run_skp(&self, vx: usize) -> Step {
    if let Some(ref ipu) = *self.bus.borrow().ipu.borrow() {
      if ipu.is_key_pressed(self.reg8[vx]) {
        return Step::Skip;
      }
    }

    Step::Next
  }

  /// SKNP Vx
  ///
  /// Skip next instruction if key with the value of Vx is pressed
  ///
  fn run_sknp(&self, vx: usize) -> Step {
    if let Some(ref ipu) = *self.bus.borrow().ipu.borrow() {
      if !ipu.is_key_pressed(self.reg8[vx]) {
        return Step::Skip;
      }
    }

    Step::Next
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
    self.key_waiting = true;
    self.key_register = vx;

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
    self.reg8[VF] = if self.reg16 > 0x0F00 { 1 } else { 0 };

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
    self.reg16 = (self.reg8[vx] * 5) as u16;

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
    if let Some(ref mut ram) = *self.bus.borrow().ram.borrow_mut() {
      ram.memory[(self.reg16) as usize] = self.reg8[vx] / 100;
      ram.memory[(self.reg16 + 1) as usize] = (self.reg8[vx] % 100) / 10;
      ram.memory[(self.reg16 + 2) as usize] = self.reg8[vx] % 10;
    }

    Step::Next
  }

  /// LD [I], Vx
  ///
  /// Store registers V0 through Vx in memory starting at location I
  ///
  fn run_ld_reg16(&mut self, vx: usize) -> Step {
    if let Some(ref mut ram) = *self.bus.borrow().ram.borrow_mut() {
      let offset = self.reg16;

      for i in V0..vx + 1 as usize {
        ram.memory[offset as usize + i] = self.reg8[i];
      }
    }

    Step::Next
  }

  /// LD Vx, [I]
  ///
  /// Read registers V0 through Vx in memory starting at location I
  ///
  fn run_ld_mem(&mut self, vx: usize) -> Step {
    if let Some(ref ram) = *self.bus.borrow().ram.borrow() {
      let offset = self.reg16;

      for i in V0..vx + 1 as usize {
        self.reg8[i] = ram.memory[offset as usize + i];
      }
    }

    Step::Next
  }
}
