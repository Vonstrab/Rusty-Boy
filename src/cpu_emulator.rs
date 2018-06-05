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

    pub fn get_next_byte(&mut self) -> u8 {
        let tmp = self.code[self.pc as usize];
        self.pc += 1;
        tmp
    }

    pub fn exeucte_inst(&mut self) -> u16 {
        match self.get_next_byte() {
            //NOP
            0x00 => {
                return 4;
            }
            //LD BC,d16
            0x01 => {
                let tmp1 = self.get_next_byte();
                let tmp2 = self.get_next_byte();
                self.reg.set_bc((tmp1 as u16) << 8 | (tmp2 as u16));
                return 12;
            }
            //LD (BC),A
            0x02 => {
                self.mem.set_value8bit(self.reg.get_bc(), self.reg.a);
                return 8;
            }
            //INC BC
            0x03 => {
                let tmp = self.reg.get_bc();
                self.reg.set_bc(tmp + 1);
                return 8;
            }
            //INC B
            0x04 => {
                self.reg.b += 1;
                return 4;
            }
            0x05 => {
                unimplemented!();
            }

            0x06 => {
                unimplemented!();
            }
            0x07 => {
                unimplemented!();
            }
            0x08 => {
                unimplemented!();
            }
            0x09 => {
                unimplemented!();
            }
            0x0A => {
                unimplemented!();
            }
            0x0B => {
                unimplemented!();
            }
            0x0C => {
                unimplemented!();
            }
            0x0D => {
                unimplemented!();
            }
            0x1E => {
                unimplemented!();
            }
            0x0F => {
                unimplemented!();
            }
            0x10 => {
                unimplemented!();
            }
            0x11 => {
                unimplemented!();
            }
            0x12 => {
                unimplemented!();
            }
            0x13 => {
                unimplemented!();
            }
            0x14 => {
                unimplemented!();
            }
            0x15 => {
                unimplemented!();
            }
            0x16 => {
                unimplemented!();
            }
            0x017 => {
                unimplemented!();
            }
            0x18 => {
                unimplemented!();
            }
            0x19 => {
                unimplemented!();
            }
            0x1A => {
                unimplemented!();
            }

            0x1B => {
                unimplemented!();
            }
            0x1C => {
                unimplemented!();
            }
            0x1D => {
                unimplemented!();
            }
            0x1E => {
                unimplemented!();
            }
            0x1F => {
                unimplemented!();
            }
            _ => panic!(format!("Instruction n°{} non couverte", self.pc)),
        }
    }
}
