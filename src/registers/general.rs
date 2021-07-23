const LIMIT: u8 = 15;

pub struct Register {
    memory: [u8; LIMIT as usize],
    vf: u8,
}

impl Register {
    pub fn new() -> Register {
        Register { 
            memory: [0; LIMIT as usize],
            vf: 0,
        }
    }

    pub fn get(&mut self, index: u8) -> u8 {
        if index > LIMIT as u8 {
            panic!("register index out of bound.");
        }
        self.memory[index as usize]
    }

    pub fn set(&mut self, index: u8, value: u8) {
        if index > LIMIT as u8 {
            panic!("register index out of bound.");
        }
        self.memory[index as usize] = value;
    }

    pub fn carry_bit_on(&mut self) {
        self.vf = 1;
    }

    pub fn carry_bit_off(&mut self) {
        self.vf = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_generic_set_success() {
        let mut reg = Register::new();
        reg.set(4, 10);
        assert_eq!(reg.memory[4], 10);
    }

    #[test]
    #[should_panic]
    fn test_register_generic_set_index_outofbound_panic() {
        let mut reg = Register::new();
        reg.set(255, 10);
    }

    #[test]
    fn test_register_generic_get_index_sucess() {
        let mut reg = Register::new();
        reg.set(4, 10);
        let k = reg.get(4);
        assert_eq!(k, 10);
    }

    #[test]
    #[should_panic]
    fn test_register_generic_get_index_outofbound_panic() {
        let mut reg = Register::new();
        reg.get(255);
    }
}