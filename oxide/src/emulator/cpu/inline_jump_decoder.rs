use super::*;

impl Cpu {

    /******************** CALL ********************/

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
            MicroOp::ReadLSB {prefetch: false},
            MicroOp::ReadMSBCC {cc},
        ])
    }

    #[inline]
    pub fn append_call_cc() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Operation{ope: Operation::Dec {
                source: RWTarget::Reg16(Reg16::SP), dest: RWTarget::Reg16(Reg16::SP), mask: 0b0000
            }, prefetch: false},
            MicroOp::DataMove {
                source: RWTarget::Indirect16D(Reg16::SP), dest: RWTarget::Reg8(Reg8::PCH), prefetch: false
            },
            MicroOp::DataMove {
                source: RWTarget::Indirect16(Reg16::SP), dest: RWTarget::Reg8(Reg8::PCL), prefetch: false
            },
            MicroOp::PrefetchOnly
        ])
    }

    /******************** RET ********************/

    #[inline]
    pub fn decode_ret() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove{
                source: RWTarget::Indirect16I(Reg16::SP), dest: RWTarget::Reg8(Reg8::Z), prefetch: false
            },
            MicroOp::DataMove {
                source: RWTarget::Indirect16I(Reg16::SP), dest: RWTarget::Reg8(Reg8::W), prefetch: false
            },
            MicroOp::DataMove {
                source: RWTarget::Reg16(Reg16::WZ), dest: RWTarget::Reg16(Reg16::PC), prefetch: false
            },
            MicroOp::PrefetchOnly
        ])
    }

    // Use the decode_ret() function to fill the conditional deque
    #[inline]
    pub fn decode_ret_cc(cc: Condition) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::CheckCC { cc }
        ])
    }

    #[inline]
    pub fn decode_reti() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove {
                source: RWTarget::Indirect16I(Reg16::SP), dest: RWTarget::Reg8(Reg8::Z), prefetch: false
            },
            MicroOp::DataMove {
                source: RWTarget::Indirect16I(Reg16::SP), dest: RWTarget::Reg8(Reg8::W), prefetch: false
            },
            MicroOp::RetI,
            MicroOp::PrefetchOnly
        ])
    }
}
