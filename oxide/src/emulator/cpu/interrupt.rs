use std::ops::{BitAnd, BitOr};
use super::*;
use crate::emulator::memory::*;


const IE: u16 = 0xFFFF;
const IF: u16 = 0xFF0F;
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Interrupt {
    VBlank = 0b1,
    LCD    = 0b10,
    Timer  = 0b100,
    Serial = 0b1000,
    Joypad = 0b10000
}

impl BitAnd<Interrupt> for u8 {
    type Output = u8;

    fn bitand(self, rhs: Interrupt) -> Self::Output {
        self & (rhs as u8)
    }
}

impl BitOr<Interrupt> for u8 {
    type Output = u8;

    fn bitor(self, rhs: Interrupt) -> Self::Output {
        self | (rhs as u8)
    }
}

impl Cpu {

    // Check if an interrupt is ready
    pub (super) fn check_interrupt(&mut self, bus: &Bus) -> bool {
        self.ime && ((bus.read(IF) & 0x1F) & (bus.read(IE) & 0x1F)) != 0
    }

    pub (super) fn decode_interrupt(&self, bus: &Bus) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Operation { ope: Operation::Dec {
                source: RWTarget::Reg16(Reg16::PC), dest: RWTarget::Reg16(Reg16::PC), mask: 0
            }, prefetch: false},
            MicroOp::Operation { ope: Operation::Dec {
                source: RWTarget::Reg16(Reg16::SP), dest: RWTarget::Reg16(Reg16::SP), mask: 0
            }, prefetch: false},
            MicroOp::DataMove {
                source: RWTarget::Reg8(Reg8::PCH), dest: RWTarget::Indirect16D(Reg16::SP), prefetch: false,
            },
            MicroOp::DataMove {
                source: RWTarget::Reg8(Reg8::PCL), dest: RWTarget::Indirect16(Reg16::SP), prefetch: false,
            },
            MicroOp::DataMove {
                source: RWTarget::Value(Self::get_interrupt_address(bus.read(IF), bus.read(IE))),
                dest: RWTarget::Reg16(Reg16::PC), prefetch: true
            }
        ])
    }

    pub (super) fn get_interrupt_address(flags: u8, enable: u8) -> u16 {
        if ((flags & Interrupt::VBlank) & (enable & Interrupt::VBlank)) != 0 { // VBlank
            debug!("VBLANK interrupt triggered !");
            0x0040
        } else if ((flags & Interrupt::LCD) & (enable & Interrupt::LCD)) != 0 { // LCD
            debug!("LCD interrupt triggered !");
            0x0048
        } else if ((flags & Interrupt::LCD) & (enable & Interrupt::LCD)) != 0 { // Timer
            debug!("TIMER interrupt triggered !");
            0x0050
        } else if ((flags & Interrupt::Serial) & (enable & Interrupt::Serial)) != 0 { // Serial
            debug!("SERIAL interrupt triggered !");
            0x0058
        } else if ((flags & Interrupt::Joypad) & (enable & Interrupt::Joypad)) != 0 { // Joypad
            debug!("JOYPAD interrupt triggered !");
            0x0060
        } else {
            panic!("Should be unreachable")
        }
    }
}


impl Bus {
    pub fn set_interrupt(&mut self, int: Interrupt) {
        let if_reg = self.read(IF);
        self.write(IF, if_reg | int);
    }
}