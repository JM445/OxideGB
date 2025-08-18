use super::memory::*;

use crate::debugger::Debugger;

#[derive(Debug, Default)]
pub struct Ppu {}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Hash)]
pub enum Mode {
    Mode0 = 0,
    Mode1 = 1,
    Mode2 = 2,
    Mode3 = 3
}
impl Ppu {
    pub fn tick<T>(&mut self, bus: &mut Bus, dbg: &mut T)
    where T: Debugger {
        ()
    }
}

impl Bus {
    fn set_ppu_mode(&mut self, mode: Mode) {
        let cur = self.ioregs[0x41] & 0b11111100;
        match mode {
            Mode::Mode0 => self.ioregs[0x41] = cur,
            Mode::Mode1 => self.ioregs[0x41] = cur & 0b01,
            Mode::Mode2 => self.ioregs[0x41] = cur & 0b10,
            Mode::Mode3 => self.ioregs[0x41] = cur & 0b11,
        }
    }
}