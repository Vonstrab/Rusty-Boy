#![crate_name = "rusty_boy_bin"]
extern crate rusty_boy_lib;

use rusty_boy_lib::cpu_emulator::CpuEmulator;

fn main() {
    let mut cpu = CpuEmulator::new("rom/cpu_instrs/cpu_instrs.gb");
    cpu.load_code();
    println!("{:?}",cpu.code );
}
