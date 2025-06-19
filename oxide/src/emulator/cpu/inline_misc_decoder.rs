use super::*;

use log::error;

impl Cpu {
    #[inline]
    pub fn decode_noop() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::PrefetchOnly
        ])
    }

    #[inline]
    pub fn decode_di() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::DataMove{
                source: RWTarget::Value(0), dest: RWTarget::IME, prefetch: true
            }
        ])
    }

    #[inline]
    pub fn decode_ei() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::ScheduleEI
        ])
    }

    #[inline]
    pub fn decode_halt() -> VecDeque<MicroOp> {
        error!("Error: Decoding an unimplemented instruction: HALT");
        Self::decode_noop()
    }

    #[inline]
    pub fn decode_stop() -> VecDeque<MicroOp> {
        error!("Error: Decoding an unimplemented instruction: STOP");
        Self::decode_noop()
    }

    #[inline]
    pub fn decode_daa() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Daa
        ])
    }

    #[inline]
    pub fn decode_cpl() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Cpl
        ])
    }

    #[inline]
    pub fn decode_ccf() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Ccf
        ])
    }

    #[inline]
    pub fn decode_scf() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Scf
        ])
    }

    #[inline]
    pub fn decode_prefix() -> VecDeque<MicroOp> {
        VecDeque::from(vec![
            MicroOp::Prefix
        ])
    }
    
    #[inline]
    pub fn decode_invalid(opcode: u8) -> VecDeque<MicroOp> {
        error!("Error: Decoded an invalid opcode: {opcode:#04X}. Using NOOP instead.");
        VecDeque::from(vec![
            MicroOp::PrefetchOnly
        ])
    }
}
