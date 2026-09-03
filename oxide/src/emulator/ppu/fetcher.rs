use std::collections::VecDeque;
use log::debug;
use crate::debugger::Debugger;
use crate::emulator::memory::Bus;
use crate::emulator::memory::regdefines::*;
use crate::emulator::ppu::pixels::*;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Hash, Default)]
enum FetchState {
    #[default] TILE = 0,
    TILE2 = 1,
    HIGH = 2,
    HIGH2 = 3,
    LOW = 4,
    LOW2 = 5,
    PUSH = 6,
}

#[derive(Debug)]
pub struct PixelFetcher {
    ready: bool,
    bg_fifo: VecDeque<PixelInfo>,
    obj_fifo: VecDeque<PixelInfo>,

    line_sprites: VecDeque<Sprite>,

    dot: u8,
    state: FetchState,
    wait_for_obj: bool,
    fetching_obj: bool,
    fetching_win: bool,

    fetching_id: u8,
    fetching_low: u8,
    fetching_high: u8,

    waiting_low: u8,    // Stores "fetching_low" before push, used to clear the fetcher when sprite fetch begins
    waiting_high: u8,   // Stores "fetching_high" before push, used to clear the fetcher when sprite fetch begins

    bg_fetch_x: u8,     // bg tile x offset for the current scanline
    win_fetch_x: u8,    // win tile x offset for the current scanline

    wly: u8,            // Reached row in Window (in pixels)
    win_drawn: bool,    // A window pixel has been pushed to FIFO this scanline
}

impl Default for PixelFetcher {
    fn default() -> Self {
        Self::new()
    }
}
impl PixelFetcher {
    pub fn new() -> Self {
        let mut filled_fifo = VecDeque::new();
        for _ in 0..8 {
            filled_fifo.push_back(PixelInfo::default());
        }
        PixelFetcher {
            ready: false,
            bg_fifo: VecDeque::new(),
            obj_fifo: filled_fifo,
            line_sprites: VecDeque::new(),
            dot: 0,
            state: FetchState::TILE,
            wait_for_obj: false,
            fetching_obj: false,
            fetching_win: false,

            fetching_id: 0,
            fetching_low: 0,
            fetching_high: 0,

            waiting_high: 0,
            waiting_low: 0,

            bg_fetch_x: 0,
            win_fetch_x: 0,

            wly: 0,
            win_drawn: false,
        }
    }
    pub fn add_sprite(&mut self, sprite: Sprite) {
        let idx = self.line_sprites.partition_point(|e| e.x <= sprite.x);
        self.line_sprites.insert(idx, sprite);
    }
    pub fn end_of_line(&mut self) {
        if self.win_drawn {
            self.wly += 1;
        }
    }

    pub fn reset_line(&mut self) {
        self.bg_fifo.clear();
        self.obj_fifo.clear();
        for _ in 0..8 {
            self.obj_fifo.push_back(PixelInfo::default())
        }
        self.line_sprites.clear();
        self.dot = 0;
        self.state = FetchState::TILE;
        self.wait_for_obj = false;
        self.fetching_obj = false;
        self.fetching_id = 0;
        self.fetching_low = 0;
        self.fetching_high = 0;
        self.bg_fetch_x = 0;
        self.win_fetch_x = 0;
        self.win_drawn = false;
    }

    pub fn reset_frame(&mut self) {
        self.wly = 0;
        self.reset_line();
    }

