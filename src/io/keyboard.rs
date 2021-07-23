
const KEY: [u8; 16] = [
    0x1, 0x2, 0x3, 0xC,
    0x4, 0x5, 0x6, 0xD,
    0x7, 0x8, 0x9, 0xE,
    0xA, 0x0, 0xB, 0xF
];

pub struct Keyboard {
    key: u8,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            key: 0,
        }
    }
    pub fn press(&mut self, value: u8) {
        for key in KEY {
            if value == key {
                self.key = value;
                break;
            }
        }
    }
}
