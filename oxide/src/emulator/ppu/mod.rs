pub mod pixels;
mod displays;
mod fetcher;

use log::debug;
use crate::debugger::DebugEvent::{PpuActivated, PpuModeChanged, SpriteScanned};
use super::memory::*;

use crate::debugger::{DebugEvent, Debugger};
use crate::emulator::cpu::interrupt::Interrupt;
use crate::emulator::memory::regdefines::*;
use crate::emulator::ppu::pixels::*;
use crate::emulator::ppu::fetcher::*;

pub const GB_W: u8 = 160;
pub const GB_H: u8 = 144;
pub const FB_LEN: usize = GB_W as usize * GB_H as usize;
pub type Frame = Box<[GBColor]>;
#[derive(Debug, Default)]
pub struct Ppu {
    frame: Frame,
    frame_dot: usize,
    mode_dot: usize,

    pixel_fetcher: PixelFetcher,
    next_x: u8,

    was_on: bool,
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
            was_on: false,
        }
    }
    pub fn tick<T>(&mut self, bus: &mut Bus, dbg: &mut T) -> Option<Frame>
    where T: Debugger {
        let mut res = None;

        if bus.read(LCDC) & 0b10000000 == 0 {
            self.was_on = false;
            return None;
        }
        if !self.was_on {
            dbg.on_ppu_event(PpuActivated(), self, bus);
            self.frame_dot = 0;
            self.mode_dot = 0;
            self.pixel_fetcher.reset_frame();
            self.next_x = 0;
            self.set_ppu_mode(Mode::Mode2, bus, dbg);
            bus.set_regs(LY, 0);
        } else if self.frame_dot >= 70224 { // OAM Scan Mode
            self.frame_dot = 0;
            bus.set_regs(LY, 0);
            dbg.on_ppu_event(DebugEvent::FrameSent(), self, bus);
            self.set_ppu_mode(Mode::Mode2, bus, dbg);
            res = Some(std::mem::replace(&mut self.frame, vec![GBColor::OFF; FB_LEN].into_boxed_slice()));
        } else if self.frame_dot % 456 == 0 { // End of scanline, back to OAM Scan Mode or VBlank
            bus.set_regs(LY, bus.read(LY) + 1);
            self.pixel_fetcher.end_of_line();
            if bus.read(LY) == 144 {
                self.set_ppu_mode(Mode::Mode1, bus, dbg);
            } else if bus.read(LY) < 144 {
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
        self.was_on = true;
        res
    }
    fn tick_oam_scan<T>(&mut self, bus: &mut Bus, dbg: &mut T)
    where T: Debugger {
        if self.mode_dot % 2 == 0 {
            let cur_sprite = Sprite::new(bus, (self.mode_dot / 2) as u8);
            let ysize = if (bus.read(LCDC) & 0b100) == 0 { 8u8 } else { 16u8 };
            let ly = bus.read(LY);
            if cur_sprite.y <= ly + 16 && cur_sprite.y + ysize > ly + 16 {
                self.pixel_fetcher.add_sprite(cur_sprite.clone());
                dbg.on_ppu_event(SpriteScanned(cur_sprite), self, bus);
            }
        }
    }

    fn tick_pixel_draw<T>(&mut self, bus: &mut Bus, dbg: &mut T)
    where T: Debugger {
        if let Some(pixel) = self.pixel_fetcher.tick(self.next_x, bus, dbg) {
            let ly = bus.read(LY) as usize;
            self.frame[ly * GB_W as usize + self.next_x as usize] = pixel;
            debug!("Pixel Sent: X = {}, Y = {}, {}", self.next_x, bus.read(LY), pixel);
            self.next_x += 1;
        }
    }

    fn send_frame(&mut self, bus: &mut Bus) {
    }

    fn set_ppu_mode<T>(&mut self, mode: Mode, bus: &mut Bus, dbg: &mut T)
    where T: Debugger {
        self.mode_dot = 0;
        bus.set_ppu_mode(mode);
        match mode {
            Mode::Mode2 => {
                self.pixel_fetcher.reset_line();
                self.next_x = 0;
            },
            Mode::Mode1 => {
                bus.set_interrupt(Interrupt::VBlank);
                self.pixel_fetcher.reset_frame();
            },
            Mode::Mode0 => {
                self.next_x = 0;
            }
            _ => ()
        }
        dbg.on_ppu_event(PpuModeChanged(mode), self, bus)
    }
}
