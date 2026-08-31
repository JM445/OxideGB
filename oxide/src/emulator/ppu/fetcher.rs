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
    state: FetchState,
    fetching_obj: bool,
    fetching_x: u8,
    fetching_id: u8,
    fetching_low: u8,
    fetching_high: u8
}

impl PixelFetcher {
    pub fn new() -> Self {
        PixelFetcher {
            ready: false,
            bg_fifo: VecDeque::new(),
            obj_fifo: VecDeque::new(),
            line_sprites: VecDeque::new(),
            dot: 0,
            state: FetchState::TILE,
            fetching_obj: false,
            fetching_x: 0, // Currently fetched tile left-most position, different from currently drawn pixel
            fetching_id: 0,
            fetching_low: 0,
            fetching_high: 0,
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

    // Get the tile ID of the currently fetched BG or Window tile (fetcher not in obj mode)
    fn get_bg_tile_id(&mut self, bus: &Bus) -> u8 {
        let lcdc = bus.read(LCDC);
        let ly = bus.read(LY);
        let window_enabled = lcdc & 0b100000 != 0;
        let window_map_address = if lcdc & 0b1000000 == 0 {0x9800} else {0x9C00};
        let bg_map_address = if lcdc & 0b1000 == 0 {0x9800} else {0x9C00};
        let is_in_window = self.fetching_x >= bus.read(WX) - 7 && ly >= bus.read(WY);

        if window_enabled && is_in_window {

        } else {}
        0x00
    }

    pub fn tick<T>(&mut self, pixel_x: u8, bus: &mut Bus, dbg: &mut T) -> Option<GBColor>
    where T: Debugger {
        if self.dot % 2 == 0 {
            match (self.state, self.fetching_obj) {
                (FetchState::TILE, false) => {
                    self.fetching_id = self.get_bg_tile_id(bus);
                    self.state = FetchState::LOW;
                },
                _ => ()
            }
        }

        self.dot += 1;
        self.render_pixel(bus)
    }
}