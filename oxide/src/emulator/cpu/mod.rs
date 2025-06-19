pub mod registers;
pub mod micro_ops;
pub mod decoder;
pub mod displays;

mod inline_ld_decoder;
mod inline_alu_decoder;
mod inline_jump_decoder;
mod inline_misc_decoder;
mod inline_binop_decoder;

use micro_ops::*;
use registers::*;

use crate::debugger::DebugEvent;
use crate::debugger::Debugger;
use crate::emulator::memory::Bus;

use std::collections::VecDeque;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

#[derive(Debug)]
pub struct Cpu {
    pub a: u8, // Accumulator
    pub f: u8, // Flags
    pub b: u8, // General Purpose
    pub c: u8, // General Purpose
    pub d: u8, // General Purpose
    pub e: u8, // General Purpose
    pub h: u8, // General Purpose
    pub l: u8, // General Purpose
    
    pub w: u8, // TMP Register
    pub z: u8, // TMP Register

    pub sp: u16, // Stack Pointer
    pub pc: u16, // Program Counter

    pub ime: bool, // Iterrupt Enable flag
    pub ir: u8,    // Instruction Register
    pub ir_pc: u16,// Address of current instruction
    prefix: bool,  // Was the last decoded instruction the 0xCB prefix ?
    ei_next: bool, // Is an EI scheduled for next cycle ?
    next_ops: VecDeque<MicroOp>,
    cond_ops: VecDeque<MicroOp>,
}

impl Cpu {
    pub fn new(initial_pc: u16) -> Cpu {
        Cpu {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            
            w: 0,
            z: 0,

            sp: 0,
            pc: initial_pc,

            ime: false,
            ir: 0,
            ir_pc: 0,
            prefix: false,
            ei_next: false,
            next_ops: VecDeque::new(),
            cond_ops: VecDeque::new()
        }
    }

    pub fn tick<T>(&mut self, bus: &mut Bus, dbg: &mut T)
        where T: Debugger
    {
        if self.ei_next {
            self.ei_next = false;
            self.ime = true;
        }
        let res = self.next_ops.pop_front();
        match res {
            Some(op) => self.execute(op, bus, dbg),
            None => self.execute_prefetch(bus)
        };
    }
}
