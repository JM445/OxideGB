pub mod memory;
pub mod ppu;

pub mod cpu;
pub mod test_roms;

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
    pub fn new<P: AsRef<Path>>(rom_path: P, boot_path: P) -> Result<Self, String> {
        let bus = Bus::new(rom_path, boot_path)?;
        Ok(Emulator{
            cpu: Cpu::new(if bus.boot_enabled {0x0000} else {0x0100}),
            bus,
            ppu: Default::default(),
        })
    }

    pub fn tick<T>(&mut self, dbg: &mut T)
    where T: Debugger {
        self.cpu.tick(&mut self.bus, dbg);
        self.bus.tick_serial();
        self.ppu.tick(&mut self.bus, dbg);
    }
}
