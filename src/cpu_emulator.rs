use memory::*;
use register::*;

pub struct CpuEmulator {
    pub reg: Register,
    pub mem: Memory,
    code: [u8; 64 * 1024],
    pc: i64,
    sp: u16,
    filename: &'static str,
    stop: bool,
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
                if self.reg.b == 0 {
                    self.reg.set_z_flag();
                }
                self.reg.unset_c_flag();
                return 4;
            }
            //DEC B
            0x05 => {
                self.reg.b -= 1;
                if self.reg.b == 0 {
                    self.reg.set_z_flag();
                }
                self.reg.set_c_flag();
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
                if self.reg.c == 0 {
                    self.reg.set_z_flag();
                }
                self.reg.unset_c_flag();
                return 4;
            }
            //DEC C
            0x0D => {
                self.reg.c -= 1;
                if self.reg.c == 0 {
                    self.reg.set_z_flag();
                }
                self.reg.set_c_flag();
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
            //STOP
            0x10 => {
                self.stop = true;
                self.get_next_byte();
                return 4;
            }
            //LD DE,d16
            0x11 => {
                let tmp1 = self.get_next_byte();
                let tmp2 = self.get_next_byte();
                self.reg.set_de((tmp1 as u16) << 8 | (tmp2 as u16));
                return 12;
            }
            //LD (DE),A
            0x12 => {
                self.mem.set_value8bit(self.reg.get_de(), self.reg.a);
                return 8;
            }
            //INC DE
            0x13 => {
                let tmp = self.reg.get_bc();
                self.reg.set_de(tmp + 1);
                return 8;
                unimplemented!();
            }
            //INC D
            0x14 => {
                self.reg.d += 1;
                if self.reg.d == 0 {
                    self.reg.set_z_flag();
                }
                self.reg.unset_c_flag();
                return 4;
            }
            //DEC D
            0x15 => {
                self.reg.d -= 1;
                if self.reg.d == 0 {
                    self.reg.set_z_flag();
                }
                self.reg.unset_c_flag();
                return 4;
            }
            //LD D,d8
            0x16 => {
                self.reg.d = self.get_next_byte();
                return 8;
            }
            //RLA
            0x017 => {
                let mut tmp = self.reg.a << 1;
                if self.reg.is_c_set() {
                    tmp = tmp | 0x01;
                }
                if self.reg.a.leading_zeros() == 0 {
                    self.reg.set_c_flag();
                } else {
                    self.reg.unset_c_flag();
                }
                self.reg.a = tmp;
                return 4;
            }
            //JR r8
            0x18 => {
                let tmp = self.get_next_byte();
                self.pc += (tmp as i64);
                return 12;
            }
            //ADD HL,DE
            0x19 => {
                let tmp = self.reg.get_hl();
                let add = tmp.overflowing_add(self.reg.get_de());
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
            //LD A,(DE)
            0x1A => {
                self.reg.a = self.mem.get_value8bit(self.reg.get_de());
                return 8;
            }
            //DEC DE
            0x1B => {
                let tmp = self.reg.get_de();
                self.reg.set_bc(tmp - 1);
                return 8;
            }
            //INC E
            0x1C => {
                self.reg.e += 1;
                if self.reg.e == 0 {
                    self.reg.set_z_flag();
                }
                self.reg.unset_c_flag();
                return 4;
            }
            //DEC E
            0x1D => {
                self.reg.e -= 1;
                if self.reg.e == 0 {
                    self.reg.set_z_flag();
                }
                self.reg.set_c_flag();
                return 4;
            }
            //LD E,d8
            0x1E => {
                let tmp = self.get_next_byte();
                self.reg.c = tmp;
                return 8;
            }
            //RRA
            0x1F => {
                let mut tmp = self.reg.a >> 1;
                if self.reg.is_c_set() {
                    tmp = tmp | 0x80;
                }
                if self.reg.a.trailing_zeros() == 0 {
                    self.reg.set_c_flag();
                } else {
                    self.reg.unset_c_flag();
                }
                self.reg.a = tmp;
                return 4;
            }
            _ => panic!(format!("Instruction n°{} non couverte", self.pc)),
        }
    }
}
