pub mod opcodes;

use std::collections::{HashMap, HashSet};
use opcodes::*;
use crate::emulator::memory::Bus;


/*
 * Struct and utilities to map code paths dynamically
 */

pub struct InstructionMeta {
    pub opcode: u8,
    pub addr: u16,
    pub full_bytes: [u8; 4],
    pub size: u8,
    pub target: Option<u16>,

    pub is_cond: bool,
    pub is_call: bool,
    pub is_ret: bool,
    pub is_jump: bool,
    pub is_dynamic: bool,
}

pub struct CodeBlock {
    pub start_addr: u16,
    pub instructions: Vec<InstructionMeta>,
    pub dynamic: bool,
    pub invalid: bool,
    size: u16,
    visited: HashSet<u16>,
    bank: usize
}

pub struct CodeMap {
    pub rom_blocks: HashMap<(usize, u16), CodeBlock>,
    pub ram_blocks: HashMap<(usize, u16), CodeBlock>,
}

impl InstructionMeta {
    pub fn new(addr: u16, bus: &Bus) -> Self {
        let opcode = bus.read(addr);
        Self {
            opcode,
            addr,
            full_bytes: bus.get_instruction(addr),
            size: get_instruction_length(opcode) as u8,
            target: Self::get_target(addr, bus),

            is_cond: Self::is_conditional(opcode),
            is_call: Self::is_call(opcode),
            is_ret: Self::is_ret(opcode),
            is_jump: Self::is_jump(opcode),
            is_dynamic: Self::is_dynamic(opcode)
        }
    }

    pub fn to_string(&self) -> String {
        disassemble(&self.full_bytes)
    }
}

impl CodeBlock {
    pub fn new(addr: u16, bank: usize, bus: &Bus) -> CodeBlock {
        let mut res = CodeBlock {
            start_addr: addr,
            instructions: Vec::new(),
            dynamic: Self::is_dynamic(addr),
            invalid: false,
            size: 0,
            visited: HashSet::new(),
            bank,
        };

        res.init(bus);
        res
    }

    fn init(&mut self, bus: &Bus) {
        
    }
}