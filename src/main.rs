use crate::chip8::CHIP8;

mod chip8;
mod cpu;
mod ram;

fn main() {
    let mut chip8 = CHIP8::new();
    let mut program = vec![0x13, 0xC5];
    chip8.run_program(&mut program);
}
