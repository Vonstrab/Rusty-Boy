pub struct Memory {
    mem: [u8; 0xFFFF + 1],
    /* internal_ram2: [u8; 0xFF - 0x80],   // adress FFFF - FF80
    empty_zone2: [u8; 0xF80 - 0xE4C],   // adress FF80 - FF4C
    io_zone: [u8; 0xF4C - 0xF00],       // adress FF4C - FF00
    empty_zone1: [u8; 0xF00 - 0xEA0],   // adress FF00 - FEA0
    sprite_attribute_memory: [u8; 160], // adress FEA0 - FE00
    internal_ram1: [u8; 8 * 1024],      // adress E000 - C000
    switchable_ram: [u8; 8 * 1024],     // adress C000 - A000
    video_ram: [u8; 8 * 1024],          // adress A000 - 8000
    rom_bank_switch: [u8; 16 * 1024],   // adress 8000 - 4000
    rom_bank0: [u8; 16 * 1024],         // adress 4000 - 0000
 */
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: [0; 0xFFFF + 1],
            /* internal_ram2: [0; 0xFF - 0x80],
            empty_zone2: [0; 0xF80 - 0xE4C],
            io_zone: [0; 0xF4C - 0xF00],
            empty_zone1: [0; 0xF00 - 0xEA0],
            sprite_attribute_memory: [0; 160],
            internal_ram1: [0; 8 * 1024],
            switchable_ram: [0; 8 * 1024],
            video_ram: [0; 8 * 1024],
            rom_bank_switch: [0; 16 * 1024],
            rom_bank0: [0; 16 * 1024], */
        }
    }

    pub fn get_value8bit(&self, adress: u16) -> u8 {
        self.mem[adress as usize]
    }

    pub fn get_value16bit(&mut self, adress_hi: u16, adress_lo: u16) -> u16 {
        ((self.get_value8bit(adress_hi) as u16) << 8) | (self.get_value8bit(adress_lo) as u16)
    }

    pub fn set_value8bit(&mut self, adress: u16, value: u8) {
        self.mem[adress as usize] = value;
    }

    pub fn set_value16bit(&mut self, adress_hi: u16, adress_lo: u16, value: u16) {
        self.set_value8bit(adress_hi, (value >> 8) as u8);
        self.set_value8bit(adress_lo, value as u8);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_set() {
        let mut mem = Memory::new();
        mem.set_value8bit(0x9000, 0xAB);
        assert_eq!(mem.get_value8bit(0x9000), 0xAB);
        mem.set_value16bit(0x9005, 0x9004, 0xCDF1);
        assert_eq!(mem.get_value16bit(0x9005, 0x9004), 0xCDF1);
    }
}
