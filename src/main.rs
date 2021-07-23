use crate::chip8::CHIP8;
use std::fs::File;
use std::io::Read;

mod cpu;
mod chip8;
mod io;
mod memories;
mod registers;
mod resources;

fn main() {
    let mut file = File::open("ROMs/PONG").unwrap();
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data).unwrap();

    let mut chip8 = CHIP8::new();
    chip8.run_program(&data);
}
