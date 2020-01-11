use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::emulator::cpu::Cpu;
use crate::emulator::gpu::Gpu;
use crate::emulator::ipu::Ipu;
use crate::emulator::ram::Ram;

pub mod cpu;
pub mod ram;
pub mod ipu;
pub mod gpu;

pub struct Emulator {
  pub gpu: Rc<RefCell<Gpu>>,
  pub ipu: Rc<RefCell<Ipu>>,
  pub cpu: Rc<RefCell<Cpu>>,
  pub ram: Rc<RefCell<Ram>>,
}

impl Emulator {
  pub fn new() -> Self {
    let ram = Rc::new(RefCell::new(Ram::new()));
    let ipu = Rc::new(RefCell::new(Ipu::new()));
    let gpu = Rc::new(RefCell::new(Gpu::new()));

    Self {
      gpu: gpu.clone(),
      ipu: ipu.clone(),
      ram: ram.clone(),
      cpu: Rc::new(RefCell::new(Cpu::new(ram, ipu, gpu))),
    }
  }
}