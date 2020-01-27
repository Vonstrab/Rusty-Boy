use memory::*;
use register::*;
use utils;

pub struct CpuEmulator {
    pub reg: Register,
    pub mem: Memory,
    pub code: Vec<u8>,
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
            code: Vec::new(),
            pc: 0,
            sp: 0,
            filename: file,
            stop: false,
        }
    }

    pub fn load_code(&mut self){
        self.code=utils::read_gb_to_vec(self.filename.to_string());
    }

    pub fn get_next_byte(&mut self) -> u8 {
        let tmp = self.code[self.pc as usize];
        self.pc += 1;
        tmp
    }

}
