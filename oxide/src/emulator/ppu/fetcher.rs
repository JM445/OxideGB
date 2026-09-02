use std::collections::VecDeque;
use log::debug;
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
    wait_for_obj: bool,
    fetching_obj: bool,
    fetching_win: bool,

    fetching_id: u8,
    fetching_low: u8,
    fetching_high: u8,

    bg_fetch_x: u8,     // bg tile x offset for the current scanline
    win_fetch_x: u8,    // win tile x offset for the current scanline

    wly: u8,            // Reached row in Window (in pixels)
    win_drawn: bool,    // A window pixel has been pushed to FIFO this scanline
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
            wait_for_obj: false,
            fetching_obj: false,
            fetching_win: false,

            fetching_id: 0,
            fetching_low: 0,
            fetching_high: 0,

            bg_fetch_x: 0,
            win_fetch_x: 0,

            wly: 0,
            win_drawn: false,
        }
    }
    pub fn add_sprite(&mut self, sprite: Sprite) {
        self.line_sprites.push_back(sprite)
    }

    pub fn reset_line(&mut self) {
        self.bg_fifo.clear();
        self.obj_fifo.clear();
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
        if !self.ready {
            None
        } else {
            let lcdc = bus.read(LCDC);
            let obj_enable = lcdc & 0b10 != 0;

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
    fn get_tile_id(&mut self, bus: &Bus) -> u8 {
        let lcdc = bus.read(LCDC);

        if self.fetching_obj {
            0x00 // TODO: Implement real sprite fetching
        } else if self.fetching_win {
            let row = ((self.wly / 8) % 32) as u16;
            let col = (self.win_fetch_x % 32) as u16;
            let win_map_address = if lcdc & 0b1000000 != 0 {0x9C00} else {0x9800};
            bus.read(win_map_address + (row * 32) + col)
        } else {
            let row = ((bus.read(SCY) as u16 + bus.read(LY) as u16) / 8) % 32;
            let col = (((bus.read(SCX) / 8) + self.bg_fetch_x) % 32) as u16;
            let bg_map_address = if lcdc & 0b1000 != 0 {0x9C00} else {0x9800};
            bus.read(bg_map_address + (row* 32) + col)
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
            0x00u8 // TODO
        } else if self.fetching_win {
            self.wly % 8
        } else {
            (bus.read(SCY) + ly) % 8
        };

        // Get the tile base address
        let tile_addr = if self.fetching_obj || lcdc & 0b10000 == 0 {
            0x8000u16 + self.fetching_id as u16
        } else {
            0x8800u16.wrapping_add_signed(i16::from(self.fetching_id as i8))
        };

        // Retrieve the two bytes of that tile that we need
        self.fetching_low = bus.read(tile_addr + tile_y as u16 * 2);
        self.fetching_high = bus.read(tile_addr + 1 + tile_y as u16 * 2);
    }

    // Push a row of 8 pixels to bg or obj fifo
    pub fn push_tile_data(&mut self) {
        if self.fetching_obj {self.bg_fifo.clear()}
        let fifo = if self.fetching_obj {&mut self.obj_fifo} else {&mut self.bg_fifo};
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
            fifo.push_back(pixel);
        }
    }

    pub fn tick<T>(&mut self, pixel_x: u8, bus: &mut Bus, _dbg: &mut T) -> Option<GBColor>
    where T: Debugger {
        let lcdc = bus.read(LCDC);
        let ly = bus.read(LY);
        let window_enabled = lcdc & 0b100000 != 0;
        // let next_sprite_x = if let Some(s) = self.line_sprites.front() {
        //     s.x
        // } else {
        //     255 // pixel_x should never go that high
        // };

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
        // Commented for now, I will try to have a working background/window rendering first.
        // if next_sprite_x <= pixel_x + 8 && !self.fetching_obj && !self.wait_for_obj {
        //     self.wait_for_obj = true;
        //     self.ready = false;
        // }
        //
        // if self.wait_for_obj && self.state == FetchState::TILE {
        //     self.wait_for_obj = false;
        //     self.fetching_obj = true;
        // }

        if self.dot % 2 == 0 {
            match self.state {
                FetchState::TILE => {
                    self.fetching_id = self.get_tile_id(bus);
                    self.state = FetchState::LOW;
                    debug!("Fetched Tile ID: {}", self.fetching_id);
                },

                FetchState::LOW => {
                    self.get_tile_data(bus);
                    self.state = FetchState::HIGH;
                    debug!("Fetched Tile Low: {:#04X}", self.fetching_low);
                },

                FetchState::HIGH => {
                    // No-op as high byte is already fetched in get_data()
                    self.state = FetchState::PUSH;
                    debug!("Fetched Tile Low: {:#04X}", self.fetching_high);
                },

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
                        // TODO
                        self.state = FetchState::TILE;
                        self.ready = true;
                        debug!("Pushed OBJ Tile to FIFO");
                    } else {
                        debug!("Fetcher Waiting")
                    }
                },
            }
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