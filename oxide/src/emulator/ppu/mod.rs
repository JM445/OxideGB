mod pixels;

use std::collections::VecDeque;
use super::memory::*;

use crate::debugger::Debugger;
use crate::emulator::cpu::interrupt::Interrupt;
use crate::emulator::memory::regdefines::LY;
use crate::emulator::ppu::pixels::PixelInfo;

pub const GB_W: usize = 160;
pub const GB_H: usize = 144;
pub const FB_LEN: usize = GB_W * GB_H;
pub type Frame = Box<[u32]>; // RGBA8888
#[derive(Debug, Default)]
pub struct Ppu {
    frame: Frame,
    cur_dot: usize,

    bg_fifo: VecDeque<PixelInfo>,
    obj_fifo: VecDeque<PixelInfo>,


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
            frame: vec![0u32; FB_LEN].into_boxed_slice(),
            cur_dot: 0,
            bg_fifo: VecDeque::new(),
            obj_fifo: VecDeque::new(),
        }
    }
    pub fn tick<T>(&mut self, bus: &mut Bus, dbg: &mut T)
    where T: Debugger {
        
        self.cur_dot += 1;
        if self.cur_dot >= 70224 { // OAM Scan Mode
            self.cur_dot = 0;
            bus.set_regs(LY, 0);
            self.send_frame(bus);
            bus.set_ppu_mode(Mode::Mode2);
        } else if self.cur_dot % 456 == 0 { // End of scanline, back to OAM Scan Mode of VBlank
            bus.set_regs(LY, bus.read(LY) + 1);
            if bus.read(LY) == 144 {
                bus.set_interrupt(Interrupt::VBlank);
                bus.set_ppu_mode(Mode::Mode1);
            } else {
                bus.set_ppu_mode(Mode::Mode2);
            }
        } else if self.cur_dot % 456 == 80 && bus.read(LY) < 144 { // Pixel drawing mode
            bus.set_ppu_mode(Mode::Mode3);
        }
    }
    
    fn send_frame(&mut self, bus: &mut Bus) {
        let cur = std::mem::replace(&mut self.frame, vec![0u32; FB_LEN].into_boxed_slice());
        bus.send_frame(cur);
    }


}

impl Bus {
    fn set_ppu_mode(&mut self, mode: Mode) {
        let cur = self.ioregs[0x41] & 0b11111100;
        self.ioregs[0x41] = cur | match mode {
            Mode::Mode0 => 0b00,
            Mode::Mode1 => 0b01,
            Mode::Mode2 => 0b10,
            Mode::Mode3 => 0b11,
        }
    }
}