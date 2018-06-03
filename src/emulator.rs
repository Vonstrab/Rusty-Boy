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
    pub fn new(file:&'static str) -> Emulator {
        Emulator {
            reg: Register::new(),
            mem: Memory::new(),
            clock: 0,
            code: [0; 64 * 1024],
            pc: 0,
            filename: file,
        }
    }
}
