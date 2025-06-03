use super::*;

impl Cpu {
    #[inline]
    pub fn decode_call() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadLSB {prefetch: false},
            MicroOp::ReadMSB {prefetch: false},
            MicroOp::Operation {ope: Operation::Dec {
                source: RWTarget::Reg16(Reg16::SP), dest: RWTarget::Reg16(Reg16::SP),
                mask: 0b0000
            }, prefetch: false},
            MicroOp::DataMove {
                source: RWTarget::Reg8(Reg8::PCH), dest: RWTarget::Indirect16D(Reg16::SP), prefetch: false
            },
            MicroOp::DataMove {
                source: RWTarget::Reg8(Reg8::PCH), dest: RWTarget::Indirect16(Reg16::SP), prefetch: false
            },
            MicroOp::DataMove {
                source: RWTarget::Reg16(Reg16::WZ), dest: RWTarget::Reg16(Reg16::PC), prefetch: true
            }
        ])
    }

    #[inline]
    pub fn decode_call_cc(cc: Condition) -> VecDeque<MicroOp> {
        VecDeque::from(vec![

        ])
    }
}
