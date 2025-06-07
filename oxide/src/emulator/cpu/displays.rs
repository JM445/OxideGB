use super::*;
use std::fmt;

impl fmt::Display for MicroOp {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            MicroOp::DataMove{source, dest, prefetch} => format!("DataMove({source} => {dest}. {prefetch})"),
            MicroOp::Operation{ope, prefetch} => format!("Operation({ope}. {prefetch})"),
            MicroOp::ReadIMM{prefetch} => format!("ReadImm({prefetch})"),
            MicroOp::ReadLSB{prefetch} => format!("ReadLSB({prefetch})"),
            MicroOp::ReadMSB{prefetch} => format!("ReadMSB({prefetch})"),
            MicroOp::ReadMSBCC { cc } => format!("ReadMSBCC({cc})"),
            MicroOp::CheckCC { cc } => format!("CheckCC({cc})"),
            MicroOp::RetI => format!("RETI"),
            MicroOp::PrefetchOnly => format!("PrefetchOnly"),
        };

        write!(f, "{}", s)
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Condition::Z => format!("Z == 1"),
            Condition::C => format!("C == 1"),
            Condition::NZ => format!("Z == 0"),
            Condition::NC => format!("N == 0"),
        };

        write!(f, "{}", s)
    }
}

impl fmt::Display for RWTarget {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            RWTarget::Reg8(trg) => format!("{trg}"),
            RWTarget::Reg16(trg) => format!("{trg}"),
            RWTarget::Addr => format!("[WZ]"),
            RWTarget::Indirect16(trg) => format!("[{trg}]"),
            RWTarget::Indirect8(trg) => format!("[{trg}]"),
            RWTarget::Tmp8 => format!("TMP8"),
            RWTarget::Tmp16 => format!("TMP16"),
            RWTarget::Value(v) => format!("{:#04X}", v),
            RWTarget::Indirect16D(trg) => format!("[{trg}-]"),
            RWTarget::Indirect16I(trg) => format!("[{trg}+]"),
            RWTarget::IME => format!("IME"),
        };

        write!(f, "{}", s)
    }
}

impl fmt::Display for Reg8 {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Reg8::A => "A",
            Reg8::F => "F",
            Reg8::B => "B",
            Reg8::C => "C",
            Reg8::D => "D",
            Reg8::E => "E",
            Reg8::H => "H",
            Reg8::L => "L",
            Reg8::W => "W",
            Reg8::Z => "Z",
            Reg8::PCH => "PCh",
            Reg8::PCL => "PCl",
            Reg8::SPH => "SPh",
            Reg8::SPL => "SPl"
        };

        write!(f, "{}", s)
    }
}

impl fmt::Display for Reg16 {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Reg16::AF => "AF",
            Reg16::BC => "BC",
            Reg16::DE => "DE",
            Reg16::HL => "HL",
            Reg16::SP => "SP",
            Reg16::PC => "PC",
            Reg16::WZ => "WZ",
        };

        write!(f, "{}", s)
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Operation::Add{left, right, dest, ..} => format!("({left} + {right}) => {dest}"),
            Operation::Sub{left, right, dest, ..} => format!("({left} - {right}) => {dest}"),
            Operation::Inc{dest, ..} => format!("{dest}++"),
            Operation::Dec{dest, ..} => format!("{dest}--"),
            Operation::Adc{left, right, dest, ..} => format!("({left} + {right} + C) => {dest}"),
            Operation::Sbc{left, right, dest, ..} => format!("({left} - {right} - C) => {dest}"),
            Operation::Ads{left, right, dest, ..} => format!("({left} + Signed({right})) => {dest}"),
            Operation::And{left, right, dest, ..} => format!("({left} & {right}) => {dest}"),
            Operation::Or {left, right, dest, ..} => format!("({left} | {right}) => {dest}"),
            Operation::Xor{left, right, dest, ..} => format!("({left} ^ {right}) => {dest}"),
        };

        write!(f, "{}", s)
    }
}
