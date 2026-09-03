use crate::emulator::memory::regdefines::*;
use crate::emulator::memory::Bus;
use crate::emulator::ppu::pixels::GBColor::{BLACK, DGREY, LGREY, OFF, WHITE};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Palette {
    BGP = (BGP as usize) as isize,
    OBP0 = (OBP0 as usize) as isize,
    OBP1 = (OBP1 as usize) as isize
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Priority {
    BG,
    OBJ
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GBColor {
    WHITE = 0b00,
    LGREY = 0b01,
    DGREY = 0b10,
    BLACK = 0b11,
    OFF = 0b100
}

impl From<u8> for GBColor {
    fn from(value: u8) -> Self {
        match value {
            0b00 => WHITE, 0b01 => LGREY, 0b10 => DGREY, 0b11 => BLACK, _ => OFF
        }
    }
}

impl GBColor {
    pub fn from_pixel(pixel: PixelInfo, bus: &Bus) -> Self {
        let palette = bus.read(pixel.palette as u16);
        let lcdc = bus.read(LCDC);
        if pixel.palette == Palette::BGP && (lcdc & 1) == 0 {
            return OFF
        } else if pixel.palette != Palette::BGP && (lcdc & 2) == 0 {
            return OFF
        }

        match pixel.color_index {
            0 => (palette & 0b00000011).into(),
            1 => ((palette & 0b00001100) >> 2).into(),
            2 => ((palette & 0b00110000) >> 4).into(),
            3 => ((palette & 0b11000000) >> 6).into(),
            _ => {
                log::error!("Invalid pixel color index found ({})! Defaulting to WHITE.", pixel.color_index);
                WHITE
            },
        }
    }
}

#[derive(Debug)]
pub struct PixelInfo {
    pub color_index: u8,
    pub palette: Palette,
    pub priority: Priority,
}

impl PixelInfo {
    pub fn default() -> Self {
        PixelInfo {
            color_index: 0,
            palette: Palette::OBP0,
            priority: Priority::OBJ
        }
    }

    pub fn from_bytes(low: u8, high: u8, index: u8, is_obj: bool, palette: Palette) -> Self {
        let color_l = low & (0b10000000 >> index) != 0;
        let color_h = high & (0b10000000 >> index) != 0;
        let color = ((color_h as u8) << 1) | color_l as u8;
        PixelInfo {
            color_index: color,
            palette,
            priority: if is_obj {Priority::OBJ} else {Priority::BG}
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Sprite {
    pub x: u8,
    pub y: u8,
    pub tile_id: u8,
    pub flags: u8,
}

impl Sprite {
    pub fn new(bus: &Bus, index: u8) -> Self {
        let addr = 0xFE00 + (index as u16 * 4);
        let mut data_iter = bus.ppu_iter_at(addr);
        Sprite {
            y: data_iter.next().unwrap(),
            x: data_iter.next().unwrap(),
            tile_id: data_iter.next().unwrap(),
            flags: data_iter.next().unwrap()
        }
    }

    pub fn prio(&self) -> bool {
        (self.flags & 0b10000000) != 0
    }

    pub fn y_flip(&self) -> bool {
        (self.flags & 0b01000000) != 0
    }

    pub fn x_flip(&self) -> bool {
        (self.flags & 0b00100000) != 0
    }

    pub fn palette(&self) -> Palette {
        if self.flags & 0b00010000 == 0 {
            Palette::OBP0
        } else {
            Palette::OBP1
        }
    }
}