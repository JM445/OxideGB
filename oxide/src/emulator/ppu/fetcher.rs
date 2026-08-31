use std::collections::VecDeque;
use crate::debugger::Debugger;
use crate::emulator::memory::Bus;
use crate::emulator::memory::regdefines::*;
use crate::emulator::ppu::pixels::*;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Hash, Default)]
enum FetchState {
    #[default] TILE = 0,
    HIGH = 1,
    LOW = 2,
    PUSH = 3,
}

#[derive(Debug, Default)]
pub struct PixelFetcher {
    ready: bool,
    bg_fifo: VecDeque<PixelInfo>,
    obj_fifo: VecDeque<PixelInfo>,

    line_sprites: VecDeque<Sprite>,

    dot: u8,
    state: FetchState
}

impl PixelFetcher {
    pub fn new() -> Self {
        PixelFetcher {
            ready: false,
            bg_fifo: VecDeque::new(),
            obj_fifo: VecDeque::new(),
            line_sprites: VecDeque::new(),
            dot: 0,
            state: FetchState::TILE
        }
    }
    pub fn add_sprite(&mut self, sprite: Sprite) {
        self.line_sprites.push_back(sprite)
    }

    pub fn reset(&mut self) {
        self.bg_fifo.clear();
        self.obj_fifo.clear();
        self.line_sprites.clear();
    }

    pub fn render_pixel(&mut self, bus: &Bus) -> Option<GBColor> {
        if !self.ready {
            None
        } else {
            let lcdc = bus.read(LCDC);
            let obj_enable = lcdc & 0b10 != 0;
            let bg_enable = lcdc & 0b1 != 0;

            if self.obj_fifo.is_empty() {
                self.obj_fifo.push_back(PixelInfo::default())
            }
            let bg = self.bg_fifo.pop_front().unwrap();
            let obj = self.obj_fifo.pop_front().unwrap();
            if self.bg_fifo.is_empty() {
                self.ready = false;
            }

            if obj.color_index == 0 || obj.priority == Priority::BG || obj_enable == false {
                Some(GBColor::from_pixel(bg, bus))
            } else {
                Some(GBColor::from_pixel(obj, bus))
            }
        }
    }

    pub fn tick<T>(&mut self, pixel_x: usize, bus: &mut Bus, dbg: &mut T) -> Option<GBColor>
    where T: Debugger {
        if self.dot % 2 == 0 {

        }

        self.render_pixel(bus)
    }
}