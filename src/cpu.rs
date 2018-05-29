use memory::*;
use register::*;

pub struct Cpu {
    pub reg: Register,
    pub mem: Memory,
    pub clock: u64,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg: Register::new(),
            mem: Memory::new(),
            clock: 0,
        }
    }

    pub fn nop(&mut self) {
        self.clock += 4;
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

    #[test]
    fn test_ldc() {
        let mut cpu = Cpu::new();
        cpu.ldc(0x78);
        assert_eq!(cpu.reg.c, 0x78);
        assert_eq!(cpu.clock, 8);
    }

    #[test]
    fn test_ldd() {
        let mut cpu = Cpu::new();
        cpu.ldd(0x78);
        assert_eq!(cpu.reg.d, 0x78);
        assert_eq!(cpu.clock, 8);
    }

    #[test]
    fn test_lde() {
        let mut cpu = Cpu::new();
        cpu.lde(0x78);
        assert_eq!(cpu.reg.e, 0x78);
        assert_eq!(cpu.clock, 8);
    }

    #[test]
    fn test_ldh() {
        let mut cpu = Cpu::new();
        cpu.ldh(0x78);
        assert_eq!(cpu.reg.h, 0x78);
        assert_eq!(cpu.clock, 8);
    }

    #[test]
    fn test_ldl() {
        let mut cpu = Cpu::new();
        cpu.ldl(0x78);
        assert_eq!(cpu.reg.l, 0x78);
        assert_eq!(cpu.clock, 8);
    }

}
