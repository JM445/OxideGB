#[allow(unused_imports)]
use log::{debug, info, warn, error};

use crate::emulator::memory::Bus;

use super::*;

use std::num::Wrapping;

#[derive(Debug, Copy, Clone)]
pub enum MicroOp {
    DataMove   {source: RWTarget, dest: RWTarget, prefetch: bool},
    Operation{ope: Operation, prefetch: bool},
    ReadIMM{prefetch: bool},
    ReadLSB{prefetch: bool},
    ReadMSB{prefetch: bool},
    ReadMSBCC {cc: Condition},
    CheckCC {cc: Condition},
    Cpl, Daa, Ccf, Scf, Prefix,
    RetI,
    PrefetchOnly,
}

#[derive(Debug, Copy, Clone)]
pub enum RWTarget {
    Reg8(Reg8),
    Reg16(Reg16),
    Addr,
    Indirect16(Reg16),
    Indirect16I(Reg16),
    Indirect16D(Reg16),
    Indirect8(Reg8),
    Value(u16),
    Tmp8,
    Tmp16,
    IME,
}

#[derive(Debug, Copy, Clone)]
pub enum Condition {
    Z,
    C,
    NZ,
    NC
}

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    // Arithmetic
    Add{left: RWTarget, right: RWTarget, dest: RWTarget, mask: u8},
    Sub{left: RWTarget, right: RWTarget, dest: RWTarget, mask: u8},
    Inc{source: RWTarget, dest: RWTarget, mask: u8},
    Dec{source: RWTarget, dest: RWTarget, mask: u8},
    Adc{left: RWTarget, right: RWTarget, dest: RWTarget, mask: u8},
    Sbc{left: RWTarget, right: RWTarget, dest: RWTarget, mask: u8},
    Ads{left: RWTarget, right: RWTarget, dest: RWTarget, mask: u8},

    // Logic
    And{left: RWTarget, right: RWTarget, dest: RWTarget, mask: u8},
    Or {left: RWTarget, right: RWTarget, dest: RWTarget, mask: u8},
    Xor{left: RWTarget, right: RWTarget, dest: RWTarget, mask: u8},

    // Bitshifts
    Rsh {shift: ShiftType, source: RWTarget, dest: RWTarget, mask: u8},
    Lsh {shift: ShiftType, source: RWTarget, dest: RWTarget, mask: u8},

    // Binops
    Swp {source: RWTarget, dest: RWTarget, mask: u8},
    Bit {source: RWTarget, bit: u8, mask: u8},
    Rsb {source: RWTarget, dest: RWTarget, bit: u8, value: u8},
}

#[derive(Debug, Copy, Clone)]
pub enum ShiftType {
    R,
    RC,
    SA,
    SL
}

impl Cpu {
    fn get_target(&mut self, target: RWTarget, bus: &Bus) -> u16 {
        match target {
            RWTarget::Reg8(trg) => self.read8(trg) as u16,
            RWTarget::Reg16(trg) => self.read16(trg),
            RWTarget::Addr => bus.read(self.tmp16) as u16,
            RWTarget::Indirect8(_) => panic!("Unimplemented Indirect read !"),
            RWTarget::Indirect16(trg) => bus.read(self.read16(trg)) as u16,
            RWTarget::Indirect16D(trg) => {
                let res = (bus.read(self.read16(trg)) as u16).clone();
                let hl = self.read16(trg);
                self.write16(trg, hl - 1);
                res
            }
            RWTarget::Indirect16I(trg) => {
                let res = (bus.read(self.read16(trg)) as u16).clone();
                let hl = self.read16(trg);
                self.write16(trg, hl + 1);
                res
            }
            RWTarget::Tmp8 => self.tmp8 as u16,
            RWTarget::Tmp16 => self.tmp16,
            RWTarget::IME => self.ime as u16,
            RWTarget::Value(v) => v
        }
    }

