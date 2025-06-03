#[allow(unused_imports)]
use log::{debug, info, warn, error};

use crate::emulator::memory::Bus;

use super::*;

use std::num::Wrapping;

#[derive(Debug, Copy, Clone)]
pub enum MicroOp {
    DataMove   {source: RWTarget, dest: RWTarget, prefetch: bool},
    DataMoveCC {source: RWTarget, dest: RWTarget, cc: Condition},
    Operation{ope: Operation, prefetch: bool},
    ReadIMM{prefetch: bool},
    ReadLSB{prefetch: bool},
    ReadMSB{prefetch: bool},
    PrefetchOnly
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
    IR,
    IE,
}

#[derive(Debug, Copy, Clone)]
pub enum Condition {
    Z,
    C,
    NZ,
    NC
}

// #[derive(Debug, Copy, Clone)]
// pub enum OpSize {
//     Byte,
//     Word
// }

// impl OpSize {
//     pub fn from(dest: RWTarget) -> Self {
//         match dest {
//             RWTarget::Reg8(_) | RWTarget::Tmp8 |
//             RWTarget::Indirect16(_) | RWTarget::Indirect16D(_) |
//             RWTarget::Indirect16I(_) | RWTarget::Addr => Self::Byte,
//             _ => Self::Word
//         }
//     }
// }

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Add{left: RWTarget, right: RWTarget, dest: RWTarget, mask: u8},
    Sub{left: RWTarget, right: RWTarget, dest: RWTarget, mask: u8},
    Inc{source: RWTarget, dest: RWTarget, mask: u8},
    Dec{source: RWTarget, dest: RWTarget, mask: u8},
    Adc{left: RWTarget, right: RWTarget, dest: RWTarget, mask: u8},
    Sbc{left: RWTarget, right: RWTarget, dest: RWTarget, mask: u8},
    Ads{left: RWTarget, right: RWTarget, dest: RWTarget, mask: u8},
    And{left: RWTarget, right: RWTarget, dest: RWTarget, mask: u8},
    Or {left: RWTarget, right: RWTarget, dest: RWTarget, mask: u8},
    Xor{left: RWTarget, right: RWTarget, dest: RWTarget, mask: u8},
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
            RWTarget::IR => self.ir as u16,
            RWTarget::IE => self.ie as u16,
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
            RWTarget::IR => self.ir = value as u8,
            RWTarget::IE => self.ie = value as u8,
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
        (((left.0 & 0xFF) < (borrow.0 & 0xFF)) as u8) << 3) // C
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

    pub(super) fn execute<T>(&mut self, op: MicroOp, bus: &mut Bus, dbg: &mut T)
    where T: Debugger
    {
        let prefetch = match &op {
            MicroOp::DataMove{prefetch, ..} |
            MicroOp::Operation{prefetch, ..} |
            MicroOp::ReadIMM{prefetch} |
            MicroOp::ReadLSB{prefetch} |
            MicroOp::ReadMSB{prefetch} => *prefetch,
            MicroOp::PrefetchOnly => true,
            MicroOp::DataMoveCC { .. } => false
        };

        match op {
            MicroOp::DataMove{source, dest, ..} => self.execute_move(source, dest, bus, dbg),
            MicroOp::DataMoveCC {source, dest, cc} => {
                self.execute_move(source, dest, bus, dbg);
                self.execute_cc(cc);
            }
            MicroOp::Operation{ope, ..} => self.execute_op(ope, bus),
            MicroOp::ReadIMM{..} => self.execute_imm(bus),
            MicroOp::ReadLSB{..}  => self.execute_read_lsb(bus),
            MicroOp::ReadMSB{..}  => self.execute_read_msb(bus),
            MicroOp::PrefetchOnly => (),
        };

        if prefetch {
            let old_opcode = self.ir;
            self.execute_prefetch(bus);
            dbg.on_cpu_event(DebugEvent::IrPrefetch(self.ir), self, bus);
            dbg.on_cpu_event(DebugEvent::InstructionEnd(old_opcode), self, bus);
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
        self.pc += 1;
    }

    fn execute_prefetch(&mut self, bus: &Bus) {
        self.ir = bus.read(self.pc);
        self.pc += 1;
        self.next_ops.append(&mut Self::decode(self.ir));
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
