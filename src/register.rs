pub struct Register {
    memory: [u8; 16],
}

impl Register {
    pub fn new() -> Register {
        Register { memory: [0; 16] }
    }

    pub fn set(&mut self, index: u8, value: u8) {
        self.memory[index as usize] = value;
    }
}
