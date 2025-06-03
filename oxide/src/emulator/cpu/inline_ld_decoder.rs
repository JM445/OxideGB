use super::*;

impl Cpu {
    #[inline]
    pub fn decode_ld_r16_imm16(dest: Reg16) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadLSB{prefetch: false},
            MicroOp::ReadMSB{prefetch: false},
            MicroOp::DataMove{source: RWTarget::Tmp16, dest: RWTarget::Reg16(dest), prefetch: true},
        ])
    }

    #[inline]
    pub fn decode_ld_indirect_r8(dest: RWTarget, source: RWTarget) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove{source, dest, prefetch: false},
            MicroOp::PrefetchOnly
        ])
    }

    #[inline]
    pub fn decode_ld_r_r(dest: Reg8, source: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove{source: RWTarget::Reg8(source), dest: RWTarget::Reg8(dest), prefetch: true}
        ])
    }

    #[inline]
    pub fn decode_ld_imm8(dest: Reg8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadIMM{prefetch: false},
            MicroOp::DataMove{source: RWTarget::Tmp8, dest: RWTarget::Reg8(dest), prefetch: true},
        ])
    }

    #[inline]
    pub fn decode_ld_indirect_imm8(dest: Reg16) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadIMM{prefetch: false},
            MicroOp::DataMove{source: RWTarget::Tmp8, dest: RWTarget::Indirect16(dest), prefetch: false},
            MicroOp::PrefetchOnly
        ])
    }

    #[inline]
    pub fn decode_ld_a16_a() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadLSB{prefetch: false},
            MicroOp::ReadMSB{prefetch: false},
            MicroOp::DataMove{source: RWTarget::Reg8(Reg8::A), dest: RWTarget::Addr, prefetch: false},
            MicroOp::PrefetchOnly
        ])
    }

    #[inline]
    pub fn decode_ld_a_a16() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadLSB{prefetch: false},
            MicroOp::ReadMSB{prefetch: false},
            MicroOp::DataMove{source: RWTarget::Addr, dest: RWTarget::Reg8(Reg8::A), prefetch: false},
            MicroOp::PrefetchOnly
        ])
    }

    #[inline]
    pub fn decode_ldh_imm_a() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadIMM{prefetch: false},
            MicroOp::DataMove{source: RWTarget::Tmp8, dest: RWTarget::Tmp8, prefetch: false},
            MicroOp::DataMove{source: RWTarget::Reg8(Reg8::A), dest: RWTarget::Addr, prefetch: true},
        ])
    }

    #[inline]
    pub fn decode_ldh_a_imm() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadIMM{prefetch: false},
            MicroOp::DataMove{source: RWTarget::Tmp8, dest: RWTarget::Tmp8, prefetch: false},
            MicroOp::DataMove{dest: RWTarget::Reg8(Reg8::A), source: RWTarget::Addr, prefetch: true},
        ])
    }

    #[inline]
    pub fn decode_ldh_c_a() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove{source: RWTarget::Reg8(Reg8::C), dest: RWTarget::Tmp8, prefetch: false},
            MicroOp::DataMove{source: RWTarget::Reg8(Reg8::A), dest: RWTarget::Addr, prefetch: true},
        ])
    }

    #[inline]
    pub fn decode_ldh_a_c() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove{source: RWTarget::Reg8(Reg8::C), dest: RWTarget::Tmp8, prefetch: false},
            MicroOp::DataMove{dest: RWTarget::Reg8(Reg8::A), source: RWTarget::Addr, prefetch: true},
        ])
    }

    #[inline]
    pub fn decode_ld_hl_sp_e8() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadIMM{prefetch: false},
            MicroOp::Operation { ope: Operation::Add {
                left: RWTarget::Reg8(Reg8::SPL), right: RWTarget::Tmp8,
                dest: RWTarget::Reg8(Reg8::L), mask: 0b1111
            } , prefetch: false },
            MicroOp::Operation { ope: Operation::Adc {
                left: RWTarget::Reg8(Reg8::SPH), right: RWTarget::Value(0),
                dest: RWTarget::Reg8(Reg8::H), mask: 0b1111
            }, prefetch: true },
        ])
    }

    #[inline]
    pub fn decode_ld_a16_sp() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadLSB { prefetch: false },
            MicroOp::ReadMSB { prefetch: false },
            MicroOp::DataMove{ source: RWTarget::Reg8(Reg8::SPL), dest: RWTarget::Indirect16I(Reg16::WZ), prefetch: false},
            MicroOp::DataMove{ source: RWTarget::Reg8(Reg8::SPH), dest: RWTarget::Indirect16(Reg16::WZ),  prefetch: false},
            MicroOp::PrefetchOnly
        ])
    }

    #[inline]
    pub fn decode_ld_sp_hl() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove {
                source: RWTarget::Reg8(Reg8::L), dest: RWTarget::Reg8(Reg8::SPL), prefetch: false
            },
            MicroOp::DataMove {
                source: RWTarget::Reg8(Reg8::H), dest: RWTarget::Reg8(Reg8::SPH), prefetch: true
            }
        ])
    }

    #[inline]
    pub fn decode_push(source: Reg16) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Operation {ope: Operation::Dec {
                source: RWTarget::Reg16(Reg16::SP), dest: RWTarget::Reg16(Reg16::SP), mask: 0b0000
            }, prefetch: false},
            MicroOp::DataMove {
                source: RWTarget::Reg8(source.msb()), dest: RWTarget::Indirect16D(Reg16::SP), prefetch: false
            },
            MicroOp::DataMove {
                source: RWTarget::Reg8(source.lsb()), dest: RWTarget::Indirect16(Reg16::SP), prefetch: false
            },
            MicroOp::PrefetchOnly
        ])
    }

    #[inline]
    pub fn decode_pop(dest: Reg16) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove {
                source: RWTarget::Indirect16I(Reg16::SP),
                dest: RWTarget::Reg8(dest.lsb()), prefetch: false
            },
            MicroOp::DataMove {
                source: RWTarget::Indirect16I(Reg16::SP),
                dest: RWTarget::Reg8(dest.msb()), prefetch: false
            },
            MicroOp::PrefetchOnly
        ])
    }

}
