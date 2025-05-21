use super::mbc_type::*;

#[allow(unused_imports)]
use log::{debug, info, warn};

use std::path::Path;
use std::fs;

#[derive(Debug)]
pub struct Cartridge {
    rom_banks: Vec<[u8; 0x4000]>, // Rom Banks
    ram_banks: Vec<[u8; 0x2000]>, // Ram Banks
    mbc: MbcKind,

    cur_rom: usize, // Current Rom Bank index
    cur_ram: usize, // Current Ram Bank index
    ram_enabled: bool,
}

impl Cartridge {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let raw = fs::read(path).map_err(|e| e.to_string())?;
        let kind = MbcKind::from_u8(raw[0x0147]);

        match kind {
            Some(MbcKind::NO_MBC(_)) => Self::load_no_mbc(raw, kind.unwrap()),
            None => Err(format!("Unknown cartridge type value: {}", raw[0x0147])),
            _ => Err(format!("MBC Kind {} not yet implemented !", kind.unwrap()))
        }
    }

    fn load_no_mbc(raw: Vec<u8>, kind: MbcKind) -> Result<Self, String> {
        let ram_banks = match kind {
            MbcKind::NO_MBC(value) if value == 0x00 => Ok(vec![[0; 0x2000]; 0]),
            MbcKind::NO_MBC(value) if value == 0x08 => Ok(vec![[0; 0x2000]; 1]),
            MbcKind::NO_MBC(value) if value == 0x09 => Ok(vec![[0; 0x2000]; 1]),
            MbcKind::NO_MBC(value) => Err(format!("MBC Type error. {:#02X} is not a valid NO_MBC value.", value)),
            _ => Err("Should be unreachable".to_string())
        }?;

        let rom_banks: Vec<[u8; 0x4000]> = raw.chunks(0x4000)
                                              .map(|x| {
                                                  let chunk: &[u8] = x;
                                                  <[u8; 0x4000]>::try_from(chunk).expect("Incomplete ROM: chunk size is less than 0x4000")
                                              }).collect();

        let enabled = if ram_banks.len() == 1 { true } else {false};

        Ok(Self {
            rom_banks,
            ram_banks,
            mbc: kind,

            cur_rom: 1,
            cur_ram: 0,
            ram_enabled: enabled
        })
    }

    pub fn read(&self, addr: u16) -> u8 {
        match (addr, self.mbc) {
            (0x0000..=0x3FFF, _) => self.rom_banks[0][addr as usize],
            (0x4000..=0x7FFF, MbcKind::NO_MBC(_)) => self.rom_banks[1][addr as usize],
            (0xA000..=0xBFFF, MbcKind::NO_MBC(value)) if value == 0x08 || value == 0x09 => self.ram_banks[0][(addr - 0xA000) as usize],
            _ => panic!("Unexpected memory access: Address = {:#04X}, MBC = {}", addr, self.mbc)
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match (addr, self.mbc) {
            (0x0000..=0x7FFF, MbcKind::NO_MBC(_)) => warn!("Strange memory write to NO_MBC ROM: {:#02X} => {:#04X}", value, addr),
            (0xA000..=0xBFFF, MbcKind::NO_MBC(value)) if value == 0x08 || value == 0x09 => self.ram_banks[0][(addr - 0xA000) as usize] = value,
            _ => panic!("Unexpected memory write: Address = {:#04X}, MBC = {}", addr, self.mbc)
        };
    }
}
