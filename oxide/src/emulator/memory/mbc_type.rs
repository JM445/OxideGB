use std::fmt;

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum MbcKind {
    NO_MBC(u8),
    MBC1(u8),
    MBC2(u8),
    MBC3(u8),
    MBC4(u8),
    MBC5(u8),
    MBC6(u8),
    MBC7(u8),
    MMM01(u8),
    HUC1(u8),
    HUC3(u8),
    CAM(u8),
    TAMA(u8),
}

impl fmt::Display for MbcKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            MbcKind::NO_MBC(val) => format!("No Mbc ({})", val),
            MbcKind::MBC1(val) => format!("MBC1 ({})", val),
            MbcKind::MBC2(val) => format!("MBC2 ({})", val),
            MbcKind::MBC3(val) => format!("MBC3 ({})", val),
            MbcKind::MBC4(val) => format!("MBC4 ({})", val),
            MbcKind::MBC5(val) => format!("MBC5 ({})", val),
            MbcKind::MBC6(val) => format!("MBC6 ({})", val),
            MbcKind::MBC7(val) => format!("MBC7 ({})", val),
            MbcKind::MMM01(val) => format!("MMM01 ({})", val),
            MbcKind::HUC1(val) => format!("HuC1 ({})", val),
            MbcKind::HUC3(val) => format!("HuC3 ({})", val),
            MbcKind::CAM(val) => format!("Camera ({})", val),
            MbcKind::TAMA(val) => format!("Tama5 ({})", val),
        };

        write!(f, "{}", s)
    }
}

impl MbcKind {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x00 | 0x08 | 0x09 => Some(MbcKind::NO_MBC(value)),
            0x01 | 0x02 | 0x03 => Some(MbcKind::MBC1(value)),
            0x05 | 0x06        => Some(MbcKind::MBC2(value)),
            0x0B | 0x0C | 0x0D => Some(MbcKind::MMM01(value)),
            0x0F..=0x13        => Some(MbcKind::MBC3(value)),
            0x19..=0x1E        => Some(MbcKind::MBC5(value)),
            0x20               => Some(MbcKind::MBC6(value)),
            0x22               => Some(MbcKind::MBC7(value)),
            0xFE               => Some(MbcKind::HUC3(value)),
            0xFF               => Some(MbcKind::HUC1(value)),
            0xFC               => Some(MbcKind::CAM(value)),
            0xFD               => Some(MbcKind::TAMA(value)),
            _ => None
        }
    }
}
