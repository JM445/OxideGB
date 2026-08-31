pub mod pixels;
mod displays;
mod fetcher;

use std::collections::VecDeque;
use crate::debugger::DebugEvent::{PpuModeChanged, SpriteScanned};
use super::memory::*;

use crate::debugger::Debugger;
use crate::emulator::cpu::interrupt::Interrupt;
use crate::emulator::memory::regdefines::*;
use crate::emulator::ppu::pixels::*;
use crate::emulator::ppu::fetcher::*;

pub const GB_W: usize = 160;
pub const GB_H: usize = 144;
pub const FB_LEN: usize = GB_W * GB_H;
pub type Frame = Box<[GBColor]>;
#[derive(Debug, Default)]
pub struct Ppu {
    frame: Frame,
    frame_dot: usize,
    mode_dot: usize,

    pixel_fetcher: PixelFetcher,
    next_x: usize,
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
            frame: vec![GBColor::OFF; FB_LEN].into_boxed_slice(),
            frame_dot: 0,
            mode_dot: 0,
            pixel_fetcher: PixelFetcher::new(),
            next_x: 0,
        }
    }
    pub fn tick<T>(&mut self, bus: &mut Bus, dbg: &mut T)
    where T: Debugger {
        if self.frame_dot >= 70224 { // OAM Scan Mode
            self.frame_dot = 0;
            bus.set_regs(LY, 0);
            self.send_frame(bus);
            self.set_ppu_mode(Mode::Mode2, bus, dbg);
        } else if self.frame_dot % 456 == 0 { // End of scanline, back to OAM Scan Mode or VBlank
            bus.set_regs(LY, bus.read(LY) + 1);
            if bus.read(LY) == 144 {
                self.set_ppu_mode(Mode::Mode1, bus, dbg);
            } else {
                self.set_ppu_mode(Mode::Mode2, bus, dbg);
            }
        } else if self.frame_dot % 456 == 80 && bus.read(LY) < 144 { // Pixel drawing mode
            self.set_ppu_mode(Mode::Mode3, bus, dbg);
        } else if self.next_x >= GB_W {
            self.set_ppu_mode(Mode::Mode0, bus, dbg);
        }

        match bus.get_ppu_mode() {
            Mode::Mode2 => self.tick_oam_scan(bus, dbg),
            Mode::Mode3 => self.tick_pixel_draw(bus, dbg),
            _ => ()
        }

        self.frame_dot += 1;
        self.mode_dot  += 1;
    }


    fn tick_oam_scan<T>(&mut self, bus: &mut Bus, dbg: &mut T)
    where T: Debugger {
        if self.mode_dot % 2 == 0 {
            let cur_sprite = Sprite::new(bus, (self.mode_dot / 2) as u8);
            let ysize = if (bus.read(LCDC) & 0b100) == 0 { 8u8 } else { 16u8 };
            let ly = bus.read(LY);
            if cur_sprite.y <= ly && cur_sprite.y + ysize > ly {
                self.pixel_fetcher.add_sprite(cur_sprite.clone());
                dbg.on_ppu_event(SpriteScanned(cur_sprite), self, bus);
            }
        }
    }

    fn tick_pixel_draw<T>(&mut self, bus: &mut Bus, dbg: &mut T)
    where T: Debugger {
        if let Some(pixel) = self.pixel_fetcher.tick(self.next_x, bus, dbg) {
            let ly = bus.read(LY) as usize;
            self.frame[ly * GB_W + self.next_x] = pixel;
            self.next_x += 1;
        }
    }

    fn send_frame(&mut self, bus: &mut Bus) {
        let cur = std::mem::replace(&mut self.frame, vec![GBColor::OFF; FB_LEN].into_boxed_slice());
        bus.send_frame(cur);
    }

    fn set_ppu_mode<T>(&mut self, mode: Mode, bus: &mut Bus, dbg: &mut T)
    where T: Debugger {
        self.mode_dot = 0;
        bus.set_ppu_mode(mode);
        match mode {
            Mode::Mode2 => {
                self.pixel_fetcher.reset();
                self.next_x = 0;
            },
            Mode::Mode1 => {
                bus.set_interrupt(Interrupt::VBlank);
            },
            _ => ()
        }
        dbg.on_ppu_event(PpuModeChanged(mode), self, bus)
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