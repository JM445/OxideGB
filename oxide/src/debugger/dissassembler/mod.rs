pub mod opcodes;

use std::collections::{HashMap, HashSet};
use log::debug;
use opcodes::*;
use crate::emulator::cpu::Cpu;
use crate::emulator::memory::{Bus, MemBlock};


/*
 * Struct and utilities to map code paths dynamically
 */

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct InstructionMeta {
    pub opcode: u8,
    pub addr: u16,
    pub full_bytes: [u8; 4],
    pub size: usize,
    pub next: u16,
    pub target: Option<u16>,
    pub mem_block: MemBlock,

    pub is_cond: bool,
    pub is_call: bool,
    pub is_ret: bool,
    pub is_jump: bool,
    pub is_dynamic: bool,
}

pub struct CodeBlock {
    pub start_addr: u16,
    pub instructions: Vec<InstructionMeta>,
    pub invalid: bool,
    pub size: usize,
    pub mem_block: MemBlock,
    pub hash: u64,
    visited: HashSet<u16>,
    linked: HashSet<u16>,
    
}

pub struct CodeMap {
    pub blocks: HashMap<u16, CodeBlock>,
}

impl InstructionMeta {
    pub fn new(addr: u16, bus: &Bus) -> Self {
        let opcode = bus.read(addr);
        let size = get_instruction_length(opcode);
        Self {
            opcode,
            addr,
            full_bytes: bus.get_instruction(addr),
            size:  size as usize,
            next: addr + size,
            target: Self::get_target(addr, bus),
            mem_block: MemBlock::from_addr(addr),

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
    pub fn new(addr: u16, bus: &Bus) -> CodeBlock {
        let mut res = CodeBlock {
            start_addr: addr,
            instructions: Vec::new(),
            invalid: false,
            size: 0,
            visited: HashSet::new(),
            linked: HashSet::new(),
            mem_block: MemBlock::from_addr(addr),
            hash: 0,
        };

        res.init(bus);
        res
    }

    // Analyse a Code Block
    fn init(&mut self, bus: &Bus) {
        let mut cur: InstructionMeta;
        let mut addr = self.start_addr;

        loop {
            // Get current instruction data
            cur = InstructionMeta::new(addr, bus);
            self.instructions.push(cur.clone());
            self.size += cur.size;
            // Add all bytes of the instruction to visited list
            for i in 0..cur.size {
                self.visited.insert(addr + i as u16);
            }

            // If instruction have a target, add it to linked list
            if let Some(t) = cur.target {
                self.linked.insert(t);
            }

            // If it's a dead end, block is analyzed
            if cur.is_dead_end() || cur.mem_block != MemBlock::from_addr(cur.next) {
                break;
            }
            addr += cur.size as u16;
        }
        
        self.hash = bus.hash_region(self.start_addr, self.size);

        // Remove from linked list all addresses that are in the current block
        self.linked.retain(|e| !self.visited.contains(e));
    }
    
    pub fn has_changes(&self, bus: &Bus) -> bool {
        self.hash == bus.hash_region(self.start_addr, self.size)
    }

    pub fn update(&mut self, bus: &Bus) {
        self.visited.clear();
        self.linked.clear();
        self.instructions.clear();
        self.size = 0;
        self.init(bus);
    }
}

impl CodeMap {
    pub fn new(starting_addr: u16) -> Self {
        Self {
            blocks: HashMap::new(),
        }
    }

    pub fn get_block(&mut self, bus: &Bus, cpu: &Cpu) -> (&CodeBlock, bool) {
        let cur_block: &mut CodeBlock;
        let mut new_block = false;
        let addr = cpu.ir_pc;

        let search = self.blocks.iter().find(|b| {
            addr >= b.1.start_addr && addr < b.1.start_addr + b.1.size as u16
        }).map(|f| f.0.clone());
        if let Some(found) = search {
            debug!("Found a CodeBlock for address: {addr:#06X}.");
            cur_block = self.blocks.get_mut(&found).unwrap();
            if cur_block.has_changes(bus) {
                cur_block.update(bus)
            }
        } else {
            debug!("No CodeBlock found for address: {addr:#06X}. Creating one.");
            let block = CodeBlock::new(addr, bus);
            self.blocks.insert(addr, block);
            cur_block = self.blocks.get_mut(&addr).unwrap();
            new_block = true;
        }

        (cur_block, new_block)
    }
}