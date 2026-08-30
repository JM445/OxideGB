use image::buffer::PixelsMut;
use crate::emulator::memory::regdefines::*;
use crate::emulator::memory::Bus;
use crate::emulator::ppu::Mode;
use crate::settings::GLOB_SETTINGS;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Palette {
    BGP,
    OBP0,
    OBP1
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Priority {
    BG,
    OBJ
}

#[derive(Debug)]
pub struct PixelInfo {
    color_index: u8,
    palette: Palette,
    priority: Priority,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Hash)]
enum BgFetchState {
    TILE = 0,
    HIGH = 1,
    LOW = 2,
    PUSH = 3,
}

#[derive(Debug)]
pub struct PixelFetcher {
    // BG
    bg_state: BgFetchState,
    bg_phase: bool, // Alternate at each tick, actions only when false (2 ticks per action)
    bg_tile_id: u8,
    bg_tile_low: u8,
    bg_tile_high: u8,
    bg_fetch_x: u8,
    bg_window: bool,

    // OAM
    current_sprite: Option<Sprite>,
    oam_state: OamFetchState,
    oam_tile_hi: u8,
    oam_tile_low: u8,
    oam_phase: bool,
}