use crate::resources::font::*;

pub struct RAM {
    memory: [u8; 4096],
}

impl RAM {
    pub fn new() -> RAM {
        let mut ram = RAM { memory: [0; 4096] };
        ram.add_font();
        ram
    }

    pub fn memory(&self) -> *const u8 {
        self.memory.as_ptr()
    }

    fn add_font(&mut self) {
        let sprites = [
            CHAR_0, CHAR_1, CHAR_2, CHAR_3,
            CHAR_4, CHAR_5, CHAR_6, CHAR_7,
            CHAR_8, CHAR_9, CHAR_A, CHAR_B,
            CHAR_C, CHAR_D, CHAR_E, CHAR_F,
        ];

        let mut i = 0;
        for sprite in sprites.iter() {
            for byte in sprite {
                self.memory[i] = *byte;
                i += 1;
            }
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        if address as usize >= self.memory.len() {
            panic!("ram: access out of bound");
        }

        self.memory[address as usize] = value;
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        if address as usize >= self.memory.len() {
            panic!("ram: access out ot bound");
        }
        self.memory[address as usize]
    }

    pub fn read_bytes(&self, from: u16, size: u16) -> &[u8] {
        if from as usize >= self.memory.len() {
            panic!("ram: access out of bound");
        }

        &self.memory[from as usize..(from + size + 1) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ram_init_success() {
        let ram = RAM::new();
        assert_eq!(&ram.memory[0..5], CHAR_0);
    }

    #[test]
    fn test_ram_write_byte_success() {
        let mut ram = RAM::new();
        ram.write_byte(0x3FF, 0xFF);
        assert_eq!(ram.memory[0x3FF], 0xFF);
    }

    #[test]
    #[should_panic]
    fn test_ram_write_byte_outofbound_panic() {
        let mut ram = RAM::new();
        ram.write_byte(0xFFFF, 0xFF);
    }

    #[test]
    fn test_ram_read_byte_success() {
        let mut ram = RAM::new();
        ram.write_byte(0x3FF, 0xFF);
        assert_eq!(ram.memory[0x3FF], 0xFF);
    }
 
    #[test]
    #[should_panic]
    fn test_ram_read_byte_outofbound_panic() {
        let ram = RAM::new();
        ram.read_byte(0xFFFF);
    }

    #[test]
    fn test_ram_read_bytes_success() {
        let mut ram = RAM::new();
        ram.write_byte(0x1, 1);
        ram.write_byte(0x2, 2);
        ram.write_byte(0x3, 3);
        let bytes = ram.read_bytes(0x1, 2);
        assert_eq!(bytes, [1,2,3]);
    }

    #[test]
    #[should_panic]
    fn test_ram_read_bytes_outofbound_failed() {
        let ram = RAM::new();
        ram.read_bytes(0xFFFF, 3);
    }
}