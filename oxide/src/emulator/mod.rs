pub mod memory;
pub mod ppu;
pub mod cpu;

use memory::*;
use ppu::*;
use cpu::*;

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
}
