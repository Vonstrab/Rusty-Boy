pub struct Memory {
    ram: [u8; 0xFFFF],
}

impl Memory {
    pub fn new() -> Memory {
        Memory { ram: [0; 0xFFFF] }
    }

    pub fn load_value8bit(&mut self, adress: u16, value: u8) {
        self.ram[adress as usize] = value;
    }

    pub fn load_value16bit(&mut self, adress_hi: u16, adress_lo: u16, value: u16) {
        self.ram[adress_hi as usize] = (value >> 8) as u8;
        self.ram[adress_lo as usize] = value as u8;
    }
}
