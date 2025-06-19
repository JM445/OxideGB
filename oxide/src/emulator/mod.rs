pub mod memory;
pub mod ppu;

pub mod cpu;

use cpu::*;
use memory::*;
use ppu::*;

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
        let cpu = if bus.boot_enabled {
            Cpu::new_boot()
        } else {
            Cpu::new_noboot()
        };
        
        Ok(Emulator{
            cpu,
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
