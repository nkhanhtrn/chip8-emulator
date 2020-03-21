use crate::counter::ProgramCounter;
use crate::input::Input;
use crate::keyboard::Keyboard;
use crate::ram::RAM;
use crate::register::Register;
use crate::screen::Screen;
use crate::stack::Stack;
use rand::Rng;

pub const PROGRAM_START: u16 = 0x200;

pub struct CPU {
    register: Register,
    input: Input,
    counter: ProgramCounter,
    stack: Stack,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            register: Register::new(),
            input: Input::new(),
            counter: ProgramCounter::new(),
            stack: Stack::new(),
        }
    }

    pub fn run_instruction(&mut self, ram: &RAM, screen: &mut Screen, keyboard: &mut Keyboard) {
        let pc = self.counter.read();
        let hi = ram.read_byte(pc) as u16;
        let lo = ram.read_byte(pc + 1) as u16;
        let instruction: u16 = (hi << 8) | lo; // hi 8 + lo => u16, big-endian
        println!(
            "Instruction read {:#X}: lo {:#X} - hi {:#X}",
            instruction, lo, hi
        );

        let nnn = (instruction & 0x0FFF) as u16;
        let nn = (instruction & 0x00FF) as u8;
        let n = (instruction & 0x000F) as u8;
        let x = (instruction & 0x0F00 >> 8) as u8;
        let y = (instruction & 0x00F0 >> 4) as u8;
        let vx = self.register.get(x);
        let vy = self.register.get(y);

        let opcode = instruction >> 12;

        match opcode {
            0x00 => match nnn {
                0x0E0 => {
                    screen.clear();
                }
                0x0EE => {
                    let new_pc = self.stack.pop();
                    self.counter.jump(new_pc);
                }
                _ => panic!("Unrecognized instruction: {:#X}", opcode),
            },
            0x1 => {
                // Jumps to address NNN
                self.counter.jump(nnn);
            }
            0x2 => {
                // call subroutine at NNN
                self.stack.push(pc);
                self.counter.jump(nnn);
            }
            0x3 => {
                // Skip next instruction if VX == NN
                if self.register.get(x) == nn {
                    self.counter.skip();
                }
            }
            0x4 => {
                // Skip next instruction if VX != NN
                if self.register.get(x) != nn {
                    self.counter.skip();
                }
            }
            0x5 => {
                // Skip next instruction if VX == VY
                if self.register.get(x) == self.register.get(x) {
                    self.counter.skip();
                }
            }
            0x6 => {
                // Sets VX to NN
                self.register.set(x, nn);
            }
            0x7 => {
                // Vx += NN
                self.register.set(x, vx + nn);
            }
            0x8 => match n {
                0x0 => {
                    self.register.set(x, vy);
                }
                0x1 => {
                    self.register.set(x, vx | vy);
                }
                0x2 => {
                    self.register.set(x, vx & vy);
                }
                0x3 => {
                    self.register.set(x, vx ^ vy);
                }
                0x4 => {
                    // vx + vy, carry
                    let sum: u16 = (vx as u16) + (vy as u16);
                    if sum > 0xFF {
                        self.register.carry_on();
                    }
                    self.register.set(x, sum as u8);
                }
                0x5 => {
                    // vx - vy, carry if < 0
                    let sub: i8 = (vx as i8) - (vy as i8);
                    if sub < 0 {
                        self.register.carry_on();
                    } else {
                        self.register.carry_off();
                    }
                    self.register.set(x, sub as u8);
                }
                0x6 => {
                    // vx least bit => vF
                    // vx >> 1
                    self.register.set(0xF, vx & 0x1);
                    self.register.set(x, vx >> 1);
                }
                0x7 => {
                    // vy - vx, carry if < 0
                    let sub: i8 = (vy as i8) - (vx as i8);
                    if sub < 0 {
                        self.register.carry_on();
                    } else {
                        self.register.carry_off();
                    }
                    self.register.set(x, sub as u8);
                }
                0xE => {
                    // vx most bit => vF
                    // vx << 1
                    self.register.set(0xF, (vx & 0x80) >> 7);
                    self.register.set(x, vx << 1);
                }
                _ => panic!("Unrecognized instruction: {:#X}", opcode),
            },
            0x9 => {
                // skip if vx != vy
                if self.register.get(x) != self.register.get(y) {
                    self.counter.skip();
                }
            }
            0xA => {
                // Sets I to address NNN
                self.input.set(nnn);
            }
            0xB => {
                // Jumps to address NNN + V0
                self.counter.jump(nnn + self.register.get(0) as u16);
            }
            0xC => {
                // Sets VX to rand() & NNN
                let mut randomize = rand::thread_rng();
                let random_num = randomize.gen_range(0, 256) as u8;
                self.register.set(x, random_num & nn)
            }
            0xD => {
                screen.draw_byte(n, vx, vy);
            }
            0xE => match nn {
                // Skips if key press == vx
                0x9E => {
                    if keyboard.key_press(vx) {
                        self.counter.skip();
                    }
                }
                // Skips if key press !!= vx
                0xA1 => {
                    if !keyboard.key_press(vx) {
                        self.counter.skip();
                    }
                }
                _ => panic!("Unrecognized instruction: {:#X}", opcode),
            },
            0xF => match nn {
                _ => panic!("Unrecognized instruction: {:#X}", opcode),
            },
            _ => panic!("Unrecognized instruction: {:#X}", opcode),
        };

        self.counter.next();
    }
}