    fn set_target(&mut self, target: RWTarget, value: u16, bus: &mut Bus) {
        match target {
            RWTarget::Reg8(trg) => self.write8(trg, value as u8),
            RWTarget::Reg16(trg) => self.write16(trg, value),
            RWTarget::Addr => bus.write(self.tmp16, value as u8),
            RWTarget::Indirect8(_) => panic!("Unimplemented Indirect write !"),
            RWTarget::Indirect16(trg) => bus.write(self.read16(trg), value as u8),
            RWTarget::Indirect16D(trg) => {
                bus.write(self.read16(trg), value as u8);
                let hl = self.read16(trg);
                self.write16(trg, hl - 1);
            }
            RWTarget::Indirect16I(trg) => {
                bus.write(self.read16(trg), value as u8);
                let hl = self.read16(trg);
                self.write16(trg, hl + 1);
            }
            RWTarget::Tmp8 => {
                self.tmp8 = value as u8;
                self.tmp16 = value as u16 & 0xFF00;
            },
            RWTarget::Tmp16 => self.tmp16 = value,
            RWTarget::IME => self.ime = value > 0,
            RWTarget::Value(_) => ()
        };
    }

    fn set_flags(&mut self, value: u8, mask: u8, ) {
        if mask & 0b1000 != 0 {
            self.set_flag(Flag::Z, (value & 0b1000) >> 3);
        }
        if mask & 0b0100 != 0 {
            self.set_flag(Flag::N, (value & 0b0100) >> 3);
        }
        if mask & 0b0010 != 0 {
            self.set_flag(Flag::H, (value & 0b0010) >> 3);
        }
        if mask & 0b0001 != 0 {
            self.set_flag(Flag::C, (value & 0b0001) >> 3);
        }
    }

    fn alu_add(left: Wrapping<u16>, right: Wrapping<u16>, carry: Wrapping<u16>) -> (u16, u8) {
        let res = left + right + carry;
        let xor = (left ^ right ^ carry).0;
        (res.0,
        (((res.0 & 0xFF) == 0) as u8) << 3 | // Z
        0b0000 |                             // N
        (((xor & 0x10)  != 0) as u8) << 2 |  // H
        (((xor & 0x100) != 0) as u8) << 3)   // C
    }

    fn alu_sub(left: Wrapping<u16>, right: Wrapping<u16>, carry: Wrapping<u16>) -> (u16, u8) {
        let res = left - right - carry;
        let borrow = right + carry;

        (res.0,
        ((((res.0 & 0xFF) == 0) as u8) << 3) |              // Z
        0b0100 |                                            // N
        (((left.0 & 0xF) < (borrow.0 & 0xF)) as u8) << 2 |  // H
        (((left.0 & 0xFF) < (borrow.0 & 0xFF)) as u8)) // C
    }

    fn alu_and(left: Wrapping<u16>, right: Wrapping<u16>) -> (u16, u8) {
        let res = left & right;
        let z   = (left.0 & 0xFF) == 0;
        (res.0, (((z as u8) << 3) | 2))
    }

    fn alu_or(left: Wrapping<u16>, right: Wrapping<u16>) -> (u16, u8) {
        let res = left | right;
        let z   = (left.0 & 0xFF) == 0;
        (res.0, ((z as u8) << 3))
    }

    fn alu_xor(left: Wrapping<u16>, right: Wrapping<u16>) -> (u16, u8) {
        let res = left ^ right;
        let z   = (left.0 & 0xFF) == 0;

        (res.0, ((z as u8) << 3))
    }

    fn alu_rsh(shift: ShiftType, val: Wrapping<u8>, old_c: u8) -> (u16, u8) {
        let (res, c) = match shift {
            ShiftType::R => ((val >> 1).0 | (old_c << 7), (val.0 & 0x01)),
            ShiftType::RC => ((val >> 1).0 | (val << 7).0, val.0 & 0x01),
            ShiftType::SL => ((val >> 1).0, val.0 & 0x01),
            ShiftType::SA => ((val >> 1).0 | (val.0 & 0x80), val.0 & 0x01)
        };

        (res as u16, (((res == 0) as u8) << 3) | (c & 0x01))
    }

