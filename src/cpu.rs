use crate::ram::RAM;
use crate::register::Register;

pub const PROGRAM_START: u16 = 0x200;

pub struct CPU {
    v: Register,
    i: u16,
    pc: u16,
    sp: u8,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            v: Register::new(),
            i: 0,
            pc: PROGRAM_START,
            sp: 0,
        }
    }

    pub fn run_instruction(&mut self, ram: &RAM) {
        let hi = ram.read_byte(self.pc) as u16;
        let lo = ram.read_byte(self.pc + 1) as u16;
        let instruction: u16 = (hi << 8) | lo; // hi 8 + lo => u16, big-endian
        println!(
            "Instruction read {:#X}: lo {:#X} - hi {:#X}",
            instruction, lo, hi
        );

        let mut pc_modified = false;
        let nnn = (instruction & 0x0FFF) as u16;
        let nn = (instruction & 0x00FF) as u8;
        let n = (instruction & 0x000F) as u8;
        let x = (instruction & 0x0F00 >> 8) as u8;
        let y = (instruction & 0x00F0 >> 4) as u8;
        let opcode = instruction & 0xF000 >> 12;
        match opcode {
            0x1 => {
                // Jumps to address NNN
                self.pc = nnn;
                pc_modified = true;
            }
            0x6 => {
                // Sets VX to NN
                self.v.set(x, nn);
            }
            _ => panic!("Unrecognized instruction: {:#X}", instruction),
        };

        if !pc_modified {
            self.pc += 2;
        }
    }
}
