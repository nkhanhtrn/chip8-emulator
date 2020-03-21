pub struct Input {
    key: u16,
}

impl Input {
    pub fn new() -> Input {
        Input { key: 0 }
    }

    pub fn set(&mut self, value: u16) {
        self.key = value;
    }

    pub fn key_press(&mut self, key: u16) -> bool {
        self.key == key
    }
}
