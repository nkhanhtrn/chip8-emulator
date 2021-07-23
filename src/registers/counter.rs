use crate::cpu::cpu;

pub struct ProgramCounter {
    counter: u16,
}

impl ProgramCounter {
    pub fn new() -> ProgramCounter {
        ProgramCounter {
            counter: cpu::PROGRAM_START,
        }
    }

    pub fn jump(&mut self, nnn: u16) {
        self.counter = nnn;
    }

    pub fn read(&self) -> u16 {
        self.counter
    }

    pub fn next(&mut self) {
        self.counter += cpu::INSTRUCTION_LENGTH;
    }

    pub fn skip(&mut self) {
        self.counter += cpu::INSTRUCTION_LENGTH * 2;
    }
}

#[cfg(tests)]
mod tests {
    use super::*;
}