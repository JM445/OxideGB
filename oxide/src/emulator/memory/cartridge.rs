mod mbc_type;

use mbc_type::*;

#[derive(debug)]
pub struct Cartridge {
    raw_rom: Vec<u8>,
    rom_banks: Vec<&[u8]>, // Rom Banks
    ram_banks: Vec<Vec<u8>>, // Ram Banks
    mbc: MbcKind,

    cur_rom: usize, // Current Rom Bank index
    cur_ram: usize, // Current Ram Bank index
    ram_enabled: bool,
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
        let ram_bank = match kind {
            MbcKind::NO_MBC(value) if value == 0x00 => Vec::new(),
            MbcKind::NO_MBC(value) if value == 0x08 => vec![vec![0, 0x2000], 1],
            MbcKind::NO_MBC(value) if value == 0x09 => vec![vec![0, 0x2000], 1],
            MbcKind::NO_MBC(value) => Err(format!("MBC Type error. 0x{:X} is not a valid NO_MBC value.", value)),
            _ => Err("Should be unreachable".to_string())
        }?;

        Ok(Self {
            raw_rom: raw,
            rom_banks: raw.chunks(0x8000).collect(),
            ram_banks: ram_bank,
            mbc: kind,

            cur_rom: 0,
            cur_ram: 0,
            ram_enabled: if ram_bank.len() { true } else { false }
        })
    }
}
