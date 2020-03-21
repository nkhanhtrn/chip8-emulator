use crate::cpu;
use crate::cpu::CPU;
use crate::ram::RAM;

pub struct CHIP8 {
    cpu: CPU,
    ram: RAM,
}

impl CHIP8 {
    pub fn new() -> CHIP8 {
        CHIP8 {
            cpu: CPU::new(),
            ram: RAM::new(),
        }
    }
    pub fn run_program(&mut self, program: &Vec<u8>) {
        for i in 0..program.len() {
            println!("{:#X}", program[i]);
            self.ram
                .write_byte(cpu::PROGRAM_START + (i as u16), program[i]);
        }

        loop {
            self.cpu.run_instruction(&self.ram);
        }
    }
}
