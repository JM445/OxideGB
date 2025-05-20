#[allow(unused_imports)]
use log::{debug, info, warn, error};

use super::*;

use std::collections::VecDeque;

/*
 * CPU Instruction decoder.
 * Each instruction is converted to a deque of micro operations (see micro_ops.rs)
 * decode() is the entrypoint and dispatches decoding to sub-functions
 */

impl Cpu {

    // Main decoding entrypoint
    // Input:
    //   ir: Opcode
    //
    // Output: A deque of the next MicroOps to execute
    pub(super) fn decode(ir: u8) -> VecDeque<MicroOp> {
        match ir {
            0x00 => VecDeque::from(vec![MicroOp::PrefetchOnly]),
            0x02 | 0x12 | 0x22 | 0x32 | 0x36 => Self::decode_ld_to_indirect_reg(ir),
            _ => panic!("Not implemented yet: {ir:#02x} !")
        }
    }

    // Decode LD [R16], R8 instructions.
    fn decode_ld_to_indirect_reg(ir: u8) -> VecDeque<MicroOp> {
        match ir {
            0x02 => VecDeque::from(vec![
                MicroOp::DataMove{source: RWTarget::Reg8(Reg8::A), dest: RWTarget::Indirect16(Reg16::BC), prefetch: false},
                MicroOp::PrefetchOnly
            ]),
            0x12 => VecDeque::from(vec![
                MicroOp::DataMove{source: RWTarget::Reg8(Reg8::A), dest: RWTarget::Indirect16(Reg16::DE), prefetch: false},
                MicroOp::PrefetchOnly
            ]),
            0x22 => VecDeque::from(vec![
                MicroOp::DataMove{source: RWTarget::Reg8(Reg8::A), dest: RWTarget::Indirect16(Reg16::HL), prefetch: false},
                MicroOp::Operation{ope: Operation::Inc{dest: RWTarget::Reg16(Reg16::HL)}, prefetch: true},
            ]),
            0x32 => VecDeque::from(vec![
                MicroOp::DataMove{source: RWTarget::Reg8(Reg8::A), dest: RWTarget::Indirect16(Reg16::HL), prefetch: false},
                MicroOp::Operation{ope: Operation::Dec{dest: RWTarget::Reg16(Reg16::HL)}, prefetch: true},
            ]),
            _ => panic!("Not implemented yet: {ir:#02x} !")
        }
    }
}
