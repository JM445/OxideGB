#[allow(unused_imports)]
use log::{debug, error, info, warn};

use super::*;

use crate::emulator::cpu::micro_ops::ShiftType::*;
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

            0x80 => Self::decode_add_r8(Reg8::A, Reg8::B, Reg8::A),
            0x81 => Self::decode_add_r8(Reg8::A, Reg8::C, Reg8::A),
            0x82 => Self::decode_add_r8(Reg8::A, Reg8::D, Reg8::A),
            0x83 => Self::decode_add_r8(Reg8::A, Reg8::E, Reg8::A),
            0x84 => Self::decode_add_r8(Reg8::A, Reg8::H, Reg8::A),
            0x85 => Self::decode_add_r8(Reg8::A, Reg8::L, Reg8::A),
            0x86 => Self::decode_add_indirect(Reg8::A, Reg16::HL, Reg8::A),
            0x87 => Self::decode_add_r8(Reg8::A, Reg8::A, Reg8::A),
            0x88 => Self::decode_adc_r8(Reg8::A, Reg8::B, Reg8::A),
            0x89 => Self::decode_adc_r8(Reg8::A, Reg8::C, Reg8::A),
            0x8A => Self::decode_adc_r8(Reg8::A, Reg8::D, Reg8::A),
            0x8B => Self::decode_adc_r8(Reg8::A, Reg8::E, Reg8::A),
            0x8C => Self::decode_adc_r8(Reg8::A, Reg8::H, Reg8::A),
            0x8D => Self::decode_adc_r8(Reg8::A, Reg8::L, Reg8::A),
            0x8E => Self::decode_adc_indirect(Reg8::A, Reg16::HL, Reg8::A),
            0x8F => Self::decode_adc_r8(Reg8::A, Reg8::A, Reg8::A),

            0x90 => Self::decode_sub_r8(Reg8::A, Reg8::B, Reg8::A),
            0x91 => Self::decode_sub_r8(Reg8::A, Reg8::C, Reg8::A),
            0x92 => Self::decode_sub_r8(Reg8::A, Reg8::D, Reg8::A),
            0x93 => Self::decode_sub_r8(Reg8::A, Reg8::E, Reg8::A),
            0x94 => Self::decode_sub_r8(Reg8::A, Reg8::H, Reg8::A),
            0x95 => Self::decode_sub_r8(Reg8::A, Reg8::L, Reg8::A),
            0x96 => Self::decode_sub_indirect(Reg8::A, Reg16::HL, Reg8::A),
            0x97 => Self::decode_sub_r8(Reg8::A, Reg8::A, Reg8::A),
            0x98 => Self::decode_sbc_r8(Reg8::A, Reg8::B, Reg8::A),
            0x99 => Self::decode_sbc_r8(Reg8::A, Reg8::C, Reg8::A),
            0x9A => Self::decode_sbc_r8(Reg8::A, Reg8::D, Reg8::A),
            0x9B => Self::decode_sbc_r8(Reg8::A, Reg8::E, Reg8::A),
            0x9C => Self::decode_sbc_r8(Reg8::A, Reg8::H, Reg8::A),
            0x9D => Self::decode_sbc_r8(Reg8::A, Reg8::L, Reg8::A),
            0x9E => Self::decode_sbc_indirect(Reg8::A, Reg16::HL, Reg8::A),
            0x9F => Self::decode_sbc_r8(Reg8::A, Reg8::A, Reg8::A),

            0xA0 => Self::decode_and_r8(Reg8::A, Reg8::B, Reg8::A),
            0xA1 => Self::decode_and_r8(Reg8::A, Reg8::C, Reg8::A),
            0xA2 => Self::decode_and_r8(Reg8::A, Reg8::D, Reg8::A),
            0xA3 => Self::decode_and_r8(Reg8::A, Reg8::E, Reg8::A),
            0xA4 => Self::decode_and_r8(Reg8::A, Reg8::H, Reg8::A),
            0xA5 => Self::decode_and_r8(Reg8::A, Reg8::L, Reg8::A),
            0xA6 => Self::decode_and_indirect(Reg8::A, Reg16::HL, Reg8::A),
            0xA7 => Self::decode_and_r8(Reg8::A, Reg8::A, Reg8::A),
            0xA8 => Self::decode_xor_r8(Reg8::A, Reg8::B, Reg8::A),
            0xA9 => Self::decode_xor_r8(Reg8::A, Reg8::C, Reg8::A),
            0xAA => Self::decode_xor_r8(Reg8::A, Reg8::D, Reg8::A),
            0xAB => Self::decode_xor_r8(Reg8::A, Reg8::E, Reg8::A),
            0xAC => Self::decode_xor_r8(Reg8::A, Reg8::H, Reg8::A),
            0xAD => Self::decode_xor_r8(Reg8::A, Reg8::L, Reg8::A),
            0xAE => Self::decode_xor_indirect(Reg8::A, Reg16::HL, Reg8::A),
            0xAF => Self::decode_xor_r8(Reg8::A, Reg8::A, Reg8::A),

            0xB0 => Self::decode_or_r8(Reg8::A, Reg8::B, Reg8::A),
            0xB1 => Self::decode_or_r8(Reg8::A, Reg8::C, Reg8::A),
            0xB2 => Self::decode_or_r8(Reg8::A, Reg8::D, Reg8::A),
            0xB3 => Self::decode_or_r8(Reg8::A, Reg8::E, Reg8::A),
            0xB4 => Self::decode_or_r8(Reg8::A, Reg8::H, Reg8::A),
            0xB5 => Self::decode_or_r8(Reg8::A, Reg8::L, Reg8::A),
            0xB6 => Self::decode_or_indirect(Reg8::A, Reg16::HL, Reg8::A),
            0xB7 => Self::decode_or_r8(Reg8::A, Reg8::A, Reg8::A),
            0xB8 => Self::decode_cp_r8(Reg8::A, Reg8::B),
            0xB9 => Self::decode_cp_r8(Reg8::A, Reg8::C),
            0xBA => Self::decode_cp_r8(Reg8::A, Reg8::D),
            0xBB => Self::decode_cp_r8(Reg8::A, Reg8::E),
            0xBC => Self::decode_cp_r8(Reg8::A, Reg8::H),
            0xBD => Self::decode_cp_r8(Reg8::A, Reg8::L),
            0xBE => Self::decode_cp_indirect(Reg8::A, Reg16::HL),
            0xBF => Self::decode_cp_r8(Reg8::A, Reg8::A),

            0xC0 => Self::decode_ret_cc(Condition::NZ),
            0xC1 => Self::decode_pop(Reg16::BC),
            0xC2 => Self::decode_jp_cc_nn(Condition::NZ),
            0xC3 => Self::decode_jp_nn(),
            0xC4 => Self::decode_call_cc(Condition::NZ),
            0xC5 => Self::decode_push(Reg16::BC),
            0xC6 => Self::decode_add_imm(Reg8::A, Reg8::A),
            0xC7 => Self::decode_rst(0x00),
            0xC8 => Self::decode_ret_cc(Condition::Z),
            0xC9 => Self::decode_ret(),
            0xCA => Self::decode_jp_cc_nn(Condition::Z),
            0xCB => Self::decode_prefix(),
            0xCC => Self::decode_call_cc(Condition::Z),
            0xCD => Self::decode_call(),
            0xCE => Self::decode_adc_imm(Reg8::A, Reg8::A),
            0xCF => Self::decode_rst(0x08),
            
            0xD0 => Self::decode_ret_cc(Condition::NC),
            0xD1 => Self::decode_pop(Reg16::DE),
            0xD2 => Self::decode_jp_cc_nn(Condition::NC),
            0xD3 => Self::decode_invalid(0xD3),
            0xD4 => Self::decode_call_cc(Condition::NC),
            0xD5 => Self::decode_push(Reg16::DE),
            0xD6 => Self::decode_sub_imm(Reg8::A, Reg8::A),
            0xD7 => Self::decode_rst(0x10),
            0xD8 => Self::decode_ret_cc(Condition::C),
            0xD9 => Self::decode_reti(),
            0xDA => Self::decode_jp_cc_nn(Condition::C),
            0xDB => Self::decode_invalid(0xDB),
            0xDC => Self::decode_call_cc(Condition::C),
            0xDD => Self::decode_invalid(0xDD),
            0xDE => Self::decode_sbc_imm(Reg8::A, Reg8::A),
            0xDF => Self::decode_rst(0x18),
            
            0xE0 => Self::decode_ldh_imm_a(),
            0xE1 => Self::decode_pop(Reg16::HL),
            0xE2 => Self::decode_ldh_c_a(),
            0xE3 => Self::decode_invalid(0xE3),
            0xE4 => Self::decode_invalid(0xE4),
            0xE5 => Self::decode_push(Reg16::HL),
            0xE6 => Self::decode_and_imm(Reg8::A, Reg8::A),
            0xE7 => Self::decode_rst(0x20),
            0xE8 => Self::decode_add_sp_e8(),
            0xE9 => Self::decode_jp_hl(),
            0xEA => Self::decode_ld_a16_a(),
            0xEB => Self::decode_invalid(0xEB),
            0xEC => Self::decode_invalid(0xEC),
            0xED => Self::decode_invalid(0xED),
            0xEE => Self::decode_xor_imm(Reg8::A, Reg8::A),
            0xEF => Self::decode_rst(0x28),
            
            0xF0 => Self::decode_ldh_a_imm(),
            0xF1 => Self::decode_pop(Reg16::AF),
            0xF2 => Self::decode_ldh_a_c(),
            0xF3 => Self::decode_di(),
            0xF4 => Self::decode_invalid(0xF4),
            0xF5 => Self::decode_push(Reg16::AF),
            0xF6 => Self::decode_or_imm(Reg8::A, Reg8::A),
            0xF7 => Self::decode_rst(0x30),
            0xF8 => Self::decode_ld_hl_sp_e8(),
            0xF9 => Self::decode_ld_sp_hl(),
            0xFA => Self::decode_ld_a_a16(),
            0xFB => Self::decode_ei(),
            0xFC => Self::decode_invalid(0xFC),
            0xFD => Self::decode_invalid(0xFD),
            0xFE => Self::decode_cp_imm(Reg8::A),
            0xFF => Self::decode_rst(0x38),
        }
    }

    pub (super) fn decode_prefix_opcode(ir: u8) -> VecDeque<MicroOp> {
        let x = ir >> 6;
        let y = (ir & 0b00111000) >> 3;
        let z = ir & 0b00000111;

        let target = match z {
            0 => RWTarget::Reg8(Reg8::B),
            1 => RWTarget::Reg8(Reg8::C),
            2 => RWTarget::Reg8(Reg8::D),
            3 => RWTarget::Reg8(Reg8::E),
            4 => RWTarget::Reg8(Reg8::H),
            5 => RWTarget::Reg8(Reg8::L),
            6 => RWTarget::Indirect16(Reg16::HL),
            7 => RWTarget::Reg8(Reg8::A),
            _ => panic!("Impossible Z value"),
        };

        match x {
            0 => Self::decode_rot_opcode(target, y),
            1 => Self::decode_bit_opcode(target, y),
            2 => Self::decode_res_opcode(target, y),
            3 => Self::decode_set_opcode(target, y),
            _ => panic!("Impossible X value"),
        }
    }

    fn decode_set_opcode(target: RWTarget, y: u8) -> VecDeque<MicroOp> {
        match target {
            RWTarget::Reg8(r) => Self::decode_rsb_rr(r, y, 1),
            RWTarget::Indirect16(_) => Self::decode_rsb_indirect(y, 1),
            _ => panic!("Should be unreachable")
        }
    }

    fn decode_res_opcode(target: RWTarget, y: u8) -> VecDeque<MicroOp> {
        match target {
            RWTarget::Reg8(r) => Self::decode_rsb_rr(r, y, 0),
            RWTarget::Indirect16(_) => Self::decode_rsb_indirect(y, 0),
            _ => panic!("Should be unreachable")
        }
    }
    fn decode_bit_opcode(target: RWTarget, y: u8) -> VecDeque<MicroOp> {
        match target {
            RWTarget::Reg8(r) => Self::decode_bit_rr(r, y),
            RWTarget::Indirect16(_) => Self::decode_bit_indirect(y),
            _ => panic!("Should be unreachable")
        }
    }
    fn decode_rot_opcode(target: RWTarget, y: u8) -> VecDeque<MicroOp> {
        let (left, shift) = match y {
            0 => (true, RC),
            1 => (false, RC),
            2 => (true, R),
            3 => (false, R),
            4 => (true, SA),
            5 => (false, SA),
            7 => (false, SL),
            6 => {
                match target {
                    RWTarget::Reg8(r) => {return Self::decode_swap_rr(r);},
                    RWTarget::Indirect16(_) => {return Self::decode_swap_indirect();},
                    _ => panic!("Should be unreachable")
                };
            }
            _ => panic!("Should be unreachable")
        };

        if left {
            match target {
                RWTarget::Reg8(r) => {return Self::decode_rl_r8(r, shift);},
                RWTarget::Indirect16(_) => {return Self::decode_rl_indirect(shift)},
                _ => panic!("Should be unreachable")
            };
        } else {
            match target {
                RWTarget::Reg8(r) => {return Self::decode_rr_r8(r, shift);},
                RWTarget::Indirect16(_) => {return Self::decode_rr_indirect(shift)},
                _ => panic!("Should be unreachable")
            };
        }
    }

    pub (super) fn decode_condition(ir: u8) -> VecDeque<MicroOp> {
        match ir {
            0x20 | 0x30 | 0x28 | 0x38 => Self::append_jr_cc(),
            0xC0 | 0xD0 | 0xC8 | 0xD8 => Self::decode_ret(),
            0xC2 | 0xD2 | 0xCA | 0xDA => Self::append_jp_cc_nn(),
            0xC4 | 0xD4 | 0xCC | 0xDC => Self::append_call_cc(),
            _ => VecDeque::new()
        }
    }
}
