pub struct Memory {
    internal_ram2: [u8; 0xFF - 0x80],   // adress FFFF - FF80
    empty_zone2: [u8; 0xF80 - 0xE4C],   // adress FF80 - FF4C
    io_zone: [u8; 0xF4C - 0xF00],       // adress FF4C - FF00
    empty_zone1: [u8; 0xF00 - 0xEA0],   // adress FF00 - FEA0
    sprite_attribute_memory: [u8; 160], // adress FEA0 - FE00
    internal_ram1: [u8; 8 * 1024],      // adress E000 - C000
    switchable_ram: [u8; 8 * 1024],     // adress C000 - A000
    video_ram: [u8; 8 * 1024],          // adress A000 - 8000
    rom_bank_switch: [u8; 16 * 1024],   // adress 8000 - 4000
    rom_bank0: [u8; 16 * 1024],         // adress 4000 - 0000
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            internal_ram2: [0; 0xFF - 0x80],
            empty_zone2: [0; 0xF80 - 0xE4C],
            io_zone: [0; 0xF4C - 0xF00],
            empty_zone1: [0; 0xF00 - 0xEA0],
            sprite_attribute_memory: [0; 160],
            internal_ram1: [0; 8 * 1024],
            switchable_ram: [0; 8 * 1024],
            video_ram: [0; 8 * 1024],
            rom_bank_switch: [0; 16 * 1024],
            rom_bank0: [0; 16 * 1024],
        }
    }

    pub fn load_value8bit(&self, adress: u16) -> u8 {
        if adress < 0x4000 {
            return self.rom_bank0[adress as usize];
        } else if adress >= 0x4000 && adress < 0x8000 {
            return self.rom_bank_switch[(adress - 0x4000) as usize];
        } else if adress >= 0x8000 && adress < 0xA000 {
            return self.video_ram[(adress - 0x8000) as usize];
        } else if adress >= 0xA000 && adress < 0xC000 {
            return self.switchable_ram[(adress - 0xA000) as usize];
        } else if adress >= 0xC000 && adress < 0xE000 {
            return self.internal_ram1[(adress - 0xC000) as usize];
        } else if adress >= 0xE000 && adress < 0xFE00 {
            return self.internal_ram1[(adress - 0xE000) as usize];
        } else if adress >= 0xFE00 && adress < 0xFEA0 {
            return self.sprite_attribute_memory[(adress - 0xFE00) as usize];
        } else if adress >= 0xFEA0 && adress < 0xFF00 {
            return self.empty_zone1[(adress - 0xFEA0) as usize];
        } else if adress >= 0xFF00 && adress < 0xFF4C {
            return self.io_zone[(adress - 0xFF00) as usize];
        } else if adress >= 0xFF4C && adress < 0xFF80 {
            return self.empty_zone2[(adress - 0xFF4C) as usize];
        } else {
            return self.internal_ram2[(adress - 0xFF80) as usize];
        }
    }

    pub fn load_value16bit(&mut self, adress_hi: u16, adress_lo: u16) -> u16 {
        return ((self.load_value8bit(adress_hi) as u16) << 8)
            | (self.load_value8bit(adress_lo) as u16);
    }

    pub fn store_value8bit(&mut self, adress: u16, value: u8) {
        if adress < 0x4000 {
            self.rom_bank0[adress as usize] = value;
        } else if adress >= 0x4000 && adress < 0x8000 {
            self.rom_bank_switch[(adress - 0x4000) as usize] = value;
        } else if adress >= 0x8000 && adress < 0xA000 {
            self.video_ram[(adress - 0x8000) as usize] = value;
        } else if adress >= 0xA000 && adress < 0xC000 {
            self.switchable_ram[(adress - 0xA000) as usize] = value;
        } else if adress >= 0xC000 && adress < 0xE000 {
            self.internal_ram1[(adress - 0xC000) as usize] = value;
        } else if adress >= 0xE000 && adress < 0xFE00 {
            self.internal_ram1[(adress - 0xE000) as usize] = value;
        } else if adress >= 0xFE00 && adress < 0xFEA0 {
            self.sprite_attribute_memory[(adress - 0xFE00) as usize] = value;
        } else if adress >= 0xFEA0 && adress < 0xFF00 {
            self.empty_zone1[(adress - 0xFEA0) as usize] = value;
        } else if adress >= 0xFF00 && adress < 0xFF4C {
            self.io_zone[(adress - 0xFF00) as usize] = value;
        } else if adress >= 0xFF4C && adress < 0xFF80 {
            self.empty_zone2[(adress - 0xFF4C) as usize] = value;
        } else {
            self.internal_ram2[(adress - 0xFF80) as usize] = value;
        }
        ();
    }

    pub fn store_value16bit(&mut self, adress_hi: u16, adress_lo: u16, value: u16) {
        self.store_value8bit(adress_hi, (value >> 8) as u8);
        self.store_value8bit(adress_lo, value as u8);
        ();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn testLoadStore() {
        let mut mem = Memory::new();
        mem.store_value8bit(0x9000, 0xAB);
        assert_eq!(mem.load_value8bit(0x9000), 0xAB);
        mem.store_value16bit(0x9005, 0x9004, 0xCDF1);
        assert_eq!(mem.load_value16bit(0x9005, 0x9004), 0xCDF1);
    }

    #[test]
    fn testEcho() {
        let mut mem = Memory::new();
        mem.store_value8bit(0xC000, 0xAB);
        assert_eq!(mem.load_value8bit(0xE000), 0xAB);
        mem.store_value8bit(0xE004, 0xBA);
        assert_eq!(mem.load_value8bit(0xE004), 0xBA);
    }
}
