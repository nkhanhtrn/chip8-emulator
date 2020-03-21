use crate::cpu;

pub struct ProgramCounter {
    counter: u16,
    moved: bool,
}

impl ProgramCounter {
    pub fn new() -> ProgramCounter {
        ProgramCounter {
            counter: cpu::PROGRAM_START,
            moved: false,
        }
    }

    pub fn read(&mut self) -> u16 {
        self.moved = false;
        self.counter
    }

    pub fn next(&mut self) {
        if !self.moved {
            self.counter += 2;
        }
    }

    pub fn skip(&mut self) {
        self.counter += 4;
        self.moved = true;
    }

    pub fn jump(&mut self, nnn: u16) {
        self.counter = nnn;
        self.moved = true;
    }
}