    fn alu_lsh(shift: ShiftType, val: Wrapping<u8>, old_c: u8) -> (u16, u8) {
        let (res, c) = match shift {
            ShiftType::R => ((val << 1).0 | (old_c & 0x01), (val.0 & 0x80) >> 7),
            ShiftType::RC => ((val << 1).0 | ((val.0 & 0x80) >> 7), (val.0 & 0x80) >> 7),
            ShiftType::SL => ((val << 1).0, (val.0 & 0x80) >> 7),
            ShiftType::SA => {
                error!("Error: Invalid Shift Left Arithmetic");
                (val.0, 0)
            }
        };

        (res as u16, (((res == 0) as u8) << 3) | (c & 0x01))
    }

    fn alu_swap(val: Wrapping<u8>) -> (u16, u8) {
        let res = ((val.0 & 0xF0) >> 4) | ((val.0 & 0x0F) << 4);
        (res as u16, ((res == 0) as u8) << 3)
    }

    fn alu_bit(val: Wrapping<u8>, bit: u8) -> (u16, u8) {
        let res = ((val.0 & (1 << bit)) != 0) as u8;
        (res as u16, (res << 3) | 0x02)
    }

    fn alu_rsb(val: Wrapping<u8>, bit: u8, set: u8) -> (u16, u8) {
        let mask = !(1 << bit);
        let res = (val.0 & mask) | (set << bit);
        (res as u16, 0)
    }

    pub(super) fn execute<T>(&mut self, op: MicroOp, bus: &mut Bus, dbg: &mut T)
    where T: Debugger
    {
        let prefetch = match &op {
            MicroOp::DataMove{prefetch, ..} |
            MicroOp::Operation{prefetch, ..} |
            MicroOp::ReadIMM{prefetch} |
            MicroOp::ReadLSB{prefetch} |
            MicroOp::ReadMSB{prefetch} => *prefetch,
            MicroOp::ReadMSBCC { .. } => false,
            MicroOp::CheckCC { .. } => false,
            MicroOp::RetI { .. } => false,
            MicroOp::PrefetchOnly | MicroOp::Cpl | MicroOp::Daa |
            MicroOp::Ccf | MicroOp::Scf | MicroOp::Prefix => true,
        };

        match op {
            MicroOp::DataMove{source, dest, ..} => self.execute_move(source, dest, bus, dbg),
            MicroOp::Operation{ope, ..} => self.execute_op(ope, bus),
            MicroOp::ReadIMM{..} => self.execute_imm(bus),
            MicroOp::ReadLSB{..}  => self.execute_read_lsb(bus),
            MicroOp::ReadMSB{..}  => self.execute_read_msb(bus),
            MicroOp::ReadMSBCC {cc} => {
                self.execute_read_msb(bus);
                self.execute_cc(cc);
            },
            MicroOp::CheckCC {cc} => self.execute_cc(cc),
            MicroOp::RetI => {
                self.execute_move(RWTarget::Reg16(Reg16::WZ), RWTarget::Reg16(Reg16::PC), bus, dbg);
                self.ime = true;
            },
            MicroOp::Cpl => {
                self.a = !self.a;
                self.set_flags(0b0110, 0b0110);
            },
            MicroOp::Ccf => {
                let val = (self.get_flag(Flag::C) == 0) as u8;
                self.set_flag(Flag::C, val);
            },
            MicroOp::Scf => {
                self.set_flag(Flag::C, 1);
            },
            MicroOp::Daa => {
                let h = self.get_flag(Flag::H);
                let c = self.get_flag(Flag::C);
                let n = self.get_flag(Flag::N);
                let mut offset = 0;
                let res;

                if (n == 0 && self.a & 0xF > 0x9) || h == 1 {
                    offset |= 0x06;
                }

                if (n == 0 && self.a > 0x99) || c == 1 {
                    offset |= 0x60;
                }

                if n == 0 {
                    res = self.a.wrapping_add(offset);
                } else {
                    res = self.a.wrapping_sub(offset);
                }
                self.a = res;
            },
            MicroOp::Prefix => {
                self.prefix = true
            }
            MicroOp::PrefetchOnly => (),
        };

        if prefetch {
            let old_opcode = self.ir;
            dbg.on_cpu_event(DebugEvent::InstructionEnd(old_opcode), self, bus);
            self.execute_prefetch(bus);
            dbg.on_cpu_event(DebugEvent::IrPrefetch(self.ir, self.pc - 1), self, bus);
        }
        dbg.on_cpu_event(DebugEvent::MicroOpEnd(op), self, bus);

    }

