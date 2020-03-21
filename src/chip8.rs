use crate::cpu;
use crate::cpu::CPU;
use crate::keyboard::Keyboard;
use crate::ram::RAM;
use crate::screen::Screen;

pub struct CHIP8 {
    cpu: CPU,
    ram: RAM,
    keyboard: Keyboard,
    screen: Screen,
}

impl CHIP8 {
    pub fn new() -> CHIP8 {
        CHIP8 {
            cpu: CPU::new(),
            ram: RAM::new(),
            keyboard: Keyboard::new(),
            screen: Screen::new(),
        }
    }
    pub fn run_program(&mut self, program: &Vec<u8>) {
        for i in 0..program.len() {
            self.ram
                .write_byte(cpu::PROGRAM_START + (i as u16), program[i]);
        }

        loop {
            self.cpu
                .run_instruction(&self.ram, &mut self.screen, &mut self.keyboard);
        }
    }
}
