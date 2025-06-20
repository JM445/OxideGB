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
                source: RWTarget::Reg8(Reg8::PCL), dest: RWTarget::Indirect16(Reg16::SP), prefetch: false
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
                source: RWTarget::Reg8(Reg8::PCH), dest: RWTarget::Indirect16D(Reg16::SP), prefetch: false
            },
            MicroOp::DataMove {
                source: RWTarget::Reg8(Reg8::PCL), dest: RWTarget::Indirect16(Reg16::SP), prefetch: false
            },
            MicroOp::DataMove {
                source: RWTarget::Reg16(Reg16::WZ), dest: RWTarget::Reg16(Reg16::PC), prefetch: true
            }
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

    /******************** JUMPS ********************/

    #[inline]
    pub fn decode_jp_nn() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadLSB{prefetch: false},
            MicroOp::ReadMSB{prefetch: false},
            MicroOp::DataMove {
                source: RWTarget::Reg16(Reg16::WZ), dest: RWTarget::Reg16(Reg16::PC), prefetch: false
            },
            MicroOp::PrefetchOnly
        ])
    }

    #[inline]
    pub fn decode_jp_hl() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove {
                source: RWTarget::Reg16(Reg16::HL), dest: RWTarget::Reg16(Reg16::PC), prefetch: true
            }
        ])
    }

    #[inline]
    pub fn decode_jp_cc_nn(cc: Condition) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadLSB { prefetch: false },
            MicroOp::ReadMSBCC { cc },
        ])
    }

    #[inline]
    pub fn append_jp_cc_nn() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove{
                source: RWTarget::Reg16(Reg16::WZ), dest: RWTarget::Reg16(Reg16::PC), prefetch: false,
            },
            MicroOp::PrefetchOnly
        ])
    }

    #[inline]
    pub fn decode_jr() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadIMM{prefetch: false},
            MicroOp::Operation {ope: Operation::Ads {
                left: RWTarget::Reg16(Reg16::PC), right: RWTarget::Reg8(Reg8::Z),
                dest: RWTarget::Reg16(Reg16::WZ), mask: 0b0000
            }, prefetch: false},
            MicroOp::DataMove{
                source: RWTarget::Reg16(Reg16::WZ), dest: RWTarget::Reg16(Reg16::PC), prefetch: true
            }
        ])
    }

    #[inline]
    pub fn decode_jr_cc(cc: Condition) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ReadLSBCC {cc}
        ])
    }

    #[inline]
    pub fn append_jr_cc() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Operation {ope: Operation::Ads {
                left: RWTarget::Reg16(Reg16::PC), right: RWTarget::Reg8(Reg8::Z),
                dest: RWTarget::Reg16(Reg16::WZ), mask: 0b0000
            }, prefetch: false},
            MicroOp::DataMove {source: RWTarget::Reg16(Reg16::WZ), dest: RWTarget::Reg16(Reg16::PC), prefetch: true}
        ])
    }

    /******************** JUMPS ********************/

    #[inline]
    pub fn decode_rst(addr: u8) -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Operation{ope: Operation::Dec {
                source: RWTarget::Reg16(Reg16::SP), dest: RWTarget::Reg16(Reg16::SP), mask: 0b0000
            }, prefetch: false},
            MicroOp::DataMove {
                source: RWTarget::Reg8(Reg8::PCH), dest: RWTarget::Indirect16D(Reg16::SP), prefetch: false
            },
            MicroOp::DataMove {
                source: RWTarget::Reg8(Reg8::PCL), dest: RWTarget::Indirect16(Reg16::SP), prefetch: false
            },
            MicroOp::DataMove {
                source: RWTarget::Value(addr as u16), dest: RWTarget::Reg16(Reg16::PC), prefetch: true
            }
        ])
    }
}
