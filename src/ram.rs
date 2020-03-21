
public struct RAM {
    memory: [u8; 4096]
}

impl RAM {
    pub fn new() -> RAM {
        RAM {
            memory: [0; 4096]
        }
    }
    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address] = value;
    }

    pub fn read_byte(&mut self, address: u16, value: u8) {

    }
}