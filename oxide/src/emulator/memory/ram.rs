#[allow(unused_imports)]
use log::{debug, info, warn};

pub struct Ram {
    vram: Vec<[u8; 0x2000]>,
    wram: [u8; 0x1000],
    wram_banks: Vec<[u8; 0x1000]>,
    hram: [u8; 0x80],
    oam: [u8; 0x10],
    cur_wram: usize,
    cur_vram: usize
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            vram: vec![[0; 0x2000]; 1],
            wram: [0; 0x1000],
            wram_banks: vec![[0; 0x1000]; 1],
            hram: [0; 0x80],
            oam: [0; 0x10],
            cur_wram: 0,
            cur_vram: 0,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9FFF => self.vram[self.cur_vram][(addr - 0x8000) as usize],
            0xC000..=0xCFFF => self.wram[(addr - 0xC000) as usize],
            0xD000..=0xDFFF => self.wram_banks[self.cur_wram][(addr - 0xD000) as usize],
            0xE000..=0xEFFF => {
                warn!("Strange memory read to Echo RAM: {:#04X}", addr);
                self.wram[(addr - 0xE000) as usize]
            },
            0xF000..=0xFDFF => {
                warn!("Strange memory read to Echo RAM: {:#04X}", addr);
                self.wram_banks[(self.cur_wram) as usize][(addr - 0xF000) as usize]
            },
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize],
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            _ => panic!("Invalid ram access adress, this should not be possible.")
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x8000..=0x9FFF => self.vram[self.cur_vram][(addr - 0x8000) as usize] = value,
            0xC000..=0xCFFF => self.wram[(addr - 0xC000) as usize] = value,
            0xD000..=0xDFFF => self.wram_banks[self.cur_wram][(addr - 0xD000) as usize] = value,
            0xE000..=0xEFFF => {
                warn!("Strange memory write to Echo RAM: {:#02X} => {:#04X}", value, addr);
                self.wram[(addr - 0xE000) as usize] = value;
            },
            0xF000..=0xFDFF => {
                warn!("Strange memory write to Echo RAM: {:#02X} => {:#04X}", value, addr);
                self.wram_banks[self.cur_wram][(addr - 0xF000) as usize] = value;
            },
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize] = value,
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = value,
            _ => panic!("Invalid ram access adress, this should not be possible.")
        }
    }
}
