mod cartridge;

use cartridge::*;
use log::{debug, info, warn};

pub struct Bus {
    cartridge: Cartridge
}

impl Bus {
    pub fn read(&self, addr: u16) -> u8 {
        #[cfg(feature = "log_mem_access")]
        debug!("Memory read: 0x{:#06X}", addr);

        match addr {
            0x0000..0x7FFF | 0xA000..0xBFFF => self.cartridge.read(addr),
            0xE000..0xFDFF => warn!("Strange memory read to Echo RAM: 0x{:#06X}", addr),
            _ => 0xFF
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        #[cfg(feature = "log_mem_access")]
        debug!("Memory write: 0x{:#02X} => 0x{:#06X}", value, addr);

        match addr {
            0x0000..0x7FFF | 0xA000..0xBFFF => self.cartridge.write(addr),
            0xE000..0xFDFF => warn!("Strange memory write to Echo RAM: 0x{:#02X} => 0x{:#06X}", value, addr),
            _ => ()
        }
    }
}
