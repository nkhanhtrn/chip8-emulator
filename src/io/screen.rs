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

    pub fn get_screen(&self) -> &[[u8; WIDTH]; HEIGHT] {
        &self.matrix
    }

    pub fn draw_bytes(&mut self, bytes: &[u8], x: usize, y: usize) -> Result<(), &str> {
        if x >= WIDTH || y >= HEIGHT {
            return Err("screen matrix out of bound");
        }
        let mut c = 0;
        let mut x = x;
        let mut y = y;

        while c < bytes.len() {
            self.matrix[y][x] ^= bytes[c];
            x = x + 1;
            if x >= WIDTH {
                x = 0;
                y = if y == HEIGHT {y} else { y + 1 };
            }
            c += 1;
        }
        Ok(())
    }

    pub fn clear(&mut self) {
        self.matrix = [[0; WIDTH]; HEIGHT];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_draw_bytes_success() {
        let mut screen = Screen::new();
        screen.draw_bytes(&[1,2,3], 0, 5).unwrap();
        assert_eq!(screen.matrix[5][0], 1);
        assert_eq!(screen.matrix[5][1], 2);
        assert_eq!(screen.matrix[5][2], 3);
    }

    #[test]
    fn test_screen_draw_bytes_outofbound_failed() {
        let mut screen = Screen::new();
        if let Err(e) = screen.draw_bytes(&[1,2,3], 0, 99999999) {
            assert_eq!("screen matrix out of bound", e);
        }
    }

    #[test]
    fn test_screen_clear_success() {
        let mut screen = Screen::new();
        screen.draw_bytes(&[1,2,3], 0, 0).unwrap();
        screen.clear();
        assert_eq!(screen.matrix[0][0], 0);
    }
}