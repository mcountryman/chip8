use std::cell::RefCell;
use std::rc::Rc;
use crate::emulator::cpu::Cpu;
use crate::emulator::gpu::Gpu;
use crate::emulator::ipu::Ipu;
use crate::emulator::ram::Ram;

type BusRef = Rc<RefCell<Bus>>;

pub struct Bus {
  pub cpu: RefCell<Option<Cpu>>,
  pub gpu: RefCell<Option<Gpu>>,
  pub ipu: RefCell<Option<Ipu>>,
  pub ram: RefCell<Option<Ram>>,
}

impl Bus {
  pub fn new() -> Rc<RefCell<Self>> {
    let mut bus = Rc::new(RefCell::new(Self {
      cpu: RefCell::new(None),
      gpu: RefCell::new(None),
      ipu: RefCell::new(None),
      ram: RefCell::new(None),
    }));

    *bus.borrow_mut().cpu.borrow_mut() = Some(Cpu::new(bus.clone()));
    *bus.borrow_mut().gpu.borrow_mut() = Some(Gpu::new(bus.clone()));
    *bus.borrow_mut().ipu.borrow_mut() = Some(Ipu::new(bus.clone()));
    *bus.borrow_mut().ram.borrow_mut() = Some(Ram::new());
    bus
  }
}