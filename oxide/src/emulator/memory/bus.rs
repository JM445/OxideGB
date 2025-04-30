pub struct Bus {

}

impl Bus {
    pub fn read(&self, addr: u16) -> u8 {
        match addr{
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            _ => ()
        }
    }
}
