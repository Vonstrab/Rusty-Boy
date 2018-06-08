use memory::*;
use register::*;

pub struct CpuEmulator {
    pub reg: Register,
    pub mem: Memory,
    code: [u8; 64 * 1024],
    pc: u64,
    sp: u16,
    filename: &'static str,
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
        }
    }

    pub fn get_next_byte(&mut self) -> u8 {
        let tmp = self.code[self.pc as usize];
        self.pc += 1;
        tmp
    }

    pub fn execute_inst(&mut self) -> u16 {
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
            //DEC B
            0x05 => {
                self.reg.b -= 1;
                return 4;
            }
            //LD B,d8
            0x06 => {
                self.reg.b = self.get_next_byte();
                return 8;
            }
            //RLCA
            0x07 => {
                let tmp = self.reg.a.rotate_left(1);
                if tmp.trailing_zeros() == 0 {
                    self.reg.set_c_flag();
                } else {
                    self.reg.unset_c_flag();
                }
                self.reg.a = tmp;
                return 4;
            }
            //LD (ad),SP
            0x08 => {
                let la = (self.get_next_byte() as u16) << 8 | (self.get_next_byte() as u16);
                self.mem.set_value16bit(la + 1, la, self.sp);
                return 20;
            }
            //ADD HL,BC
            0x09 => {
                let tmp = self.reg.get_hl();
                let add = tmp.overflowing_add(self.reg.get_bc());
                match add {
                    (x, true) => {
                        self.reg.set_hl(x);
                        self.reg.set_c_flag();
                    }
                    (x, _) => {
                        self.reg.set_hl(x);
                        self.reg.set_c_flag();
                    }
                }
                self.reg.unset_n_flag();
                return 8;
            }
            //LD A,(BC)
            0x0A => {
                self.reg.a = self.mem.get_value8bit(self.reg.get_bc());
                return 8;
            }
            //DEC BC
            0x0B => {
                let tmp = self.reg.get_bc();
                self.reg.set_bc(tmp - 1);
                return 8;
            }
            //INC C
            0x0C => {
                self.reg.c += 1;
                return 4;
            }
            //DEC C
            0x0D => {
                self.reg.c -= 1;
                return 4;
            }
            //LD C, d8
            0x0E => {
                let tmp = self.get_next_byte();
                self.reg.c = tmp;
                return 8;
            }
            //RRCA
            0x0F => {
                let tmp = self.reg.a.rotate_right(1);
                if tmp.leading_zeros() == 0 {
                    self.reg.set_c_flag();
                } else {
                    self.reg.unset_c_flag();
                }
                self.reg.a = tmp;
                return 4;
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
