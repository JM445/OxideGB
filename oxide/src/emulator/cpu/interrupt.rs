use std::ops::{BitAnd, BitOr, Not};
use super::*;
use crate::emulator::memory::RegDefines::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Interrupt {
    None   = 0,
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

impl Not for Interrupt {
    type Output = u8;

    fn not(self) -> Self::Output {
        !(self as u8)
    }
}

impl Cpu {

    // Check if an interrupt is ready
    pub (super) fn check_interrupt(&mut self, bus: &Bus) -> bool {
        self.ime && ((bus.read(IF) & 0x1F) & (bus.read(IE) & 0x1F)) != 0
    }

    pub (super) fn decode_interrupt(&self, interrupt: Interrupt) -> VecDeque<MicroOp> {
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
                source: RWTarget::Value(Self::get_interrupt_address(interrupt)),
                dest: RWTarget::Reg16(Reg16::PC), prefetch: true
            }
        ])
    }
    
    pub (super) fn get_interrupt_address(int: Interrupt) -> u16 {
        match int {
            Interrupt::VBlank => 0x0040,
            Interrupt::LCD    => 0x0048,
            Interrupt::Timer  => 0x0050,
            Interrupt::Serial => 0x0058,
            Interrupt::Joypad => 0x0060,
            Interrupt::None   => 0x0000,
        }
    }
}


impl Bus {
    pub fn set_interrupt(&mut self, int: Interrupt) {
        let if_reg = self.read(IF);
        self.write(IF, if_reg | int);
    }

    pub fn unset_interrupt(&mut self, int: Interrupt) {
        let if_reg = self.read(IF);
        self.write(IF, if_reg & !int);
    }

    pub fn get_first_interrupt(&self) -> Interrupt {
        let flags = self.read(IF);
        let enable = self.read(IE);
        if (flags & Interrupt::VBlank) & (enable & Interrupt::VBlank) != 0 {
            Interrupt::VBlank
        } else if (flags & Interrupt::LCD) & (enable & Interrupt::LCD) != 0 {
            Interrupt::LCD
        } else if (flags & Interrupt::Timer) & (enable & Interrupt::Timer) != 0 {
            Interrupt::Timer
        } else if (flags & Interrupt::Serial) & (enable & Interrupt::Serial) != 0 {
            Interrupt::Serial
        } else if (flags & Interrupt::Joypad) & (enable & Interrupt::Joypad) != 0 {
            Interrupt::Joypad
        } else {
            Interrupt::None
        }
    }


}