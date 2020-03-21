pub struct Register {
    memory: [u8; 16],
}

impl Register {
    pub fn new() -> Register {
        Register { memory: [0; 16] }
    }

    pub fn get(&mut self, index: u8) -> u8 {
        self.memory[index as usize]
    }

    pub fn set(&mut self, index: u8, value: u8) {
        self.memory[index as usize] = value;
    }

    pub fn carry_on(&mut self) {
        self.memory[0xF] = 1;
    }

    pub fn carry_off(&mut self) {
        self.memory[0xF] = 0;
    }
}
