pub mod mbc1;
mod no_mbc;

use std::fs;
use std::path::Path;
use mbc1::*;
use crate::emulator::memory::cartridge::no_mbc::NoMbc;
use crate::emulator::memory::mbc_type::MbcKind;

pub trait Mbc {
    fn read(&self, rom: &[u8], ram: &[u8], addr: u16) -> u8;
    fn write(&mut self, ram: &mut [u8], addr: u16, value: u8) -> ();
}

pub struct Cartridge<M: Mbc> {
    mbc: M,
    rom: Vec<u8>,
    ram: Vec<u8>,
}

pub enum AnyCartridge {
    NO_MBC(Cartridge<NoMbc>),
    MBC1(Cartridge<Mbc1>),
}

impl<M: Mbc> Cartridge<M> {
    fn read(&self, addr: u16) -> u8 {
        self.mbc.read(&self.rom, &self.ram, addr)
    }

    fn write(&mut self, addr: u16, value: u8) -> () {
        self.mbc.write(&mut self.ram, addr, value);
    }
}

impl AnyCartridge {
    pub fn read(&self, addr: u16) -> u8 {
        match self {
            AnyCartridge::NO_MBC(cart) => cart.read(addr),
            AnyCartridge::MBC1(cart) => cart.read(addr),
        }
    }
    
    pub fn write(&mut self, addr: u16, value: u8) -> () {
        match self {
            AnyCartridge::NO_MBC(cart) => cart.write(addr, value),
            AnyCartridge::MBC1(cart) => cart.write(addr, value),
        }
    }
    
    pub fn load_from_file<P: AsRef<Path>>(rom_path: P) -> Result<Self, String> {
        let rom = fs::read(rom_path).map_err(|e| e.to_string())?;
        let mbc_val = rom[0x0147];
        let ram : Vec<u8> = match rom[0x0149] {
            0x00 => vec![0; 0],
            0x01 => vec![0; 2 * 1024],
            0x02 => vec![0; 8 * 1024],
            0x03 => vec![0; 32 * 1024],
            0x04 => vec![0; 128 * 1024],
            0x05 => vec![0; 64 * 1024],
            _ => return Err(format!("Invalid Ram size value: {}", rom[0x0149])),
        };
        
        match mbc_val {
            0x00 | 0x08 | 0x09 => {
                let mbc = NoMbc{};
                Ok(AnyCartridge::NO_MBC(Cartridge { mbc, rom, ram }))
            },
            0x01 | 0x02 | 0x03 => {
                let mbc = Mbc1::new(rom.len() / 0x4000, ram.len() / 0x2000);
                Ok(AnyCartridge::MBC1(Cartridge {mbc, rom, ram}))
            },
            _ => Err(format!("Unimplemented MBC Type: {mbc_val}"))
        }
    }
}
