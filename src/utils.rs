use std::fs::File;
use std::io::prelude::*;

pub fn read_gb_to_vec(filepath: String) -> Vec<u8> {
    let mut f = File::open(filepath).expect("Something went wrong reading the file");
    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer)
        .expect("Something went wrong reading the file");

    buffer
}
