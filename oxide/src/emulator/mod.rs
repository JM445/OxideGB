pub mod memory;
pub mod ppu;
pub mod cpu;

use memory::*;
use ppu::*;
use cpu::*;

use crate::debugger::*;


use std::path::Path;

pub struct Emulator {
    cpu: Cpu,
    bus: Bus,
    ppu: Ppu,
}

impl Emulator {
    pub fn new<P: AsRef<Path>>(rom_path: P) -> Result<Self, String> {
        Ok(Emulator{
            cpu: Cpu::new(),
            bus: Bus::new(rom_path)?,
            ppu: Default::default(),
        })
    }

    pub fn tick(&mut self, dbg: &mut DebuggerKind) {
        self.cpu.tick(&mut self.bus, dbg);
        self.ppu.tick(&mut self.bus, dbg);
    }
}
