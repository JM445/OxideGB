pub mod memory;
pub mod ppu;

pub mod cpu;

use memory::*;
use ppu::*;
use cpu::*;

use crate::debugger::*;


use std::path::Path;

pub struct Emulator {
    pub cpu: Cpu,
    pub bus: Bus,
    pub ppu: Ppu,
}

impl Emulator {
    pub fn new<P: AsRef<Path>>(rom_path: P) -> Result<Self, String> {
        Ok(Emulator{
            cpu: Cpu::new(),
            bus: Bus::new(rom_path)?,
            ppu: Default::default(),
        })
    }

    pub fn tick<T>(&mut self, dbg: &mut T)
    where T: Debugger {
        self.cpu.tick(&mut self.bus, dbg);
        self.ppu.tick(&mut self.bus, dbg);
    }
}
