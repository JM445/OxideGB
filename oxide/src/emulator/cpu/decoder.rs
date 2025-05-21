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
            0x01 | 0x11 | 0x21 | 0x31 => Self::decode_ld_imm16(ir),
            0x02 | 0x12 | 0x46 | 0x56 | 0x66 | 0x0A | 0x1A | 0x4E | 0x5E | 0x6E | 0x7E |
            0x77 | 0x70..=0x75 => Self::decode_ld_indirect_reg(ir),
            _ => panic!("Not implemented yet: {ir:#02x} !")
        }
    }

    // Decode LD [R16], R8 and LD R8, [R16]  instructions
    fn decode_ld_indirect_reg(ir: u8) -> VecDeque<MicroOp> {
        let (source, dest) = match ir {
            0x02 => (RWTarget::Reg8(Reg8::A), RWTarget::Indirect16(Reg16::BC)),
            0x12 => (RWTarget::Reg8(Reg8::A), RWTarget::Indirect16(Reg16::DE)),

            0x46 => (RWTarget::Indirect16(Reg16::HL), RWTarget::Reg8(Reg8::B)),
            0x56 => (RWTarget::Indirect16(Reg16::HL), RWTarget::Reg8(Reg8::D)),
            0x66 => (RWTarget::Indirect16(Reg16::HL), RWTarget::Reg8(Reg8::H)),

            0x0A => (RWTarget::Indirect16(Reg16::BC), RWTarget::Reg8(Reg8::A)),
            0x1A => (RWTarget::Indirect16(Reg16::DE), RWTarget::Reg8(Reg8::A)),

            0x4E => (RWTarget::Indirect16(Reg16::HL), RWTarget::Reg8(Reg8::C)),
            0x5E => (RWTarget::Indirect16(Reg16::HL), RWTarget::Reg8(Reg8::E)),
            0x6E => (RWTarget::Indirect16(Reg16::HL), RWTarget::Reg8(Reg8::L)),
            0x7E => (RWTarget::Indirect16(Reg16::HL), RWTarget::Reg8(Reg8::A)),

            0x70 => (RWTarget::Reg8(Reg8::B), RWTarget::Indirect16(Reg16::HL)),
            0x71 => (RWTarget::Reg8(Reg8::C), RWTarget::Indirect16(Reg16::HL)),
            0x72 => (RWTarget::Reg8(Reg8::D), RWTarget::Indirect16(Reg16::HL)),
            0x73 => (RWTarget::Reg8(Reg8::E), RWTarget::Indirect16(Reg16::HL)),
            0x74 => (RWTarget::Reg8(Reg8::H), RWTarget::Indirect16(Reg16::HL)),
            0x75 => (RWTarget::Reg8(Reg8::L), RWTarget::Indirect16(Reg16::HL)),
            0x77 => (RWTarget::Reg8(Reg8::A), RWTarget::Indirect16(Reg16::HL)),
            _ => panic!("Not implemented yet: {ir:#02x} !")
        };
        VecDeque::from(vec![
                MicroOp::DataMove{source, dest, prefetch: false},
                MicroOp::PrefetchOnly,
        ])
    }

    // Decode LD R16, imm16
    fn decode_ld_imm16(ir: u8) -> VecDeque<MicroOp> {
        let dest = match ir {
            0x01 => RWTarget::Reg16(Reg16::BC),
            0x11 => RWTarget::Reg16(Reg16::DE),
            0x21 => RWTarget::Reg16(Reg16::HL),
            0x31 => RWTarget::Reg16(Reg16::SP),
            _ => panic!("Not implemented yet: {ir:#02x} !")
        };
        VecDeque::from(vec![
            MicroOp::ReadLSB{prefetch: false},
            MicroOp::ReadMSB{prefetch: false},
            MicroOp::DataMove{source: RWTarget::Tmp16, dest, prefetch: true},
        ])
    }
}
