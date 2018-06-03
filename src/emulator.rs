use memory::*;
use register::*;

pub struct Emulator {
    pub reg: Register,
    pub mem: Memory,
    clock: u64,
    code: [u8; 64 * 1024],
    pc: u64,
    filename: &'static str,
}

impl Emulator {
    pub fn new(file: &'static str) -> Emulator {
        Emulator {
            reg: Register::new(),
            mem: Memory::new(),
            clock: 0,
            code: [0; 64 * 1024],
            pc: 0,
            filename: file,
        }
    }

    pub fn exeucte_inst(&mut self) -> u16 {
        match self.code[self.pc as usize] {
            0x00 => {
                self.clock += 4;
                return 4;
            }
            0x01 => {
                self.reg.set_bc(
                    (self.code[(self.pc + 1) as usize] as u16) << 8
                        | (self.code[(self.pc + 2) as usize] as u16),
                );
                self.clock += 12;
                self.pc += 3;
                return 12;
            }
            _ => panic!(format!("Instruction n°{} non couverte", self.pc)),
        }
    }
}
