const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Screen {
    matrix: [[u8; WIDTH]; HEIGHT],
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            matrix: [[0; WIDTH]; HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.matrix = [[0; WIDTH]; HEIGHT];
    }

    pub fn draw_byte(&mut self, byte: u8, x: u8, y: u8) {}
}
