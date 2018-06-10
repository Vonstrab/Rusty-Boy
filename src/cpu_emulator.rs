use inst::*;
use memory::*;
use register::*;

pub struct CpuEmulator {
    pub reg: Register,
    pub mem: Memory,
    pub code: [u8; 64 * 1024],
    pub pc: i64,
    pub sp: u16,
    filename: &'static str,
    pub stop: bool,
}

impl CpuEmulator {
    pub fn new(file: &'static str) -> CpuEmulator {
        CpuEmulator {
            reg: Register::new(),
            mem: Memory::new(),
            code: [0; 64 * 1024],
            pc: 0,
            sp: 0,
            filename: file,
            stop: false,
        }
    }

    pub fn get_next_byte(&mut self) -> u8 {
        let tmp = self.code[self.pc as usize];
        self.pc += 1;
        tmp
    }
}
