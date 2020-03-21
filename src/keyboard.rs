pub struct Keyboard {
    key: u8,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard { key: 0 }
    }

    pub fn set(&mut self, value: u8) {
        self.key = value;
    }

    pub fn key_press(&mut self, key: u8) -> bool {
        self.key == key
    }
}
