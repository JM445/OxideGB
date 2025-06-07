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
            0x00 => Self::decode_noop(),
            0x01 => Self::decode_ld_r16_imm16(Reg16::BC),
            0x02 => Self::decode_ld_indirect_r8(RWTarget::Indirect16(Reg16::BC), RWTarget::Reg8(Reg8::A)),
            0x03 => Self::decode_inc_r16(Reg16::BC),
            0x04 => Self::decode_inc_r8(Reg8::B),
            0x05 => Self::decode_dec_r8(Reg8::B),
            0x06 => Self::decode_ld_imm8(Reg8::B),
            0x07 => Self::decode_rl_r8(Reg8::A, ShiftType::RC),
            0x08 => Self::decode_ld_a16_sp(),
            0x09 => Self::decode_add_r16(Reg16::HL, Reg16::BC, Reg16::HL),
            0x0A => Self::decode_ld_indirect_r8(RWTarget::Reg8(Reg8::A), RWTarget::Indirect16(Reg16::BC)),
            0x0B => Self::decode_dec_r16(Reg16::BC),
            0x0C => Self::decode_inc_r8(Reg8::C),
            0x0D => Self::decode_dec_r8(Reg8::C),
            0x0E => Self::decode_ld_imm8(Reg8::C),
            0x0F => Self::decode_rr_r8(Reg8::A, ShiftType::RC),

            0x10 => Self::decode_stop(),
            0x11 => Self::decode_ld_r16_imm16(Reg16::DE),
            0x12 => Self::decode_ld_indirect_r8(RWTarget::Indirect16(Reg16::DE), RWTarget::Reg8(Reg8::A)),
            0x13 => Self::decode_inc_r16(Reg16::DE),
            0x14 => Self::decode_inc_r8(Reg8::D),
            0x15 => Self::decode_dec_r8(Reg8::D),
            0x16 => Self::decode_ld_imm8(Reg8::D),
            0x17 => Self::decode_rl_r8(Reg8::A, ShiftType::R),
            0x18 => Self::decode_jr(),
            0x19 => Self::decode_add_r16(Reg16::HL, Reg16::DE, Reg16::HL),
            0x1A => Self::decode_ld_indirect_r8(RWTarget::Reg8(Reg8::A), RWTarget::Indirect16(Reg16::DE)),
            0x1B => Self::decode_dec_r16(Reg16::DE),
            0x1C => Self::decode_inc_r8(Reg8::E),
            0x1D => Self::decode_dec_r8(Reg8::E),
            0x1E => Self::decode_ld_imm8(Reg8::E),
            0x1F => Self::decode_rr_r8(Reg8::A, ShiftType::R),

            0x20 => Self::decode_jr_cc(Condition::NZ),
            0x21 => Self::decode_ld_r16_imm16(Reg16::HL),
            0x22 => Self::decode_ld_indirect_r8(RWTarget::Indirect16I(Reg16::HL), RWTarget::Reg8(Reg8::A)),
            0x23 => Self::decode_inc_r16(Reg16::HL),
            0x24 => Self::decode_inc_r8(Reg8::H),
            0x25 => Self::decode_dec_r8(Reg8::H),
            0x26 => Self::decode_ld_imm8(Reg8::H),
            0x27 => Self::decode_daa(),
            0x28 => Self::decode_jr_cc(Condition::Z),
            0x29 => Self::decode_add_r16(Reg16::HL, Reg16::HL, Reg16::HL),
            0x2A => Self::decode_ld_indirect_r8(RWTarget::Reg8(Reg8::A), RWTarget::Indirect16I(Reg16::HL)),
            0x2B => Self::decode_dec_r16(Reg16::HL),
            0x2C => Self::decode_inc_r8(Reg8::L),
            0x2D => Self::decode_dec_r8(Reg8::L),
            0x2E => Self::decode_ld_imm8(Reg8::L),
            0x2F => Self::decode_cpl(),

            0x30 => Self::decode_jr_cc(Condition::NC),
            0x31 => Self::decode_ld_r16_imm16(Reg16::SP),
            0x32 => Self::decode_ld_indirect_r8(RWTarget::Indirect16D(Reg16::HL), RWTarget::Reg8(Reg8::A)),
            0x33 => Self::decode_inc_r16(Reg16::SP),
            0x34 => Self::decode_inc_indirect(Reg16::HL),
            0x35 => Self::decode_dec_indirect(Reg16::HL),
            0x36 => Self::decode_ld_indirect_imm8(Reg16::HL),
            0x37 => Self::decode_scf(),
            0x38 => Self::decode_jr_cc(Condition::C),
            0x39 => Self::decode_add_r16(Reg16::HL, Reg16::SP, Reg16::HL),
            0x3A => Self::decode_ld_indirect_r8(RWTarget::Reg8(Reg8::A), RWTarget::Indirect16D(Reg16::HL)),
            0x3B => Self::decode_dec_r16(Reg16::SP),
            0x3C => Self::decode_inc_r8(Reg8::A),
            0x3D => Self::decode_dec_r8(Reg8::A),
            0x3E => Self::decode_ld_imm8(Reg8::A),
            0x3F => Self::decode_ccf(),

            0x40 => Self::decode_ld_r_r(Reg8::B, Reg8::B),
            0x41 => Self::decode_ld_r_r(Reg8::B, Reg8::C),
            0x42 => Self::decode_ld_r_r(Reg8::B, Reg8::D),
            0x43 => Self::decode_ld_r_r(Reg8::B, Reg8::E),
            0x44 => Self::decode_ld_r_r(Reg8::B, Reg8::H),
            0x45 => Self::decode_ld_r_r(Reg8::B, Reg8::L),
            0x46 => Self::decode_ld_indirect_r8( RWTarget::Reg8(Reg8::B), RWTarget::Indirect16(Reg16::HL)),
            0x47 => Self::decode_ld_r_r(Reg8::B, Reg8::A),
            0x48 => Self::decode_ld_r_r(Reg8::C, Reg8::B),
            0x49 => Self::decode_ld_r_r(Reg8::C, Reg8::C),
            0x4A => Self::decode_ld_r_r(Reg8::C, Reg8::D),
            0x4B => Self::decode_ld_r_r(Reg8::C, Reg8::E),
            0x4C => Self::decode_ld_r_r(Reg8::C, Reg8::H),
            0x4D => Self::decode_ld_r_r(Reg8::C, Reg8::L),
            0x4E => Self::decode_ld_indirect_r8( RWTarget::Reg8(Reg8::C), RWTarget::Indirect16(Reg16::HL)),
            0x4F => Self::decode_ld_r_r(Reg8::C, Reg8::A),

            0x50 => Self::decode_ld_r_r(Reg8::D, Reg8::B),
            0x51 => Self::decode_ld_r_r(Reg8::D, Reg8::C),
            0x52 => Self::decode_ld_r_r(Reg8::D, Reg8::D),
            0x53 => Self::decode_ld_r_r(Reg8::D, Reg8::E),
            0x54 => Self::decode_ld_r_r(Reg8::D, Reg8::H),
            0x55 => Self::decode_ld_r_r(Reg8::D, Reg8::L),
            0x56 => Self::decode_ld_indirect_r8( RWTarget::Reg8(Reg8::D), RWTarget::Indirect16(Reg16::HL)),
            0x57 => Self::decode_ld_r_r(Reg8::D, Reg8::A),
            0x58 => Self::decode_ld_r_r(Reg8::E, Reg8::B),
            0x59 => Self::decode_ld_r_r(Reg8::E, Reg8::C),
            0x5A => Self::decode_ld_r_r(Reg8::E, Reg8::D),
            0x5B => Self::decode_ld_r_r(Reg8::E, Reg8::E),
            0x5C => Self::decode_ld_r_r(Reg8::E, Reg8::H),
            0x5D => Self::decode_ld_r_r(Reg8::E, Reg8::L),
            0x5E => Self::decode_ld_indirect_r8( RWTarget::Reg8(Reg8::E), RWTarget::Indirect16(Reg16::HL)),
            0x5F => Self::decode_ld_r_r(Reg8::E, Reg8::A),

            0x60 => Self::decode_ld_r_r(Reg8::H, Reg8::B),
            0x61 => Self::decode_ld_r_r(Reg8::H, Reg8::C),
            0x62 => Self::decode_ld_r_r(Reg8::H, Reg8::D),
            0x63 => Self::decode_ld_r_r(Reg8::H, Reg8::E),
            0x64 => Self::decode_ld_r_r(Reg8::H, Reg8::H),
            0x65 => Self::decode_ld_r_r(Reg8::H, Reg8::L),
            0x66 => Self::decode_ld_indirect_r8( RWTarget::Reg8(Reg8::H), RWTarget::Indirect16(Reg16::HL)),
            0x67 => Self::decode_ld_r_r(Reg8::H, Reg8::A),
            0x68 => Self::decode_ld_r_r(Reg8::L, Reg8::B),
            0x69 => Self::decode_ld_r_r(Reg8::L, Reg8::C),
            0x6A => Self::decode_ld_r_r(Reg8::L, Reg8::D),
            0x6B => Self::decode_ld_r_r(Reg8::L, Reg8::E),
            0x6C => Self::decode_ld_r_r(Reg8::L, Reg8::H),
            0x6D => Self::decode_ld_r_r(Reg8::L, Reg8::L),
            0x6E => Self::decode_ld_indirect_r8( RWTarget::Reg8(Reg8::L), RWTarget::Indirect16(Reg16::HL)),
            0x6F => Self::decode_ld_r_r(Reg8::L, Reg8::A),

            0x70 => Self::decode_ld_indirect_r8(RWTarget::Indirect16(Reg16::HL), RWTarget::Reg8(Reg8::B)),
            0x71 => Self::decode_ld_indirect_r8(RWTarget::Indirect16(Reg16::HL), RWTarget::Reg8(Reg8::C)),
            0x72 => Self::decode_ld_indirect_r8(RWTarget::Indirect16(Reg16::HL), RWTarget::Reg8(Reg8::D)),
            0x73 => Self::decode_ld_indirect_r8(RWTarget::Indirect16(Reg16::HL), RWTarget::Reg8(Reg8::E)),
            0x74 => Self::decode_ld_indirect_r8(RWTarget::Indirect16(Reg16::HL), RWTarget::Reg8(Reg8::H)),
            0x75 => Self::decode_ld_indirect_r8(RWTarget::Indirect16(Reg16::HL), RWTarget::Reg8(Reg8::L)),
            0x76 => Self::decode_halt(),
            0x77 => Self::decode_ld_indirect_r8(RWTarget::Indirect16(Reg16::HL), RWTarget::Reg8(Reg8::A)),
            0x78 => Self::decode_ld_r_r(Reg8::A, Reg8::B),
            0x79 => Self::decode_ld_r_r(Reg8::A, Reg8::C),
            0x7A => Self::decode_ld_r_r(Reg8::A, Reg8::D),
            0x7B => Self::decode_ld_r_r(Reg8::A, Reg8::E),
            0x7C => Self::decode_ld_r_r(Reg8::A, Reg8::H),
            0x7D => Self::decode_ld_r_r(Reg8::A, Reg8::L),
            0x7E => Self::decode_ld_indirect_r8(RWTarget::Reg8(Reg8::A), RWTarget::Indirect16(Reg16::HL)),
            0x7F => Self::decode_ld_r_r(Reg8::A, Reg8::A),


            _ => panic!("Not implemented yet: {ir:#02x} !")
        }
    }
}
