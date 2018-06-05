use memory::*;
use register::*;

pub struct CpuEmulator {
    pub reg: Register,
    pub mem: Memory,
    code: [u8; 64 * 1024],
    pc: u64,
    filename: &'static str,
}

impl CpuEmulator {
    pub fn new(file: &'static str) -> CpuEmulator {
        CpuEmulator {
            reg: Register::new(),
            mem: Memory::new(),
            code: [0; 64 * 1024],
            pc: 0,
            filename: file,
        }
    }

    pub fn exeucte_inst(&mut self) -> u16 {
        match self.code[self.pc as usize] {
            0x00 => {
                self.pc += 1;
                return 4;
            }
            0x01 => {
                self.reg.set_bc(
                    (self.code[(self.pc + 1) as usize] as u16) << 8
                        | (self.code[(self.pc + 2) as usize] as u16),
                );
                self.pc += 3;
                return 12;
            }
            0x02 => {
                self.mem.set_value8bit(self.reg.get_bc(), self.reg.a);
                self.pc += 1;
                return 8;
            }
            _ => panic!(format!("Instruction n°{} non couverte", self.pc)),
        }
    }
}
