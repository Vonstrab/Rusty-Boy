pub struct Memory {
    empty_zone : [u8;0x80-0x4c] //adress FE80 - FEA0 
    sprite_attribute_memory : [u8;160] //adress FEA0 -FE00
    internal_ram: [u8; 8 * 1024],     //adress E000 - A000
    switchable_ram: [u8; 8 * 1024],   //adress C000 - A000
    video_ram: [u8; 8 * 1024],        //adress A000 - 8000
    rom_bank_switch: [u8; 16 * 1024], // adress 8000 - 4000
    rom_bank0: [u8; 16 * 1024],       // adress 4000 - 0000
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
