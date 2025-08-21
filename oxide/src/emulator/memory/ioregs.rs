use crate::emulator::memory::regdefines::*;
use crate::emulator::memory::Bus;
use crate::emulator::ppu::Mode;
use crate::settings::GLOB_SETTINGS;
use log::debug;

impl Bus {
    #[allow(unused_variables, dead_code)]
    pub(super) fn read_regs(&self, addr: u16) -> u8 {
        match addr {
            JOYP => self.read_joyp(),
            LY | SC => {
                if GLOB_SETTINGS.get().unwrap().doctor_logs {0x90} else {0xFF}
            }, // Temporary values to run Mooneye and GB Doctor
            STAT => {
                let mut val: u8 = self.ioregs[0x41];
                if self.ioregs[0x40] & 0b10000000 == 0 {
                    val = val & 0b11111100;
                }
                val
            }
            0xFF00..0xFF80 => self.ioregs[addr as usize - 0xFF00],
            IE => self.ioregs[0x7F],
            _ => 0x00
        }
    }

    #[allow(unused_variables, dead_code)]
    pub(super) fn write_regs(&mut self, addr: u16, value: u8) {
        match addr {
            JOYP => self.write_joyp(value),
            BANK => self.boot_enabled = false,
            DIV => {
                debug!("DIV Register written. Resetting counter to 0.");
                self.div_written = true;
                self.ioregs[0x04] = 0x00;
            },
            SC => {
                debug!("SC Written. Value: {value}.");
                self.ioregs[addr as usize - 0xFF00] = value;
            }
            STAT => self.ioregs[0x41] = (value & 0b11111100) | (self.ioregs[0x41] & 0b11), 
            0xFF00..0xFF80 => self.ioregs[addr as usize - 0xFF00] = value,
            IE => self.ioregs[0x7F] = value,
            _ => ()
        }
    }

    pub fn get_ppu_mode(&self) -> Mode {
        match self.read_regs(STAT) & 0b11 {
            0 => Mode::Mode0,
            1 => Mode::Mode1,
            2 => Mode::Mode2,
            3 => Mode::Mode3,
            _ => panic!("Unreachable")
        }
    }
    
    fn read_joyp(&self) -> u8 {
        let joystate = self.io_manager.get_joystate();
        let sel = self.read(JOYP) & 0x30;             // Get Register selection bits
        let buttons = (sel & 0b0010_0000) == 0;     // Is buttons selected
        let dpad = (sel & 0b0001_0000) == 0;        // Is DPad selected
        let mut result: u8 = 0;

        if buttons {result |=  joystate       & 0xF}
        if dpad    {result |= (joystate >> 4) & 0xF}

        ((!result) & 0xF) | sel | 0xC0                    // Recompute the register
    }
    
    fn write_joyp(&mut self, value: u8) {
        let old = self.ioregs[0x00] & 0b11001111;
        self.ioregs[0x00] = old | (value & 0b00110000);
    }
}