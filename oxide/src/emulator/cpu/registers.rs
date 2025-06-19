#[cfg(test)]
#[path = "tests/registers.rs"]
mod registers_tests;

use super::Cpu;

trait RegAccess<R> {
    type Value;

    fn read(&self, reg: R) -> Self::Value;
    fn write(&mut self, reg: R, value: Self::Value);
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Reg8 {
    A = 0,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    W,
    Z,
    PCH,
    PCL,
    SPH,
    SPL
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Reg16 {
    BC = 0,
    DE,
    HL,
    SP,
    PC,
    AF,
    WZ
}

impl Reg16 {
    pub fn lsb(&self) -> Reg8 {
        match self {
            Self::BC => Reg8::C,
            Self::DE => Reg8::E,
            Self::HL => Reg8::L,
            Self::SP => Reg8::SPL,
            Self::PC => Reg8::PCL,
            Self::AF => Reg8::F,
            Self::WZ => Reg8::Z,
        }
    }

    pub fn msb(&self) -> Reg8 {
        match self {
            Self::BC => Reg8::B,
            Self::DE => Reg8::D,
            Self::HL => Reg8::H,
            Self::SP => Reg8::SPH,
            Self::PC => Reg8::PCH,
            Self::AF => Reg8::A,
            Self::WZ => Reg8::W,
        }
    }

}

#[derive(Copy, Clone, Debug)]
pub enum Flag {
    Z = 7,
    N = 6,
    H = 5,
    C = 4
}

impl Cpu {

    #[inline]
    pub fn get_flag(&self, f: Flag) -> u8 {
        ((1 << (f as u8)) & self.f) >> (f as u8)
    }

    #[inline]
    pub fn set_flag(&mut self, f: Flag, value: u8) {
        let v = value & 0b00000001; // This may cause bugs if value is not 0 or 1
        self.f = (self.f & !(1 << (f as u8))) | (v << (f as u8));
    }

    pub fn read8(&self, r: Reg8) -> u8 {
        match r {
            Reg8::A => self.a,
            Reg8::F => self.f & 0xF0,
            Reg8::B => self.b,
            Reg8::C => self.c,
            Reg8::D => self.d,
            Reg8::E => self.e,
            Reg8::H => self.h,
            Reg8::L => self.l,
            Reg8::PCH => (self.pc >> 8) as u8,
            Reg8::PCL => self.pc as u8,
            Reg8::SPH => (self.sp >> 8) as u8,
            Reg8::SPL => self.sp as u8,
            Reg8::W => self.w,
            Reg8::Z => self.z
        }
    }

    pub fn read16(&self, r: Reg16) -> u16 {
        match r {
            Reg16::AF => ((self.a as u16) << 8) | ((self.f & 0xF0) as u16),
            Reg16::BC => ((self.b as u16) << 8) | (self.c as u16),
            Reg16::DE => ((self.d as u16) << 8) | (self.e as u16),
            Reg16::HL => ((self.h as u16) << 8) | (self.l as u16),
            Reg16::SP => self.sp,
            Reg16::PC => self.pc,
            Reg16::WZ => ((self.w as u16) << 8) | (self.z as u16),
        }
    }

    pub fn write8(&mut self, r: Reg8, value: u8) {
        match r {
            Reg8::A => self.a = value,
            Reg8::F => self.f = value & 0xF0,
            Reg8::B => self.b = value,
            Reg8::C => self.c = value,
            Reg8::D => self.d = value,
            Reg8::E => self.e = value,
            Reg8::H => self.h = value,
            Reg8::L => self.l = value,
            Reg8::PCH => self.pc = (self.pc & 0x00FF) | (value as u16) << 8,
            Reg8::PCL => self.pc = (value as u16) | (self.pc & 0xFF00),
            Reg8::SPH => self.sp = (self.sp & 0x00FF) | (value as u16) << 8,
            Reg8::SPL => self.sp = (value as u16) | (self.sp & 0xFF00),
            Reg8::W   => self.w = value,
            Reg8::Z   => self.z = value,
        }
    }

    pub fn write16(&mut self, r: Reg16, value: u16) {
        match r {
            Reg16::AF => {
                self.a = (value >> 8) as u8;
                self.f = (value & 0xF0) as u8;
            },
            Reg16::BC => {
                self.b = (value >> 8) as u8;
                self.c = value as u8;
            },
            Reg16::DE => {
                self.d = (value >> 8) as u8;
                self.e = value as u8;
            },
            Reg16::HL => {
                self.h = (value >> 8) as u8;
                self.l = value as u8;
            },
            Reg16::SP => self.sp = value,
            Reg16::PC => self.pc = value,
            Reg16::WZ => { 
                self.w = (value >> 8) as u8;
                self.z = value as u8;
            },
        };
    }
}

impl RegAccess<Reg8> for Cpu {
    type Value = u8;

    fn read(&self, r: Reg8) -> u8 {
        self.read8(r)
    }

    fn write(&mut self, r: Reg8, value: u8) {
        self.write8(r, value);
    }
}

impl RegAccess<Reg16> for Cpu {
    type Value = u16;

    fn read(&self, r: Reg16) -> u16 {
        self.read16(r)
    }

    fn write(&mut self, r: Reg16, value: u16) {
        self.write16(r, value);
    }
}