pub mod memory;
pub mod ppu;

pub mod cpu;
pub mod internals;

use cpu::*;
use memory::*;
use ppu::*;

use crate::debugger::*;


use crate::emu_print;
use crate::emulator::internals::iomanager::IoManager;
use crate::emulator::internals::timer::Timer;
use crate::settings::GLOB_SETTINGS;
use std::path::Path;

pub struct Emulator {
    pub cpu: Cpu,
    pub bus: Bus,
    pub ppu: Ppu,
    
    pub timer: Timer,
    
    pub ticks: usize
}

impl Emulator {
    pub fn new<P: AsRef<Path>>(rom_path: P, boot_path: P, io_manager: IoManager) -> Result<Self, String> {
        let bus = Bus::new(rom_path, boot_path, io_manager)?;
        let cpu = if bus.boot_enabled {
            Cpu::new_boot()
        } else {
            Cpu::new_noboot()
        };

        if GLOB_SETTINGS.get().unwrap().doctor_logs {
            emu_print!("{}", cpu.get_doctor_log(&bus))
        }

        Ok(Emulator{
            cpu,
            bus,
            ppu: Default::default(),
            timer: Timer::default(),
            
            ticks: 0
        })
    }
    
    pub fn get_t_cycle(&self) -> usize {
        self.ticks
    }
    
    pub fn get_m_cycle(&self) -> usize {
        self.ticks / 4
    }

    pub fn tick<T>(&mut self, dbg: &mut T)
    where T: Debugger {
        self.ticks = self.ticks.wrapping_add(1);
        
        if self.ticks & 0b11 == 0 { // M-Cycle
            self.cpu.tick(&mut self.bus, dbg);
        }
        
        // T-Cycle
        self.bus.tick_serial();
        self.ppu.tick(&mut self.bus, dbg);
        self.timer.tick(&mut self.bus);
    }
}
