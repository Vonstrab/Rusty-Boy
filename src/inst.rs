use cpu_emulator::*;

pub fn execute_inst(mut cpu: CpuEmulator) -> u16 {
    match cpu.get_next_byte() {
        //NOP
        0x00 => {
            return 4;
        }
        //LD BC,d16
        0x01 => {
            let tmp1 = cpu.get_next_byte();
            let tmp2 = cpu.get_next_byte();
            cpu.reg.set_bc((tmp1 as u16) << 8 | (tmp2 as u16));
            return 12;
        }
        //LD (BC),A
        0x02 => {
            cpu.mem.set_value8bit(cpu.reg.get_bc(), cpu.reg.a);
            return 8;
        }
        //INC BC
        0x03 => {
            let tmp = cpu.reg.get_bc();
            cpu.reg.set_bc(tmp + 1);
            return 8;
        }
        //INC B
        0x04 => {
            cpu.reg.b += 1;
            if cpu.reg.b == 0 {
                cpu.reg.set_z_flag();
            } else {
                cpu.reg.unset_z_flag();
            }
            if cpu.reg.b.trailing_zeros() >= 4 {
                cpu.reg.set_h_flag();
            } else {
                cpu.reg.unset_h_flag();
            }
            cpu.reg.unset_n_flag();
            return 4;
        }
        //DEC B
        0x05 => {
            if cpu.reg.b.trailing_zeros() >= 4 {
                cpu.reg.set_h_flag();
            } else {
                cpu.reg.unset_h_flag();
            }

            cpu.reg.b -= 1;
            if cpu.reg.b == 0 {
                cpu.reg.set_z_flag();
            } else {
                cpu.reg.unset_z_flag();
            }
            cpu.reg.set_n_flag();
            return 4;
        }
        //LD B,d8
        0x06 => {
            cpu.reg.b = cpu.get_next_byte();
            return 8;
        }
        //RLCA
        0x07 => {
            let tmp = cpu.reg.a.rotate_left(1);
            if tmp.trailing_zeros() == 0 {
                cpu.reg.set_c_flag();
            } else {
                cpu.reg.unset_c_flag();
            }
            cpu.reg.a = tmp;
            return 4;
        }
        //LD (ad),SP
        0x08 => {
            let la = (cpu.get_next_byte() as u16) << 8 | (cpu.get_next_byte() as u16);
            cpu.mem.set_value16bit(la + 1, la, cpu.sp);
            return 20;
        }
        //ADD HL,BC
        0x09 => {
            let hl = cpu.reg.get_hl();
            let bc = cpu.reg.get_bc();

            if (bc ^ 0x0800) == 0 && (hl ^ 0x0800) == 0 {
                cpu.reg.unset_h_flag();
            } else {
                cpu.reg.set_h_flag();
            }
            let add = hl.overflowing_add(bc);
            match add {
                (x, true) => {
                    cpu.reg.set_hl(x);
                    cpu.reg.set_c_flag();
                }
                (x, _) => {
                    cpu.reg.set_hl(x);
                    cpu.reg.set_c_flag();
                }
            }
            cpu.reg.unset_n_flag();
            return 8;
        }
        //LD A,(BC)
        0x0A => {
            cpu.reg.a = cpu.mem.get_value8bit(cpu.reg.get_bc());
            return 8;
        }
        //DEC BC
        0x0B => {
            let tmp = cpu.reg.get_bc();
            cpu.reg.set_bc(tmp - 1);
            return 8;
        }
        //INC C
        0x0C => {
            cpu.reg.c += 1;

            if cpu.reg.c == 0 {
                cpu.reg.set_z_flag();
            } else {
                cpu.reg.unset_z_flag();
            }

            if cpu.reg.c.trailing_zeros() >= 4 {
                cpu.reg.set_h_flag();
            } else {
                cpu.reg.unset_h_flag();
            }
            cpu.reg.unset_n_flag();
            return 4;
        }
        //DEC C
        0x0D => {
            if cpu.reg.c.trailing_zeros() >= 4 {
                cpu.reg.set_h_flag();
            } else {
                cpu.reg.unset_h_flag();
            }

            cpu.reg.c -= 1;

            if cpu.reg.c == 0 {
                cpu.reg.set_z_flag();
            } else {
                cpu.reg.unset_z_flag();
            }
            cpu.reg.set_n_flag();
            return 4;
        }
        //LD C, d8
        0x0E => {
            let tmp = cpu.get_next_byte();
            cpu.reg.c = tmp;
            return 8;
        }
        //RRCA
        0x0F => {
            let tmp = cpu.reg.a.rotate_right(1);
            if tmp.leading_zeros() == 0 {
                cpu.reg.set_c_flag();
            } else {
                cpu.reg.unset_c_flag();
            }
            cpu.reg.a = tmp;
            return 4;
        }
        //STOP
        0x10 => {
            cpu.stop = true;
            cpu.get_next_byte();
            return 4;
        }
        //LD DE,d16
        0x11 => {
            let tmp1 = cpu.get_next_byte();
            let tmp2 = cpu.get_next_byte();
            cpu.reg.set_de((tmp1 as u16) << 8 | (tmp2 as u16));
            return 12;
        }
        //LD (DE),A
        0x12 => {
            cpu.mem.set_value8bit(cpu.reg.get_de(), cpu.reg.a);
            return 8;
        }
        //INC DE
        0x13 => {
            let tmp = cpu.reg.get_bc();
            cpu.reg.set_de(tmp + 1);
            return 8;
        }
        //INC D
        0x14 => {
            cpu.reg.d += 1;
            if cpu.reg.d == 0 {
                cpu.reg.set_z_flag();
            } else {
                cpu.reg.unset_z_flag();
            }
            if cpu.reg.d.trailing_zeros() >= 4 {
                cpu.reg.set_h_flag();
            } else {
                cpu.reg.unset_h_flag();
            }
            cpu.reg.unset_n_flag();
            return 4;
        }
        //DEC D
        0x15 => {
            if cpu.reg.d.trailing_zeros() >= 4 {
                cpu.reg.set_h_flag();
            } else {
                cpu.reg.unset_h_flag();
            }

            cpu.reg.d += 1;

            if cpu.reg.d == 0 {
                cpu.reg.set_z_flag();
            } else {
                cpu.reg.unset_z_flag();
            }
            cpu.reg.set_n_flag();
            return 4;
        }
        //LD D,d8
        0x16 => {
            cpu.reg.d = cpu.get_next_byte();
            return 8;
        }
        //RLA
        0x017 => {
            let mut tmp = cpu.reg.a << 1;
            if cpu.reg.is_c_set() {
                tmp = tmp | 0x01;
            }
            if cpu.reg.a.leading_zeros() == 0 {
                cpu.reg.set_c_flag();
            } else {
                cpu.reg.unset_c_flag();
            }
            cpu.reg.a = tmp;
            return 4;
        }
        //JR r8
        0x18 => {
            let tmp = cpu.get_next_byte();
            cpu.pc += tmp as i64;
            return 12;
        }
        //ADD HL,DE
        0x19 => {
            let hl = cpu.reg.get_hl();
            let de = cpu.reg.get_de();

            if (de ^ 0x0800) == 0 && (hl ^ 0x0800) == 0 {
                cpu.reg.unset_h_flag();
            } else {
                cpu.reg.set_h_flag();
            }
            let add = hl.overflowing_add(de);
            match add {
                (x, true) => {
                    cpu.reg.set_hl(x);
                    cpu.reg.set_c_flag();
                }
                (x, _) => {
                    cpu.reg.set_hl(x);
                    cpu.reg.set_c_flag();
                }
            }
            cpu.reg.unset_n_flag();
            return 8;
        }
        //LD A,(DE)
        0x1A => {
            cpu.reg.a = cpu.mem.get_value8bit(cpu.reg.get_de());
            return 8;
        }
        //DEC DE
        0x1B => {
            let tmp = cpu.reg.get_de();
            cpu.reg.set_bc(tmp - 1);
            return 8;
        }
        //INC E
        0x1C => {
            cpu.reg.e += 1;

            if cpu.reg.e.trailing_zeros() >= 4 {
                cpu.reg.set_h_flag();
            } else {
                cpu.reg.unset_h_flag();
            }

            if cpu.reg.e == 0 {
                cpu.reg.set_z_flag();
            } else {
                cpu.reg.unset_z_flag();
            }
            cpu.reg.unset_n_flag();
            return 4;
        }
        //DEC E
        0x1D => {
            if cpu.reg.e.trailing_zeros() >= 4 {
                cpu.reg.set_h_flag();
            } else {
                cpu.reg.unset_h_flag();
            }

            cpu.reg.e += 1;

            if cpu.reg.e == 0 {
                cpu.reg.set_z_flag();
            } else {
                cpu.reg.unset_z_flag();
            }
            cpu.reg.set_n_flag();
            return 4;
        }
        //LD E,d8
        0x1E => {
            let tmp = cpu.get_next_byte();
            cpu.reg.c = tmp;
            return 8;
        }
        //RRA
        0x1F => {
            let mut tmp = cpu.reg.a >> 1;
            if cpu.reg.is_c_set() {
                tmp = tmp | 0x80;
            }
            if cpu.reg.a.trailing_zeros() == 0 {
                cpu.reg.set_c_flag();
            } else {
                cpu.reg.unset_c_flag();
            }
            cpu.reg.a = tmp;
            return 4;
        }
        //JR NZ r8
        0x20 => {
            let tmp = cpu.get_next_byte();
            if cpu.reg.is_z_set() {
                return 12;
            } else {
                cpu.pc += tmp as i64;
                return 8;
            }
        }

        _ => panic!(format!("Instruction n°{} non couverte", cpu.pc)),
    }
}
