pub mod registers;
pub mod micro_ops;
pub mod decoder;
pub mod displays;

mod inline_ld_decoder;
mod inline_alu_decoder;
mod inline_jump_decoder;
mod inline_misc_decoder;
mod inline_binop_decoder;
pub(crate) mod interrupt;

use micro_ops::*;
use registers::*;

use crate::debugger::DebugEvent;
use crate::debugger::Debugger;
use crate::emulator::memory::Bus;

use std::collections::VecDeque;

#[allow(unused_imports)]
use log::{debug, error, info, warn};
use crate::emu_print;
use crate::emulator::memory::RegDefines::*;
use crate::settings::GLOB_SETTINGS;

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
    
    pub halted: bool, // Is CPU Halted ?
    
    prefix: bool,  // Was the last decoded instruction the 0xCB prefix ?
    ei_next: bool, // Is an EI scheduled for next cycle ?
    next_ops: VecDeque<MicroOp>,
    cond_ops: VecDeque<MicroOp>,
}

impl Cpu {
    
    pub fn new_noboot() -> Self {
        Cpu {
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,

            w: 0,
            z: 0,

            sp: 0xFFFE,
            pc: 0x0100,

            ime: false,
            ir: 0,
            ir_pc: 0,
            
            halted: false,
            prefix: false,
            ei_next: false,
            next_ops: VecDeque::new(),
            cond_ops: VecDeque::new()
        }
    }
    pub fn new_boot() -> Self {
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
            pc: 0x0000,

            ime: false,
            ir: 0,
            ir_pc: 0,
            halted: false,
            
            prefix: false,
            ei_next: false,
            next_ops: VecDeque::new(),
            cond_ops: VecDeque::new()
        }
    }
    
    // Returns a log String formated for GameBoy Doctor
    // Format: A:00 F:11 B:22 C:33 D:44 E:55 H:66 L:77 SP:8888 PC:9999 PCMEM:AA,BB,CC,DD
    pub fn get_doctor_log(&self, bus: &Bus) -> String {
        format!("A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}\n",
            self.a, self.f, self.b, self.c, 
            self.d, self.e, self.h, self.l,
            self.sp, self.pc, 
            bus.read(self.pc), bus.read(self.pc + 1),
            bus.read(self.pc + 2), bus.read(self.pc + 3)
        )
    }

    pub fn tick<T>(&mut self, bus: &mut Bus, dbg: &mut T)
        where T: Debugger
    {
        if self.halted && ((bus.read(IF) & 0x1F) & (bus.read(IE) & 0x1F)) != 0 {
            self.execute_interrupt(bus);
            self.halted = false;
        }
        
        if self.halted {
            return
        }
        if self.ei_next {
            self.ei_next = false;
            self.ime = true;
        }
        let res = self.next_ops.pop_front();
        match res {
            Some(op) => self.execute(op, bus, dbg),
            None => {
                self.execute_prefetch(bus)
            }
        };
    }
}