    pub fn render_pixel(&mut self, bus: &Bus) -> Option<GBColor> {
        // OBJ fifo should always have 8 pixels
        debug_assert!(self.obj_fifo.len() == 8, "Invalid OBJ Fifo length: {}", self.obj_fifo.len());
        if !self.ready {
            None
        } else {
            let lcdc = bus.read(LCDC);
            let obj_enable = lcdc & 0b10 != 0;

            let bg = self.bg_fifo.pop_front().unwrap();
            let obj = self.obj_fifo.pop_front().unwrap();
            self.obj_fifo.push_back(PixelInfo::default());
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
    fn get_tile_id(&mut self, bus: &Bus) -> u8 {
        let lcdc = bus.read(LCDC);

        if self.fetching_obj {
            self.line_sprites.front().unwrap().tile_id
        } else if self.fetching_win {
            let row = ((self.wly / 8) % 32) as u16;
            let col = (self.win_fetch_x % 32) as u16;
            let win_map_address = if lcdc & 0b1000000 != 0 {0x9C00} else {0x9800};
            bus.ppu_read(win_map_address + (row * 32) + col)
        } else {
            let row = ((bus.read(SCY) as u16 + bus.read(LY) as u16) / 8) % 32;
            let col = (((bus.read(SCX) / 8) + self.bg_fetch_x) % 32) as u16;
            let bg_map_address = if lcdc & 0b1000 != 0 {0x9C00} else {0x9800};
            bus.ppu_read(bg_map_address + (row* 32) + col)
        }
    }

    // Retrieve both tile data bytes.
    // Not exact behavior but avoids computing tile address 2 times by fetch
    // Fetching high byte then should be no-op
    fn get_tile_data(&mut self, bus: &Bus) {
        let lcdc = bus.read(LCDC);
        let ly = bus.read(LY);

        // Get the current line offset in the tile
        let tile_y = if self.fetching_obj {
            let screen_y = self.line_sprites.front().unwrap().y as i16 - 16;
            let ty = (ly as i16 - screen_y) as u8;
            if self.line_sprites.front().unwrap().y_flip() {
                // TODO: Implement 8x16 behavior
                7 - ty
            } else {
                ty
            }
        } else if self.fetching_win {
            self.wly % 8
        } else {
            bus.read(SCY).wrapping_add(ly) % 8
        };

        // Get the tile base address
        let tile_addr = if self.fetching_obj || lcdc & 0b10000 != 0 {
            0x8000u16 + (self.fetching_id as u16 * 16)
        } else {
            0x9000u16.wrapping_add_signed(i16::from(self.fetching_id as i8) * 16)
        };

        // Retrieve the two bytes of that tile that we need
        self.fetching_low = bus.ppu_read(tile_addr + tile_y as u16 * 2);
        self.fetching_high = bus.ppu_read(tile_addr + 1 + tile_y as u16 * 2);

        if self.fetching_obj && self.line_sprites.front().unwrap().x_flip() {
            self.fetching_low = self.fetching_low.reverse_bits();
            self.fetching_high = self.fetching_high.reverse_bits();
        }
    }

    // Push a row of 8 pixels to bg or obj fifo
    pub fn push_tile_data(&mut self) {
        for i in 0u8..8 {
            let palette = if self.fetching_obj {
                self.line_sprites.front().unwrap().palette()
            } else {
                Palette::BGP
            };

            let pixel = PixelInfo::from_bytes(self.fetching_low,
                                              self.fetching_high,
                                              i,
                                              self.fetching_obj,
                                              palette
            );
            if !self.fetching_obj {
                self.bg_fifo.push_back(pixel);
            } else {
                let cur = &self.obj_fifo[i as usize];
                if cur.color_index == 0 {
                    self.obj_fifo[i as usize] = PixelInfo::from_bytes(
                        self.fetching_low,
                        self.fetching_high,
                        i,
                        self.fetching_obj,
                        palette
                    );
                }
            }
        }
    }

    pub fn tick<T>(&mut self, pixel_x: u8, bus: &mut Bus, _dbg: &mut T) -> Option<GBColor>
    where T: Debugger {
        let lcdc = bus.read(LCDC);
        let ly = bus.read(LY);
        let window_enabled = lcdc & 0b100000 != 0;
        let next_sprite_x = if let Some(s) = self.line_sprites.front() {
            s.x
        } else {
            255 // pixel_x should never go that high
        };

        // Did we reach the window ? If yes, clear the fifo and reset the fetcher
        if window_enabled && !self.fetching_win && pixel_x + 7 >= bus.read(WX) && ly >= bus.read(WY) {
            self.fetching_win = true;
            self.bg_fifo.clear();
            self.state = FetchState::TILE;
            self.dot = 0;
            self.win_fetch_x = 0;
        }

        // Did we reach a sprite ? If yes, wait for fetch end before switching to obj fetching
        // Source: https://www.reddit.com/r/EmuDev/comments/s6cpis/gameboy_trying_to_understand_sprite_fifo_behavior/
        if next_sprite_x <= pixel_x + 8 && !self.fetching_obj && !self.wait_for_obj {
            self.wait_for_obj = true;
            self.ready = false;
        }

        // We are waiting for bg fetch to end and it has reached push state
        // Save fetching bytes to waiting bytes and switch to obj fetch mode
        if self.wait_for_obj && self.state as u8 >= 5 {
            self.wait_for_obj = false;
            self.fetching_obj = true;
            self.waiting_low = self.fetching_low;
            self.waiting_high = self.fetching_high;
            self.state = FetchState::TILE;
        }

        match self.state {
            FetchState::TILE => {
                self.fetching_id = self.get_tile_id(bus);
                self.state = FetchState::TILE2;
                debug!("Fetched Tile ID: {}", self.fetching_id);
            },

            FetchState::TILE2 => self.state = FetchState::LOW,

            FetchState::LOW => {
                self.get_tile_data(bus);
                self.state = FetchState::LOW2;
                debug!("Fetched Tile Low: {:#04X}", self.fetching_low);
            },

            FetchState::LOW2 => self.state = FetchState::HIGH,

            FetchState::HIGH => {
                // No-op as high byte is already fetched in get_data()
                self.state = FetchState::HIGH2;
                debug!("Fetched Tile Low: {:#04X}", self.fetching_high);
            },

            FetchState::HIGH2 => self.state = FetchState::PUSH,

            FetchState::PUSH => {
                if !self.fetching_obj && self.bg_fifo.is_empty() {
                    self.push_tile_data();
                    if self.fetching_win {
                        self.win_fetch_x += 1;
                    } else {
                        self.bg_fetch_x += 1;
                    }
                    self.state = FetchState::TILE;
                    self.ready = true;
                    debug!("Pushed BG Tile to FIFO");
                } else if self.fetching_obj {
                    self.push_tile_data();
                    self.fetching_low = self.waiting_low;
                    self.fetching_high = self.waiting_high;
                    self.fetching_obj = false;
                    self.ready = !self.bg_fifo.is_empty();
                    self.line_sprites.pop_front();
                    debug!("Pushed OBJ Tile to FIFO");
                } else {
                    debug!("Fetcher Waiting")
                }
            },
        }

        if self.bg_fifo.is_empty() {
            self.ready = false;
        }
        self.dot += 1;
        self.dot %= 8;
        let res = self.render_pixel(bus);

        res
    }
}