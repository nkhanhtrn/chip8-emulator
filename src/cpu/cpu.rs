use crate::io::keyboard::Keyboard;
use crate::io::screen::Screen;
use crate::memories::ram::RAM;
use crate::memories::stack::Stack;
use crate::registers::counter::ProgramCounter;
use crate::registers::general::Register;
use crate::registers::I::RegisterI;
use super::instruction::Instruction;
use rand::Rng;

pub const PROGRAM_START: u16 = 0x200;
pub const INSTRUCTION_LENGTH: u16 = 2;

pub struct CPU {
    pub counter: ProgramCounter,
    pub keyboard: Keyboard,
    pub register: Register,
    pub register_i: RegisterI,
    pub ram: RAM,
    pub screen: Screen,
    pub stack: Stack,
}

impl CPU {
    pub fn new(
        counter: ProgramCounter, keyboard: Keyboard, register: Register,
        register_i: RegisterI, ram: RAM,  screen: Screen,
        stack: Stack) -> CPU {
        CPU {
            counter,
            keyboard,
            register,
            register_i,
            ram,
            screen,
            stack,
        }
    }

    pub fn run_instruction(&mut self, ins: &Instruction) {
        let vx = self.register.get(ins.x);
        let vy = self.register.get(ins.y);

        match (ins.op, ins.x, ins.y, ins.n) {
            (0x0, 0x0, 0xE, 0x0) => self.screen.clear(),
            (0x0, 0x0, 0xE, 0xE) => {
                let previous_pc = self.stack.pop();
                self.counter.jump(previous_pc);
            },
            (0x1, _, _, _) => self.counter.jump(ins.nnn),
            (0x2, _, _, _) => {
                self.stack.push(self.counter.read()).unwrap();
                self.counter.jump(ins.nnn);
            },
            (0x3, _, _, _) => {
                if vx == ins.kk {
                    self.counter.skip();
                }
            },
            (0x4, _, _, _) => {
                if vx != ins.kk {
                    self.counter.skip();
                }
            },
            (0x5, _, _, 0x0) => {
                if vx == vy {
                    self.counter.skip();
                }
            },
            (0x6, _, _, _) => {
                self.register.set(ins.x, ins.kk);
            },
            (0x7, _, _, _) => {
                self.register.set(ins.x, vx + ins.kk);
            },
            (0x8, _, _, 0x0) => {
                self.register.set(ins.x, vy);
            },
            (0x8, _, _, 0x1) => {
                self.register.set(ins.x, vx | vy);
            },
            (0x8, _, _, 0x2) => {
                self.register.set(ins.x, vx & vy);
            },
            (0x8, _, _, 0x3) => {
                self.register.set(ins.x, vx ^ vy);
            },
            (0x8, _, _, 0x4) => {
                let mut sum: u16 = (vx as u16) + (vy as u16);
                if sum > 0xFF {
                    self.register.carry_bit_on();
                    sum -= 0xFF;
                }
                self.register.set(ins.x, sum as u8);
            },
            (0x8, _, _, 0x5) => {
                let sub: i8 = (vx as i8) - (vy as i8);
                if sub < 0 {
                    self.register.carry_bit_on();
                } else {
                    self.register.carry_bit_off();
                }
                self.register.set(ins.x, sub as u8);
            },
            (0x8, _, _, 0x6) => {
                if vx & 0x1 == 1 {
                    self.register.carry_bit_on();
                } else {
                    self.register.carry_bit_off();
                }
                self.register.set(ins.x, vx >> 1);
            },
            (0x8, _, _, 0x7) => {
                let sub: i8 = (vy as i8) - (vx as i8);
                if sub < 0 {
                    self.register.carry_bit_on();
                } else {
                    self.register.carry_bit_off();
                }
                self.register.set(ins.x, sub as u8);
            },
            (0x8, _, _, 0xE) => {
                if (vx >> 7) & 0x1 == 1 {
                    self.register.carry_bit_on();
                } else {
                    self.register.carry_bit_off();
                }
                self.register.set(ins.x, vx << 1);
            },
            (0x9, _, _, 0x0) => {
                if vx != vy {
                    self.counter.skip();
                }
            },
            (0xA, _, _, _) => {
                self.register_i.set(ins.nnn);
            },
            (0xB, _, _, _) => {
                self.counter.jump(ins.nnn + self.register.get(0) as u16);
            },
            (0xC, _, _, _) => {
                let mut randomize = rand::thread_rng();
                let random_num = randomize.gen_range(0..255) as u8;
                self.register.set(ins.x, random_num & ins.kk)
            },
            (0xD, _, _, _) => {
                let sprite = self.ram.read_bytes(self.register_i.get(), ins.n as u16);
                let is_collision = self.screen.draw(sprite, vx as isize, vy as isize);
                if is_collision {
                    self.register.carry_bit_on();
                } else {
                    self.register.carry_bit_off();
                }
            },
            /*
            0xE => match kk {
                // 0xEx9E
                0x9E => {
                    if keyboard.press(vx).unwrap() {
                        self.counter.skip();
                    }
                },
                // 0xExA1
                0xA1 => {
                    if !keyboard.press(vx).unwrap() {
                        self.counter.skip();
                    }
                },
                _ => panic!("Unrecognized instruction: {:#X}", opcode),
            },
            0xF => match kk {
                _ => panic!("Unrecognized instruction: {:#X}", opcode),
            },
            */
            (_,_,_,_) => panic!("Unrecognized instruction: {:#X}", ins.op),
        };

        self.counter.next();
    }
}