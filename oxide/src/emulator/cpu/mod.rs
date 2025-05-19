pub mod registers;
pub mod micro_ops;

use registers::*;
use micro_ops::*;

use crate::emulator::memory::Bus;

use std::collections::VecDeque;

pub struct Cpu {
    pub a: u8, // Accumulator
    pub f: u8, // Flags
    pub b: u8, // General Purpose
    pub c: u8, // General Purpose
    pub d: u8, // General Purpose
    pub e: u8, // General Purpose
    pub h: u8, // General Purpose
    pub l: u8, // General Purpose

    pub sp: u16, // Stack Pointer
    pub pc: u16, // Program Counter

    pub ir: u8,  // Instruction Register
    pub ie: u8,  // Interrupt Enable

    tmp8: u8,
    tmp16: u16,
    next_ops: VecDeque<MicroOp>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,

            sp: 0,
            pc: 0,

            ir: 0,
            ie: 0,

            tmp8: 0,
            tmp16: 0,
            next_ops: VecDeque::new()
        }
    }

    pub fn tick(&mut self, bus: &mut Bus) {
        let res = self.next_ops.pop_front();
        match res {
            Some(op) => self.execute(op, bus),
            None => ()
        }
    }
}
