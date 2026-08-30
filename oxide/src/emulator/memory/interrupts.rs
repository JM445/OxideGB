use super::*;
use crate::{emu_print, GLOB_SETTINGS};
use crate::emulator::cpu::interrupt::Interrupt;

impl Bus {
    pub fn tick_serial(&mut self) {
        match self.ioregs[0x02] & 0x83 {
            0x81 => {
                if GLOB_SETTINGS.get().unwrap().print_serial {
                    let c = self.ioregs[0x01] as char;
                    emu_print!("{c}");
                    self.ioregs[0x01] = 0xFF;
                    self.ioregs[0x02] &= 0x7F;
                }
            },
            _ => ()
        }
    }
    
    pub fn tick_stat(&mut self) {
        let stat = self.read(STAT);
        let stat_active =
            (stat & 0b0100_0000 != 0 && stat & 0b0100 != 0) ||
                (stat & 0b0010_0000 != 0 && self.get_ppu_mode() == Mode::Mode2) ||
                (stat & 0b0001_0000 != 0 && self.get_ppu_mode() == Mode::Mode1) ||
                (stat & 0b1000 != 0 && self.get_ppu_mode() == Mode::Mode0);
        if stat_active && !self.last_stat {
            self.set_interrupt(Interrupt::LCD)
        }
        self.last_stat = stat_active;
    }
}