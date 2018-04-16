#[derive(Copy, Clone)]
pub struct Register {
    pub A: u8,
    F: u8,
    pub B: u8,
    pub C: u8,
    pub D: u8,
    pub E: u8,
    pub H: u8,
    pub L: u8,
    pub PC: u16,
    pub SP: u16,
}

#[derive(Copy, Clone)]
pub enum CpuFlag {
    C = 0b00010000,
    H = 0b00100000,
    N = 0b01000000,
    Z = 0b10000000,
}

impl Register {
    pub fn new() -> Register {
        Register {
            A: 0x00,
            F: 0xB0,
            B: 0x00,
            C: 0x00,
            D: 0x00,
            E: 0x00,
            H: 0x00,
            L: 0x00,
            PC: 0x0100,
            SP: 0xFFFE,
        }
    }

    pub fn getaf(&self) -> u16 {
        ((self.A as u16) << 8) | (self.F as u16)
    }

    pub fn getbc(&self) -> u16 {
        ((self.B as u16) << 8) | (self.C as u16)
    }

    pub fn getde(&self) -> u16 {
        ((self.D as u16) << 8) | (self.E as u16)
    }

    pub fn gethl(&self) -> u16 {
        ((self.H as u16) << 8) | (self.L as u16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registers() {
        let mut reg = Register::new();
        reg.A = 0x01;
        reg.B = 0x02;
        reg.C = 0x03;
        reg.D = 0x04;
        reg.E = 0x05;
        reg.H = 0x06;
        reg.L = 0x07;

        assert_eq!(reg.getaf(), 0x01B0);
        assert_eq!(reg.getbc(), 0x0203);
        assert_eq!(reg.getde(), 0x0405);
        assert_eq!(reg.gethl(), 0x0607);
    }
}
