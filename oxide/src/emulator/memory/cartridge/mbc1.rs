use super::*;

pub struct Mbc1 {
    ram_enable: bool,
    rom_bank: u8,
    ram_bank: u8,
    bank_mode: bool,
    
    rom_count: usize,
    ram_count: usize,
}

impl Mbc for Mbc1 {
    fn read(&self, rom: &[u8], ram: &[u8], addr: u16) -> u8 {
        match (addr, self.bank_mode, self.ram_enable) {
            (0x0000..0x4000, false, _) => rom[addr as usize],
            (0x0000..0x4000, true, _) => {
                let bank = self.ram_bank << 5;
                rom[bank as usize * 0x4000 + addr as usize]
            }
            (0x4000..0x8000, _, _) => {
                let mut bank = self.ram_bank & 0b11111;
                if bank == 0 {bank = 1};
                rom[bank as usize * 0x4000 + (addr - 0x4000) as usize]
            }
            (0xA000..0xC000, _, false) => {
                0xFF
            },
            (0xA000..0xC000, false, true) => {
                ram[addr as usize - 0xA000]
            },
            (0xA000..0xC000, true, true) => {
                if !self.ram_enable { return 0xFF }
                let bank = self.ram_bank & 0b11;
                ram[(addr as usize - 0xA000) + 0x2000 * bank as usize]
            }
            (_, _, _) => panic!("Should be unreachable. Addr: {addr:#06X}"),
        }
    }

    fn write(&mut self, ram: &mut [u8], addr: u16, mut value: u8) -> () {
        match addr {
            0x0000..0x2000 => self.ram_enable =  value & 0xF == 0xA,
            0x2000..0x4000 => {
                value = value & 0b11111;
                if value == 0 {value = 1};
                let mask = (self.rom_count.next_power_of_two() - 1) as u8;
                value &= mask;
                self.rom_bank = value;
            },
            0x4000..0x6000 => {
                self.ram_count = value as usize;
            },
            0x6000..0x8000 => self.bank_mode = (value & 1) != 0,
            0xA000..0xC000 => {
                if self.ram_enable {
                    let bank = self.ram_bank & 0b11;
                    ram[(addr as usize - 0xA000) + 0x2000 * bank as usize] = value;
                }
            }
            _ => ()
        }
    }
    
}

impl Mbc1 {
    pub fn new(rom_count: usize, ram_count: usize) -> Self {
        Self {
            ram_enable: false,
            rom_bank: 1,
            ram_bank: 0,
            bank_mode: false,
            
            rom_count,
            ram_count
        }
    }
}