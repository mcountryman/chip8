use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::emulator::cpu::Cpu;
use crate::emulator::ipu::Ipu;
use crate::emulator::ram::Ram;

pub mod cpu;
pub mod ram;
pub mod ipu;

pub struct Emulator {
  pub ipu: Rc<RefCell<Ipu>>,
  pub cpu: Rc<RefCell<Cpu>>,
  pub ram: Rc<RefCell<Ram>>,
}

impl Emulator {
  pub fn new() -> Self {
    let ram = Rc::new(RefCell::new(Ram::new()));
    let ipu = Rc::new(RefCell::new(Ipu::new()));

    Self {
      ipu: ipu.clone(),
      ram: ram.clone(),
      cpu: Rc::new(RefCell::new(Cpu::new(ram, ipu))),
    }
  }
}