use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    generate_decoder(out_dir.clone().join("generated_decoder.rs"));
}

fn generate_decoder(out_path: PathBuf) {
    let mut output = String::new();

    for opcode in 0x00..=0xFF {
        let x = (opcode & 0b11000000) >> 6;
        let y = (opcode & 0b00111000) >> 3;
        let z =  opcode & 0b00000111;
        let p = (opcode & 0b00110000) >> 4;
        let q = (opcode & 0b00001000) >> 3;
    }
}

fn generate
