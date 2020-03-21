use crate::chip8::CHIP8;
use std::fs::File;
use std::io::Read;

mod chip8;
mod counter;
mod cpu;
mod input;
mod keyboard;
mod ram;
mod register;
mod screen;
mod stack;

fn main() {
    let mut file = File::open("ROMs/PONG").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    let mut chip8 = CHIP8::new();
    chip8.run_program(&data);
}
