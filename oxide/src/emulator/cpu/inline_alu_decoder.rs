use super::*;

impl Cpu {

    /******************** ADD/SUB ********************/

    #[inline]
    pub fn decode_add_r8(left: Reg8, right: Reg8, dest: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Operation { ope: Operation::Add{
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(right),
                dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true }
        ])
    }

    #[inline]
    pub fn decode_adc_r8(left: Reg8, right: Reg8, dest: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Operation{ ope: Operation::Adc {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(right),
                dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true }
        ])
    }

    #[inline]
    pub fn decode_sub_r8(left: Reg8, right: Reg8, dest: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Operation{ ope: Operation::Sub {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(right),
                dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true }
        ])
    }

    #[inline]
    pub fn decode_sbc_r8(left: Reg8, right: Reg8, dest: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Operation{ ope: Operation::Sbc {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(right),
                dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true }
        ])
    }

    #[inline]
    pub fn decode_add_indirect(left: Reg8, right: Reg16, dest: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove {source: RWTarget::Indirect16(right), dest: RWTarget::Reg8(Reg8::Z), prefetch: false},
            MicroOp::Operation { ope: Operation::Add {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(Reg8::Z),
                dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true }
        ])
    }

    #[inline]
    pub fn decode_sub_indirect(left: Reg8, right: Reg16, dest: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove {source: RWTarget::Indirect16(right), dest: RWTarget::Reg8(Reg8::Z), prefetch: false},
            MicroOp::Operation { ope: Operation::Sub {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(Reg8::Z),
                dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true }
        ])
    }

    #[inline]
    pub fn decode_adc_indirect(left: Reg8, right: Reg16, dest: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove {source: RWTarget::Indirect16(right), dest: RWTarget::Reg8(Reg8::Z), prefetch: false},
            MicroOp::Operation { ope: Operation::Adc {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(Reg8::Z),
                dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true }
        ])
    }

    #[inline]
    pub fn decode_sbc_indirect(left: Reg8, right: Reg16, dest: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove {source: RWTarget::Indirect16(right), dest: RWTarget::Reg8(Reg8::Z), prefetch: false},
            MicroOp::Operation { ope: Operation::Sbc {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(Reg8::Z),
                dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true }
        ])
    }

    #[inline]
    pub fn decode_add_imm(left: Reg8, dest: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadIMM {prefetch: false},
            MicroOp::Operation {ope: Operation::Add {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(Reg8::Z),
                dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true}
        ])
    }

    #[inline]
    pub fn decode_sub_imm(left: Reg8, dest: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadIMM {prefetch: false},
            MicroOp::Operation {ope: Operation::Sub {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(Reg8::Z),
                dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true}
        ])
    }

    #[inline]
    pub fn decode_adc_imm(left: Reg8, dest: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadIMM {prefetch: false},
            MicroOp::Operation {ope: Operation::Adc {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(Reg8::Z),
                dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true}
        ])
    }

    #[inline]
    pub fn decode_sbc_imm(left: Reg8, dest: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadIMM {prefetch: false},
            MicroOp::Operation {ope: Operation::Sbc {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(Reg8::Z),
                dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true}
        ])
    }

    /******************** COMPARE ********************/

    #[inline]
    pub fn decode_cp_r8(left: Reg8, right: Reg8) -> VecDeque<MicroOp>  {
        VecDeque::from(vec![
            MicroOp::Operation {ope: Operation::Sub {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(right),
                dest: RWTarget::Value(0), mask: 0b1111 // Dest is set to a value to discard the result
            }, prefetch: true}
        ])
    }

    #[inline]
    pub fn decode_cp_imm(left: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadIMM {prefetch: false},
            MicroOp::Operation {ope: Operation::Sub {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(Reg8::Z),
                dest: RWTarget::Value(0), mask: 0b1111 // Dest is set to a value to discard the result
            }, prefetch: true}
        ])
    }

    #[inline]
    pub fn decode_cp_indirect(left: Reg8, right: Reg16) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove {source: RWTarget::Indirect16(right), dest: RWTarget::Reg8(Reg8::Z), prefetch: false},
            MicroOp::Operation {ope: Operation::Sub {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(Reg8::Z),
                dest: RWTarget::Value(0), mask: 0b1111 // Dest is set to a value to discard the result
            }, prefetch: true}
        ])
    }

    /******************** INC/DEC ********************/

    #[inline]
    pub fn decode_inc_r8(target: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Operation {ope: Operation::Inc {
                source: RWTarget::Reg8(target), dest: RWTarget::Reg8(target),
                mask: 0b1110
            }, prefetch: true}
        ])
    }

    #[inline]
    pub fn decode_dec_r8(target: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Operation {ope: Operation::Dec {
                source: RWTarget::Reg8(target), dest: RWTarget::Reg8(target),
                mask: 0b1110
            }, prefetch: true}
        ])
    }

    #[inline]
    pub fn decode_inc_indirect(target: Reg16) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove {source: RWTarget::Indirect16(target), dest: RWTarget::Reg8(Reg8::Z), prefetch: false},
            MicroOp::Operation {ope: Operation::Inc {
                source: RWTarget::Reg8(Reg8::Z), dest: RWTarget::Indirect16(target),
                mask: 0b1110
            }, prefetch: false},
            MicroOp::PrefetchOnly
        ])
    }

    #[inline]
    pub fn decode_dec_indirect(target: Reg16) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove {source: RWTarget::Indirect16(target), dest: RWTarget::Reg8(Reg8::Z), prefetch: false},
            MicroOp::Operation {ope: Operation::Dec {
                source: RWTarget::Reg8(Reg8::Z), dest: RWTarget::Indirect16(target),
                mask: 0b1110
            }, prefetch: false},
            MicroOp::PrefetchOnly
        ])
    }

    /******************** LOGIC ********************/

    #[inline]
    pub fn decode_and_r8(left: Reg8, right: Reg8, dest: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Operation {ope: Operation::And {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(right),
                dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true}
        ])
    }

    #[inline]
    pub fn decode_or_r8(left: Reg8, right: Reg8, dest: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Operation {ope: Operation::Or {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(right),
                dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true}
        ])
    }

    #[inline]
    pub fn decode_xor_r8(left: Reg8, right: Reg8, dest: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Operation {ope: Operation::Xor {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(right),
                dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true}
        ])
    }

    #[inline]
    pub fn decode_and_indirect(left: Reg8, right: Reg16, dest: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove { source: RWTarget::Indirect16(right), dest: RWTarget::Reg8(Reg8::Z), prefetch: false},
            MicroOp::Operation { ope: Operation::And {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(Reg8::Z),
                dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true }
        ])
    }

    #[inline]
    pub fn decode_or_indirect(left: Reg8, right: Reg16, dest: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove { source: RWTarget::Indirect16(right), dest: RWTarget::Reg8(Reg8::Z), prefetch: false},
            MicroOp::Operation { ope: Operation::Or {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(Reg8::Z),
                dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true }
        ])
    }

    #[inline]
    pub fn decode_xor_indirect(left: Reg8, right: Reg16, dest: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove { source: RWTarget::Indirect16(right), dest: RWTarget::Reg8(Reg8::Z), prefetch: false},
            MicroOp::Operation { ope: Operation::Xor {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(Reg8::Z),
                dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true }
        ])
    }

    #[inline]
    pub fn decode_and_imm(left: Reg8, dest: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadIMM {prefetch: false},
            MicroOp::Operation {ope: Operation::And {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(Reg8::Z),
                dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true}
        ])
    }

    #[inline]
    pub fn decode_or_imm(left: Reg8, dest: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadIMM {prefetch: false},
            MicroOp::Operation {ope: Operation::Or {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(Reg8::Z),
                dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true}
        ])
    }

    #[inline]
    pub fn decode_xor_imm(left: Reg8, dest: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadIMM {prefetch: false},
            MicroOp::Operation {ope: Operation::Xor {
                left: RWTarget::Reg8(left), right: RWTarget::Reg8(Reg8::Z),
                dest: RWTarget::Reg8(dest), mask: 0b1111
            }, prefetch: true}
        ])
    }

    /******************** 16B Operations ********************/
    #[inline]
    pub fn decode_add_r16(left: Reg16, right: Reg16, dest: Reg16) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Operation {ope: Operation::Add {
                left: RWTarget::Reg8(left.lsb()), right: RWTarget::Reg8(right.lsb()),
                dest: RWTarget::Reg8(dest.lsb()), mask: 0b0111
            }, prefetch: false},
            MicroOp::Operation {ope: Operation::Adc {
                left: RWTarget::Reg8(left.msb()), right: RWTarget::Reg8(right.msb()),
                dest: RWTarget::Reg8(dest.msb()), mask: 0b0111
            }, prefetch: true}
        ])
    }

    #[inline]
    pub fn decode_add_sp_e8() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadIMM {prefetch: false},
            MicroOp::Operation {ope: Operation::Ads {
                left: RWTarget::Reg8(Reg8::SPL), right: RWTarget::Reg8(Reg8::Z),
                dest: RWTarget::Reg8(Reg8::Z), mask: 0b1111
            }, prefetch: false},
            MicroOp::Operation {ope: Operation::Adc{
                left: RWTarget::Reg8(Reg8::SPH), right: RWTarget::Value(0),
                dest: RWTarget::Reg8(Reg8::W), mask: 0b0000
            }, prefetch: false},
            MicroOp::DataMove {source: RWTarget::Reg16(Reg16::WZ), dest: RWTarget::Reg16(Reg16::SP), prefetch: true}
        ])
    }

    #[inline]
    pub fn decode_inc_r16(target: Reg16) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Operation {ope: Operation::Inc {
                source: RWTarget::Reg16(target), dest: RWTarget::Reg16(target),
                mask: 0b0000
            }, prefetch: false},
            MicroOp::PrefetchOnly
        ])
    }

    #[inline]
    pub fn decode_dec_r16(target: Reg16) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Operation {ope: Operation::Dec {
                source: RWTarget::Reg16(target), dest: RWTarget::Reg16(target),
                mask: 0b0000
            }, prefetch: false},
            MicroOp::PrefetchOnly
        ])
    }
}
