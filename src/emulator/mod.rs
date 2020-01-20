use std::cell::{Ref, RefCell};
use std::fs::File;
use std::io::Result;
use std::path::Path;
use std::rc::Rc;

use crate::emulator::bus::Bus;
use crate::emulator::cpu::Cpu;
use crate::emulator::gpu::Gpu;
use crate::emulator::ipu::Ipu;
use crate::emulator::ram::Ram;

pub mod cpu;
pub mod ram;
pub mod ipu;
pub mod gpu;
pub mod bus;

pub struct Emulator {
  pub bus: Rc<RefCell<Bus>>,
}

impl Emulator {
  pub fn new() -> Self {
    Self {
      bus: Bus::new(),
    }
  }

  pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
    let mut file = File::open(path)?;

    self.reset();

    if let Some(ref mut ram) = *self.bus.borrow().ram.borrow_mut() {
      ram.load(&mut file)?;
    }

    Ok(())
  }

  pub fn reset(&mut self) {
    if let Some(ref mut cpu) = *self.bus.borrow().cpu.borrow_mut() {
      cpu.reset();
    }

    if let Some(ref mut gpu) = *self.bus.borrow().gpu.borrow_mut() {
      gpu.clear();
    }

    if let Some(ref mut ram) = *self.bus.borrow().ram.borrow_mut() {
      ram.reset();
    }
  }

  pub fn draw(&mut self) -> Result<()> {
    Ok(())
  }

  pub fn update(&mut self) -> Result<()> {
    if let Some(ref mut cpu) = *self.bus.borrow().cpu.borrow_mut() {
      cpu.tick()?;
    }

    Ok(())
  }
}