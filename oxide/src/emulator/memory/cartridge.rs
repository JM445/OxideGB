#[derive(debug)]
pub struct Cartridge {
    rom: Vec<Vec<u8>>, // Rom Banks
    ram: Vec<Vec<u8>>, // Ram Banks
    mbc: MbcKind,

    cur_rom: usize, // Current Rom Bank index
    cur_ram: usize, // Current Ram Bank index
    ram_enabled: bool,
}

#[derive(debug)]
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
            NO_MBC(val) => format!("No Mbc ({})", value),
            MBC1(val) => format!("MBC1 ({})", value),
            MBC2(val) => format!("MBC2 ({})", value),
            MBC3(val) => format!("MBC3 ({})", value),
            MBC4(val) => format!("MBC4 ({})", value),
            MBC5(val) => format!("MBC5 ({})", value),
            MBC6(val) => format!("MBC6 ({})", value),
            MBC7(val) => format!("MBC7 ({})", value),
            MMM01(val) => format!("MMM01 ({})", value),
            HUC1(val) => format!("HuC1 ({})", value),
            HUC3(val) => format!("HuC3 ({})", value),
            CAM(val) => format!("Camera ({})", value),
            TAMA(val) => format!("Tama5 ({})", value),
            _ => format!("Unknown Mbc")
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

impl Cartridge {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let raw = fs::read(path).map_err(|e| e.to_string())?;
        let kind = MbcKind::from_u8(raw[0x0147]);

        match kind {
            MbcKind::NO_MBC => Self::load_no_mbc(raw, kind),
            None => Err(format!("Unknown cartridge type value: {}", raw[0x0147])),
            _ => Err(format!("MBC Kind {} not yet implemented !", kind))
        }
    }

    fn load_no_mbc(raw: Vec<u8>, kind: MbcKind) -> Result<Self, String> {
        match (raw, kind) {
            (0x8000, MbcKind::NO_MBC(0x00)) => Ok(Self {

            })
        }
        rom = raw[0x0000..0x]
    }
}
