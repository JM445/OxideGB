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

    debugger: DebuggerKind,
}

impl Emulator {
    pub fn new<P: AsRef<Path>>(rom_path: P) -> Result<Self, String> {
        Ok(Emulator{
            cpu: Cpu::new(),
            bus: Bus::new(rom_path)?,
            ppu: Default::default(),

            debugger: DebuggerKind::Log(LogDebugger::default())
        })
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.bus, &mut self.debugger);
        self.ppu.tick(&mut self.bus, &mut self.debugger);
    }
}
