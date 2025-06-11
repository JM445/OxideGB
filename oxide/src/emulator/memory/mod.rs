pub mod cartridge;
pub mod mbc_type;
pub mod ram;

use cartridge::*;
use ram::*;

#[allow(unused_imports)]
use log::{debug, info, warn};

use std::path::Path;

pub struct Bus {
    pub cartridge: Cartridge,
    pub ram: Ram,
    pub ioregs: Vec<u8>,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum MemBlock {
    ROM0 = 0,
    ROMX = 1,
    VRAM = 2,
    ERAM = 3,
    WRAM1 = 4,
    WRAM2 = 5,
    ECHO = 6,
    OAM = 7,
    NC = 8,
    IOREG = 9,
    HRAM = 10,
    IE = 11,
}

impl MemBlock {
    pub fn from_addr(addr: u16) -> Self {
        use super::MemBlock::*;
        match addr {
            0x0000..0x4000 => ROM0,
            0x4000..0x8000 => ROMX,
            0x8000..0xA000 => VRAM,
            0xA000..0xC000 => ERAM,
            0xC000..0xD000 => WRAM1,
            0xD000..0xE000 => WRAM2,
            0xE000..0xFE00 => ECHO,
            0xFE00..0xFEA0 => OAM,
            0xFEA0..0xFF00 => NC,
            0xFF00..0xFF80 => IOREG,
            0xFF80..0xFFFF => HRAM,
            0xFFFF => IE
        }
    }
}

pub struct BusIter<'a> {
    bus: &'a Bus,
    iter_ptr: u16
}

impl<'a> Iterator for BusIter<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.bus.read(self.iter_ptr);
        self.iter_ptr += 1;
        Some(value)
    }
}

impl Bus {
    pub fn new<P: AsRef<Path>>(rom_path: P) -> Result<Self, String> {
        Ok(Bus {
            cartridge: Cartridge::from_file(rom_path)?,
            ram: Ram::new(),
            ioregs: vec![0, 0x80],
        })
    }

    pub fn read(&self, addr: u16) -> u8 {
        #[cfg(feature = "log_mem_access")]
        debug!("Memory read: 0x{:#06X}", addr);

        match addr {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.cartridge.read(addr),
            0x8000..=0x9FFF | 0xC000..=0xEFFF | 0xF000..=0xFE9F | 0xFF80..=0xFFFE => self.ram.read(addr),
            0xFEA0..=0xFEFF => {
                warn!("Memory read to prohibited zone: {:#06X}", addr);
                0xFF
            },
            0xFF00..=0xFF7F => self.read_regs(addr),
            _ => 0xFF
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        #[cfg(feature = "log_mem_access")]
        debug!("Memory write: 0x{:#04X} => 0x{:#06X}", value, addr);

        match addr {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.cartridge.write(addr, value),
            0x8000..=0x9FFF | 0xC000..=0xEFFF | 0xF000..=0xFE9F | 0xFF80..=0xFFFE => self.ram.write(addr, value),
            0xFEA0..=0xFEFF => warn!("Memory write to prohibited zone: {:#06X}", addr),
            0xFF00..=0xFF7F => self.write_regs(addr, value),
            _ => ()
        }
    }

    pub fn get_instruction(&self, addr: u16) -> [u8; 4] {
        let mut res : [u8; 4] = [0, 0, 0, 0];

        res[0] = self.read(addr);
        res[1] = self.read(addr.wrapping_add(1));
        res[2] = self.read(addr.wrapping_add(2));
        res[3] = self.read(addr.wrapping_add(3));
        res
    }

    #[allow(unused_variables, dead_code)]
    fn read_regs(&self, addr: u16) -> u8 {
        0x00
    }

    #[allow(unused_variables, dead_code)]
    fn write_regs(&mut self, addr: u16, value: u8) {
        ()
    }

    pub fn iter_at(&self, addr: u16) -> BusIter<'_> {
        BusIter{
            bus: &self,
            iter_ptr: addr
        }
    }

    pub fn iter(&self) -> BusIter<'_> {
        BusIter {
            bus: &self,
            iter_ptr: 0x0000
        }
    }

    pub fn get_rom_bank(&self) -> usize {
        self.cartridge.cur_rom
    }

    pub fn get_ram_bank(&self) -> usize {
        self.cartridge.cur_ram
    }

    pub fn is_ram(addr: u16) -> bool {
        match addr {
            0x8000..=0xFE9F | 0xFF80..=0xFFFE => true,
            _ => false
        }
    }

    pub fn is_same_block(addr1: u16, addr2: u16) -> bool {
        let blocks : [u16; 12] = [
            0x0000, 0x4000, 0x8000, 0xA000, 0xC000, 0xD000, 0xE000, 0xFE00, 0xFEA0, 0xFF00, 0xFF80, 0xFFFF
        ];
        let mut block1 = -1;
        let mut block2 = -1;
        for i in 0..11 {
            if addr1 >= blocks[i] && addr1 < blocks[i + 1] {
                block1 = i as i32;
            }

            if addr2 >= blocks[i] && addr2 < blocks[i + 1] {
                block2 = i as i32;
            }
        }

        block1 == block2
    }
}
