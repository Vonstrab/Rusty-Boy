#![crate_name = "rusty_boy_bin"]
extern crate rusty_boy_lib;

use rusty_boy_lib::register;

fn main() {
    println!("Hello, world!");
    let m = register::Register::new();
}
