use super::Cpu;

trait RegAccess<R> {
    type Value;

    fn read(&self, reg: R) -> Self::Value;
    fn write(&mut self, reg: R, value: Self::Value);
}

#[derive(Copy, Clone, Debug)]
pub enum Reg8 {
    A = 0,
    F,
    B,
    C,
    D,
    E,
    H,
    L
}

#[derive(Copy, Clone, Debug)]
pub enum Reg16 {
    BC = 0,
    DE,
    HL,
    SP,
    PC,
    AF,
}

impl Cpu {
    pub fn read8(&self, r: Reg8) -> u8 {
        match r {
            Reg8::A => self.a,
            Reg8::F => self.f,
            Reg8::B => self.b,
            Reg8::C => self.c,
            Reg8::D => self.d,
            Reg8::E => self.e,
            Reg8::H => self.h,
            Reg8::L => self.l
        }
    }

    pub fn read16(&self, r: Reg16) -> u16 {
        match r {
            Reg16::AF => ((self.a as u16) << 8) | (self.f as u16),
            Reg16::BC => ((self.b as u16) << 8) | (self.b as u16),
            Reg16::DE => ((self.d as u16) << 8) | (self.e as u16),
            Reg16::HL => ((self.h as u16) << 8) | (self.l as u16),
            Reg16::SP => self.sp,
            Reg16::PC => self.pc
        }
    }

    pub fn write8(&mut self, r: Reg8, value: u8) {
        match r {
            Reg8::A => self.a = value,
            Reg8::F => self.f = value,
            Reg8::B => self.b = value,
            Reg8::C => self.c = value,
            Reg8::D => self.d = value,
            Reg8::E => self.e = value,
            Reg8::H => self.h = value,
            Reg8::L => self.l = value
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
            Reg16::PC => self.sp = value,
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
