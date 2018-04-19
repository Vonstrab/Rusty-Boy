#[derive(Copy, Clone)]
pub struct Register {
    pub a: u8,
    f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
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
            a: 0x00,
            f: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            h: 0x00,
            l: 0x00,
            pc: 0x0100,
            sp: 0xFFFE,
        }
    }

    pub fn after_reset() -> Register {
        Register {
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            pc: 0x0100,
            sp: 0xFFFE,
        }
    }

    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_af(&mut self, af_value: u16) {
        self.a = (af_value >> 8) as u8;;
        self.f = (af_value) as u8;
    }

    pub fn set_bc(&mut self, bc_value: u16) {
        self.b = (bc_value >> 8) as u8;;
        self.c = (bc_value) as u8;
    }

    pub fn set_de(&mut self, de_value: u16) {
        self.d = (de_value >> 8) as u8;;
        self.e = (de_value) as u8;
    }

    pub fn set_hl(&mut self, hl_value: u16) {
        self.h = (hl_value >> 8) as u8;;
        self.l = (hl_value) as u8;
    }

    pub fn is_flag_set(&self) -> bool {
        if self.f == 0 {
            return false;
        }
        if (CpuFlag::C as u8) & self.f != 0 {
            return true;
        }
        if (CpuFlag::N as u8) & self.f != 0 {
            return true;
        }
        if (CpuFlag::H as u8) & self.f != 0 {
            return true;
        }
        if (CpuFlag::Z as u8) & self.f != 0 {
            return true;
        }
        return false;
    }

    fn is_c_set(&self) -> bool {
        if (CpuFlag::C as u8) & self.f != 0 {
            return true;
        }
        return false;
    }

    fn is_h_set(&self) -> bool {
        if (CpuFlag::H as u8) & self.f != 0 {
            return true;
        }
        return false;
    }

    fn is_n_set(&self) -> bool {
        if (CpuFlag::N as u8) & self.f != 0 {
            return true;
        }
        return false;
    }

    fn is_z_set(&self) -> bool {
        if (CpuFlag::Z as u8) & self.f != 0 {
            return true;
        }
        return false;
    }

    fn set_c_flag(&mut self) {
        if !self.is_c_set() {
            self.f = (CpuFlag::C as u8) | self.f;
        }
    }

    fn set_h_flag(&mut self) {
        if !self.is_h_set() {
            self.f = (CpuFlag::H as u8) | self.f;
        }
    }

    fn set_n_flag(&mut self) {
        if !self.is_n_set() {
            self.f = (CpuFlag::N as u8) | self.f;
        }
    }

    fn set_z_flag(&mut self) {
        if !self.is_z_set() {
            self.f = (CpuFlag::Z as u8) | self.f;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registers() {
        let mut reg = Register::new();
        reg.a = 0x01;
        reg.b = 0x02;
        reg.c = 0x03;
        reg.d = 0x04;
        reg.e = 0x05;
        reg.h = 0x06;
        reg.l = 0x07;

        assert_eq!(reg.get_af(), 0x0100);
        assert_eq!(reg.get_bc(), 0x0203);
        assert_eq!(reg.get_de(), 0x0405);
        assert_eq!(reg.get_hl(), 0x0607);
    }

    #[test]
    fn test_setter() {
        let mut reg = Register::new();

        reg.set_af(0xAABB);

        assert_eq!(reg.get_af(), 0xAABB);
        assert_eq!(reg.a, 0xAA);
        assert_eq!(reg.f, 0xBB);

        reg.set_bc(0x1122);

        assert_eq!(reg.get_bc(), 0x1122);
        assert_eq!(reg.b, 0x11);
        assert_eq!(reg.c, 0x22);

        reg.set_de(0x3344);

        assert_eq!(reg.get_de(), 0x3344);
        assert_eq!(reg.d, 0x33);
        assert_eq!(reg.e, 0x44);

        reg.set_hl(0x5566);

        assert_eq!(reg.get_hl(), 0x5566);
        assert_eq!(reg.h, 0x55);
        assert_eq!(reg.l, 0x66);
    }

    #[test]
    fn test_flag() {
        let mut reg = Register::new();
        assert!(!reg.is_flag_set());

        reg.f = CpuFlag::C as u8;
        assert!(reg.is_flag_set());

        reg.f = CpuFlag::H as u8;
        assert!(reg.is_flag_set());

        reg.f = CpuFlag::N as u8;
        assert!(reg.is_flag_set());

        reg.f = CpuFlag::Z as u8;
        assert!(reg.is_flag_set());

        reg.f = 0b10100000;
        assert!(reg.is_flag_set());
    }

    #[test]
    fn test_c_flag() {
        let mut reg = Register::new();
        assert!(!reg.is_c_set());

        reg.set_c_flag();

        assert!(reg.is_c_set());
    }

    #[test]
    fn test_h_flag() {
        let mut reg = Register::new();
        assert!(!reg.is_h_set());

        reg.set_h_flag();

        assert!(reg.is_h_set());
    }

    #[test]
    fn test_n_flag() {
        let mut reg = Register::new();
        assert!(!reg.is_n_set());

        reg.set_n_flag();

        assert!(reg.is_n_set());
    }

    #[test]
    fn test_z_flag() {
        let mut reg = Register::new();
        assert!(!reg.is_z_set());

        reg.set_z_flag();

        assert!(reg.is_z_set());
    }
}
