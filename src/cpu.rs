use crate::ram::RAM;

pub const PROGRAM_START: u16 = 0x200;

pub struct CPU {
    opcode: u16,
    vx: [u8; 16],
    i: u16,
    pc: u16,
    sp: u8,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            opcode: 0,
            vx: [0; 16],
            i: 0,
            pc: PROGRAM_START,
            sp: 0,
        }
    }

    pub fn run_instruction(&mut self, ram: &RAM) {
        let lo = ram.read_byte(self.pc);
        let hi = ram.read_byte(self.pc + 1);
        let instruction: u8 = lo | hi;
        println!("{:?}: lo {:?} - hi {:?}", instruction, lo, hi);

        self.pc += 2;
    }
}
