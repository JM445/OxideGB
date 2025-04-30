pub struct Cartridge {
    rom: Vec<Vec<u8>>, // Rom Banks
    ram: Vec<Vec<u8>>, // Ram Banks
    mbc: MbcKind,

    cur_rom: usize, // Current Rom Bank index
    cur_ram: usize, // Current Ram Bank index
    ram_enabled: bool,
}

impl Cartridge {
    pub fn from_file(path: string) {

    }
}
