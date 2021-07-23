use crate::cpu::{ cpu::PROGRAM_START, cpu::CPU, instruction::Instruction };
use crate::io::{ keyboard::Keyboard, screen::Screen };
use crate::memories::{ ram::RAM, stack::Stack };
use crate::registers::{ counter::ProgramCounter, general::Register, I::RegisterI };
use wasm_bindgen::prelude::*;

mod cpu;
mod io;
mod memories;
mod registers;
mod resources;

#[wasm_bindgen]
pub struct CHIP8 {
    cpu: CPU,
}

#[wasm_bindgen]
impl CHIP8 {
    pub fn new() -> CHIP8 {
        let counter = ProgramCounter::new();
        let keyboard = Keyboard::new();
        let register = Register::new();
        let register_i = RegisterI::new();
        let ram = RAM::new();
        let screen = Screen::new();
        let stack = Stack::new();
        let cpu = CPU::new(counter, keyboard, register, register_i, ram, screen, stack);
        CHIP8 { cpu }
    }

    pub fn run_program(&mut self, program: Vec<u8>) {
        // load program
        for i in 0..program.len() {
            self.cpu.ram.write_byte(PROGRAM_START + (i as u16), program[i]);
        }

        // loop to run programs
        loop {
            let pc = self.cpu.counter.read();
            let high_byte = self.cpu.ram.read_byte(pc) as u16;
            let low_byte = self.cpu.ram.read_byte(pc + 1) as u16;
            let instruction = Instruction::new(high_byte, low_byte);
            self.cpu.run_instruction(&instruction);
        }
    }

    pub fn get_screen(&self) -> *const u8 {
        self.cpu.screen.memory()
    }

    pub fn get_memory(&self) -> *const u8 {
        self.cpu.ram.memory()
    }
}