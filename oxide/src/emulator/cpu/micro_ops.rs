#[allow(unused_imports)]
use log::{debug, info, warn, error};

use crate::emulator::memory::Bus;

use super::*;

#[derive(Debug, Copy, Clone)]
pub enum MicroOp {
    DataMove {source: RWTarget, dest: RWTarget, prefetch: bool},
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
    Addr(u16),
    Indirect16(Reg16),
    Indirect8(Reg8),
    Tmp8,
    Tmp16,
    IR,
    IE,
}

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Add{left: RWTarget, right: RWTarget, dest: RWTarget},
    Sub{left: RWTarget, right: RWTarget, dest: RWTarget},
    Inc{dest: RWTarget},
    Dec{dest: RWTarget}
}

impl Cpu {
    fn get_target(&self, target: RWTarget, bus: &Bus) -> u16 {
        match target {
            RWTarget::Reg8(trg) => self.read8(trg) as u16,
            RWTarget::Reg16(trg) => self.read16(trg),
            RWTarget::Addr(trg) => bus.read(trg) as u16,
            RWTarget::Indirect8(_) => panic!("Unimplemented Indirect read !"),
            RWTarget::Indirect16(trg) => bus.read(self.read16(trg)) as u16,
            RWTarget::Tmp8 => self.tmp8 as u16,
            RWTarget::Tmp16 => self.tmp16,
            RWTarget::IR => self.ir as u16,
            RWTarget::IE => self.ie as u16
        }
    }

    fn set_target(&mut self, target: RWTarget, value: u16, bus: &mut Bus) {
        match target {
            RWTarget::Reg8(trg) => self.write8(trg, value as u8),
            RWTarget::Reg16(trg) => self.write16(trg, value),
            RWTarget::Addr(trg) => bus.write(trg, value as u8),
            RWTarget::Indirect8(_) => panic!("Unimplemented Indirect write !"),
            RWTarget::Indirect16(trg) => bus.write(self.read16(trg), value as u8),
            RWTarget::Tmp8 => self.tmp8 = value as u8,
            RWTarget::Tmp16 => self.tmp16 = value,
            RWTarget::IR => self.ir = value as u8,
            RWTarget::IE => self.ie = value as u8
        };
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
            MicroOp::PrefetchOnly => true
        };

        match op {
            MicroOp::DataMove{source, dest, ..} => self.execute_move(source, dest, bus, dbg),
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
            Operation::Add {left, right, dest} => {
                let lval = self.get_target(left, bus);
                let rval = self.get_target(right, bus);
                self.set_target(dest, lval + rval, bus);
            },
            Operation::Sub {left, right, dest} => {
                let lval = self.get_target(left, bus);
                let rval = self.get_target(right, bus);
                self.set_target(dest, lval - rval, bus);
            },
            Operation::Inc {dest} => {
                let val = self.get_target(dest, bus);
                self.set_target(dest, val + 1, bus);
            },
            Operation::Dec {dest} => {
                let val = self.get_target(dest, bus);
                self.set_target(dest, val - 1, bus);
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
}