    fn execute_move<T>(&mut self, source: RWTarget, dest: RWTarget, bus: &mut Bus, dbg: &mut T)
    where T: Debugger
    {
        let val = self.get_target(source, bus);
        self.set_target(dest, val, bus);
        match dest {
            RWTarget::Reg8(trg) => dbg.on_cpu_event(DebugEvent::Register8Change(trg, val as u8), self, bus),
            RWTarget::Reg16(trg) => dbg.on_cpu_event(DebugEvent::Register16Change(trg, val), self, bus),
            _ => ()
        };
    }

    fn execute_op(&mut self, op: Operation, bus: &mut Bus) {
        match op {

            /****** Arithmetic ******/

            Operation::Add {left, right, dest, mask} => {
                let lval = Wrapping(self.get_target(left, bus));
                let rval = Wrapping(self.get_target(right, bus));
                let (res, flags) = Self::alu_add(lval, rval, Wrapping(0));
                self.set_target(dest, res, bus);
                self.set_flags(flags, mask);
            },
            Operation::Sub {left, right, dest, mask} => {
                let lval = Wrapping(self.get_target(left, bus));
                let rval = Wrapping(self.get_target(right, bus));
                let (res, flags) = Self::alu_sub(lval, rval, Wrapping(0));
                self.set_target(dest, res, bus);
                self.set_flags(flags, mask);
            },
            Operation::Inc {source, dest, mask} => {
                let val = Wrapping(self.get_target(source, bus));
                let (res, flags) = Self::alu_add(val, Wrapping(1), Wrapping(0));
                self.set_target(dest, res, bus);
                self.set_flags(flags, mask);
            },
            Operation::Dec {source, dest, mask} => {
                let val = Wrapping(self.get_target(source, bus));
                let (res, flags) = Self::alu_sub(val, Wrapping(1), Wrapping(0));
                self.set_target(dest, res, bus);
                self.set_flags(flags, mask);
            }

            Operation::Adc {left, right, dest, mask} => {
                let lval = Wrapping(self.get_target(left, bus));
                let rval = Wrapping(self.get_target(right, bus));
                let carry = Wrapping(self.get_flag(Flag::C) as u16);
                let (res, flags) = Self::alu_add(lval, rval, carry);
                self.set_target(dest, res, bus);
                self.set_flags(flags, mask);
            }

            Operation::Sbc {left, right, dest, mask} => {
                let lval = Wrapping(self.get_target(left, bus));
                let rval = Wrapping(self.get_target(right, bus));
                let carry = Wrapping(self.get_flag(Flag::C) as u16);
                let (res, flags) = Self::alu_sub(lval, rval, carry);
                self.set_target(dest, res, bus);
                self.set_flags(flags, mask);
            }

            Operation::Ads {left, right, dest, mask} => {
                let lval = Wrapping(self.get_target(left, bus));
                let rval = Wrapping(self.get_target(right, bus));
                let signed = Wrapping(rval.0 as u8 as i8 as i16 as u16);
                let (res, flags) = Self::alu_add(lval, signed, Wrapping(0));
                self.set_target(dest, res, bus);
                self.set_flags(flags, mask);
            }

            /****** Logic ******/

            Operation::And {left, right, dest, mask} => {
                let lval = Wrapping(self.get_target(left, bus));
                let rval = Wrapping(self.get_target(right, bus));
                let (res, flags) = Self::alu_and(lval, rval);
                self.set_target(dest, res, bus);
                self.set_flags(flags, mask);
            }

            Operation::Or {left, right, dest, mask} => {
                let lval = Wrapping(self.get_target(left, bus));
                let rval = Wrapping(self.get_target(right, bus));
                let (res, flags) = Self::alu_or(lval, rval);
                self.set_target(dest, res, bus);
                self.set_flags(flags, mask);
            }

            Operation::Xor {left, right, dest, mask} => {
                let lval = Wrapping(self.get_target(left, bus));
                let rval = Wrapping(self.get_target(right, bus));
                let (res, flags) = Self::alu_xor(lval, rval);
                self.set_target(dest, res, bus);
                self.set_flags(flags, mask);
            }

            /****** Shifts ******/

            Operation::Rsh {shift, source, dest, mask} => {
                let val = Wrapping(self.get_target(source, bus) as u8);
                let (res, flags) = Self::alu_rsh(shift, val, self.get_flag(Flag::C));
                self.set_target(dest, res, bus);
                self.set_flags(flags, mask);
            }

            Operation::Lsh {shift, source, dest, mask} => {
                let val = Wrapping(self.get_target(source, bus) as u8);
                let (res, flags) = Self::alu_lsh(shift, val, self.get_flag(Flag::C));
                self.set_target(dest, res, bus);
                self.set_flags(flags, mask);
            },

            Operation::Swp { source, dest, mask } => {
                let val = Wrapping(self.get_target(source, bus) as u8);
                let (res, flags) = Self::alu_swap(val);
                self.set_target(dest, res, bus);
                self.set_flags(flags, mask);
            },

            /****** Bits ******/

            Operation::Bit { source,  bit, mask } => {
                let val = Wrapping(self.get_target(source, bus) as u8);
                let (_, flags) = Self::alu_bit(val, bit);
                self.set_flags(flags, mask);
            },

            Operation::Rsb { source, dest, bit, value } => {
                let val = Wrapping(self.get_target(source, bus) as u8);
                let (res, _) = Self::alu_rsb(val, bit, value);
                self.set_target(dest, res, bus);
            }
        }
    }

