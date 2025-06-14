use super::*;
use crate::{emu_print, GLOB_SETTINGS};

impl Bus {
    pub fn tick_serial(&mut self) {
        match self.ioregs[0x02] & 0x83 {
            0x81 => {
                if GLOB_SETTINGS.get().unwrap().print_serial {
                    let c = self.ioregs[0x01] as char;
                    emu_print!("{c}");
                    self.ioregs[0x01] = 0xFF;
                    self.ioregs[0x02] &= 0x7F;
                }
            },
            _ => ()
        }
    }
}