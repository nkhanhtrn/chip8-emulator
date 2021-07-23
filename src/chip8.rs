use crate::cpu::cpu;
use crate::cpu::cpu::CPU;
use crate::cpu::instruction::Instruction;
use crate::io::keyboard::Keyboard;
use crate::io::screen::Screen;
use crate::memories::ram::RAM;
use crate::memories::stack::Stack;
use crate::registers::counter::ProgramCounter;
use crate::registers::general::Register;
use crate::registers::I::RegisterI;

pub struct CHIP8 {
    cpu: CPU,
}

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

    pub fn run_program(&mut self, program: &Vec<u8>) {
        // load program
        for i in 0..program.len() {
            self.cpu.ram.write_byte(cpu::PROGRAM_START + (i as u16), program[i]);
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
}