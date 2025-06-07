use super::*;

impl Cpu {
    /******************** ROTATIONS / SHIFTS ********************/
    #[inline]
    pub fn decode_rl_r8(dest: Reg8, shift: ShiftType) -> VecDeque<MicroOp>{
        VecDeque::from(vec![
            MicroOp::Operation { ope: Operation::Lsh {
                shift, source: RWTarget::Reg8(dest), dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true}
        ])
    }

    #[inline]
    pub fn decode_rr_r8(dest: Reg8, shift: ShiftType) -> VecDeque<MicroOp>{
        VecDeque::from(vec![
            MicroOp::Operation { ope: Operation::Rsh {
                shift, source: RWTarget::Reg8(dest), dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true}
        ])
    }

    #[inline]
    pub fn decode_rl_indirect(shift: ShiftType) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove {
                source: RWTarget::Indirect16(Reg16::HL), dest: RWTarget::Reg8(Reg8::Z), prefetch: false
            },
            MicroOp::Operation { ope: Operation::Lsh {
                shift, source: RWTarget::Reg8(Reg8::Z), dest: RWTarget::Indirect16(Reg16::HL), mask: 0b1111
            }, prefetch: false},
            MicroOp::PrefetchOnly
        ])
    }

    #[inline]
    pub fn decode_rr_indirect(shift: ShiftType) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove {
                source: RWTarget::Indirect16(Reg16::HL), dest: RWTarget::Reg8(Reg8::Z), prefetch: false
            },
            MicroOp::Operation { ope: Operation::Rsh {
                shift, source: RWTarget::Reg8(Reg8::Z), dest: RWTarget::Indirect16(Reg16::HL), mask: 0b1111
            }, prefetch: false},
            MicroOp::PrefetchOnly
        ])
    }

    /******************** SWAP ********************/

    #[inline]
    pub fn decode_swap_rr(reg: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Operation { ope: Operation::Swp {
                source: RWTarget::Reg8(reg), dest: RWTarget::Reg8(reg), mask: 0b1111
            }, prefetch: true}
        ])
    }

    #[inline]
    pub fn decode_swap_indirect() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove {
                source: RWTarget::Indirect16(Reg16::HL), dest: RWTarget::Reg8(Reg8::Z), prefetch: false
            },
            MicroOp::Operation { ope: Operation::Swp {
                source: RWTarget::Reg8(Reg8::Z), dest: RWTarget::Indirect16(Reg16::HL), mask: 0b1111
            }, prefetch: false},
            MicroOp::PrefetchOnly
        ])
    }

    /******************** BITS ********************/

    #[inline]
    pub fn decode_bit_rr(reg: Reg8, bit: u8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Operation { ope: Operation::Bit { source: RWTarget::Reg8(reg), bit, mask: 0b1110 }, prefetch: true}
        ])
    }

    #[inline]
    pub fn decode_bit_indirect(bit: u8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove {
                source: RWTarget::Indirect16(Reg16::HL), dest: RWTarget::Reg8(Reg8::Z), prefetch: false
            },
            MicroOp::Operation { ope: Operation::Bit {
                source: RWTarget::Reg8(Reg8::Z), bit, mask: 0b1110
            }, prefetch: true}
        ])
    }

    #[inline]
    pub fn decode_rsb_rr(reg: Reg8, bit: u8, value: u8) -> VecDeque<MicroOp>{
        VecDeque::from(vec![
            MicroOp::Operation { ope: Operation::Rsb {
                source: RWTarget::Reg8(reg), dest: RWTarget::Reg8(reg), bit, value
            }, prefetch: true}
        ])
    }

    #[inline]
    pub fn decode_rsb_indirect(bit: u8, value: u8) -> VecDeque<MicroOp>{
        VecDeque::from(vec![
            MicroOp::DataMove {
                source: RWTarget::Indirect16(Reg16::HL), dest: RWTarget::Reg8(Reg8::Z), prefetch: false
            },
            MicroOp::Operation { ope: Operation::Rsb {
                source: RWTarget::Reg8(Reg8::Z), dest: RWTarget::Indirect16(Reg16::HL), bit, value
            }, prefetch: false},
            MicroOp::PrefetchOnly
        ])
    }

}
