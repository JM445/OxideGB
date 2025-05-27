pub mod cartridge;
pub mod mbc_type;
pub mod ram;

use cartridge::*;
use ram::*;

#[allow(unused_imports)]
use log::{debug, info, warn};

use std::path::Path;

pub struct Bus {
    cartridge: Cartridge,
    ram: Ram,
    ioregs: Vec<u8>,
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
        res[1] = self.read(addr + 1);
        res[2] = self.read(addr + 2);
        res[3] = self.read(addr + 3);
        return res;
    }

    #[allow(unused_variables, dead_code)]
    fn read_regs(&self, addr: u16) -> u8 {
        0x00
    }

    #[allow(unused_variables, dead_code)]
    fn write_regs(&mut self, addr: u16, value: u8) {
        ()
    }
}
