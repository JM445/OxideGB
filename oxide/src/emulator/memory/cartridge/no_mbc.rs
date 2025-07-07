use super::*;

pub struct NoMbc {}

impl Mbc for NoMbc {
    fn read(&self, rom: &[u8], ram: &[u8], addr: u16) -> u8 {
        match addr {
            0x0000..0x8000 => rom[addr as usize],
            0xA000..0xC000 => ram[addr as usize - 0xA000],
            _ => panic!("Should be unreachable")
        }
    }

    fn write(&mut self, ram: &mut [u8], addr: u16, value: u8) -> () {
        match addr {
            0xA000..0xC000 => ram[addr as usize - 0xA000] = value,
            _ => ()
        }
    }

    fn is_writeable(&self, _addr: u16) -> bool {
        true
    }
}