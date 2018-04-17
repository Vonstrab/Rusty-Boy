use register::*;

pub struct Cpu {
    pub reg: Register,
    pub clock: u64,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg: Register::new(),
            clock: 0,
        }
    }

    pub fn ldb(&mut self, n: u8) {
        self.reg.b = n;
        self.clock += 8;
    }

    pub fn ldc(&mut self, n: u8) {
        self.reg.c = n;
        self.clock += 8;
    }

    pub fn ldd(&mut self, n: u8) {
        self.reg.d = n;
        self.clock += 8;
    }

    pub fn lde(&mut self, n: u8) {
        self.reg.e = n;
        self.clock += 8;
    }

    pub fn ldh(&mut self, n: u8) {
        self.reg.h = n;
        self.clock += 8;
    }

    pub fn ldl(&mut self, n: u8) {
        self.reg.l = n;
        self.clock += 8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ldb() {
        let mut cpu = Cpu::new();
        cpu.ldb(0x78);
        assert_eq!(cpu.reg.b, 0x78);
        assert_eq!(cpu.clock, 8);
    }
}
