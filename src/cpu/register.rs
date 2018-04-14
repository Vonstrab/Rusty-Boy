#[derive(Copy, Clone)]
pub struct Register {
    pub A: u8,
    f: u8,
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
    fn new() -> Register {
        Register {
            A: Ox00,
            F: Ox00,
            B: Ox00,
            C: Ox00,
            D: Ox00,
            E: Ox00,
            H: Ox00,
            L: Ox00,
            PC: Ox0100,
            SP: OxFFFE,
        }
    }

    fn getAF(&self) -> u16 {
        ((&self.A as u16)<<8)) | (&self.F as u16)
    }

    
}