    fn execute_read_lsb(&mut self, bus: &mut Bus) {
        let value = bus.read(self.pc);
        self.pc += 1;
        self.tmp16 = (value as u16) | (self.tmp16 & 0xFF00);
    }

    fn execute_read_msb(&mut self, bus: &mut Bus) {
        let value = bus.read(self.pc);
        self.pc += 1;
        self.tmp16 = ((value as u16) << 8) | (self.tmp16 & 0x00FF);
    }

    fn execute_imm(&mut self, bus: &Bus) {
        self.tmp8 = bus.read(self.pc);
        self.tmp16 = self.tmp8 as u16;
        self.pc += 1;
    }

    pub (super) fn execute_prefetch(&mut self, bus: &Bus) {
        self.ir = bus.read(self.pc);
        self.ir_pc = self.pc;
        self.pc += 1;
        if !self.prefix  {
            self.next_ops.append(&mut Self::decode(self.ir));
            self.cond_ops.clear();
            self.cond_ops.append(&mut Self::decode_condition(self.ir));
        } else {
            self.next_ops.append(&mut Self::decode_prefix());
            self.prefix = false;
        }
    }

    fn execute_cc(&mut self, cc: Condition) {
        let val = match cc {
            Condition::Z | Condition::NZ => self.get_flag(Flag::Z),
            Condition::C | Condition::NC => self.get_flag(Flag::C)
        };

        let check = match cc {
            Condition::C | Condition::Z => |v| {v != 0},
            Condition::NC | Condition::NZ => |v| {v == 0},
        };

        if check(val) {
            self.next_ops.append(&mut self.cond_ops)
        } else {
            self.cond_ops.clear();
            self.next_ops.push_back(MicroOp::PrefetchOnly)
        }
    }
}
