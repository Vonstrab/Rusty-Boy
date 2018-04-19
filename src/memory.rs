pub struct Memory {
    empty_zone: [u8; 0xF00 - 0xEA0],    //adress FF00 - FEA0
    sprite_attribute_memory: [u8; 160], //adress FEA0 -FE00
    internal_ram: [u8; 8 * 1024],       //adress E000 - C000
    switchable_ram: [u8; 8 * 1024],     //adress C000 - A000
    video_ram: [u8; 8 * 1024],          //adress A000 - 8000
    rom_bank_switch: [u8; 16 * 1024],   // adress 8000 - 4000
    rom_bank0: [u8; 16 * 1024],         // adress 4000 - 0000
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            empty_zone: [0; 0xF00 - 0xEA0],
            sprite_attribute_memory: [0; 160],
            internal_ram: [0; 8 * 1024],
            switchable_ram: [0; 8 * 1024],
            video_ram: [0; 8 * 1024],
            rom_bank_switch: [0; 16 * 1024],
            rom_bank0: [0; 16 * 1024],
        }
    }

    pub fn load_value8bit(&mut self, adress: u16, value: u8) {
        if adress < 0x4000 {
            self.rom_bank0[adress as usize] = value;
        }
        if (adress >= 0x4000 && adress < 0x8000) {
            self.rom_bank_switch[(adress - 0x4000) as usize] = value;
        }
        if (adress >= 0x8000 && adress < 0xA000) {
            self.video_ram[(adress - 0x8000) as usize] = value;
        }
        if (adress >= 0xA000 && adress < 0xC000) {
            self.switchable_ram[(adress - 0xA000) as usize] = value;
        }
        if (adress >= 0xC000 && adress < 0xE000) {
            self.internal_ram[(adress - 0xC000) as usize] = value;
        }
        if (adress >= 0xFE00 && adress < 0xFEA0) {
            self.sprite_attribute_memory[(adress - 0xFE00) as usize] = value;
        }
        if (adress >= 0xFEA0 && adress < 0xFF00) {
            self.empty_zone[(adress - 0xFEA0) as usize] = value;
        }
    }

    pub fn load_value16bit(&mut self, adress_hi: u16, adress_lo: u16, value: u16) {
        self.load_value8bit(adress_hi, (value >> 8) as u8);
        self.load_value8bit(adress_lo, value as u8);
    }
}
