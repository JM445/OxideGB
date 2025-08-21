use super::memory::*;

use crate::debugger::Debugger;

pub const GB_W: usize = 160;
pub const GB_H: usize = 144;
pub const FB_LEN: usize = GB_W * GB_H;
pub type Frame = Box<[u32]>; // RGBA8888
#[derive(Debug, Default)]
pub struct Ppu {
    frame: Frame,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Hash)]
pub enum Mode {
    Mode0 = 0,
    Mode1 = 1,
    Mode2 = 2,
    Mode3 = 3
}
impl Ppu {
    pub fn new() -> Ppu {
        Ppu{
            frame: vec![0u32; FB_LEN].into_boxed_slice()
        }
    }
    pub fn tick<T>(&mut self, bus: &mut Bus, dbg: &mut T)
    where T: Debugger {
        ()
    }
    
    fn send_frame(&mut self, bus: &mut Bus) {
        let cur = std::mem::replace(&mut self.frame, vec![0u32; FB_LEN].into_boxed_slice());
        bus.send_frame(cur);
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