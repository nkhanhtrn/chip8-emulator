pub struct RegisterI {
    value: u16,
}

impl RegisterI {
    pub fn new() -> RegisterI {
        RegisterI { 
            value: 0,
        }
    }

    pub fn set(&mut self, value: u16) {
        self.value = value;
    }

    pub fn get(&self) -> u16 {
        self.value
    }
}
