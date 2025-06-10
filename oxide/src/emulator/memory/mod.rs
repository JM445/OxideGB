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
}
