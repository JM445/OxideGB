pub struct Cartridge {
    rom: Vec<Vec<u8>>, // Rom Banks
    ram: Vec<Vec<u8>>, // Ram Banks
    mbc: MbcKind,

    cur_rom: usize, // Current Rom Bank index
    cur_ram: usize, // Current Ram Bank index
    ram_enabled: bool,
}

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
    M161(u8),
    HUC1(u8),
    HUC3(u8),
    OTHER(u8)
}

impl Cartridge {
    pub fn from_file<P: AsRef<Path>>(path: P) -> <Self, String> {
        let raw = fs::read(path).map_err(|e| e.to_string())?;
    }
}
