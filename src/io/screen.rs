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

    pub fn draw_bytes(&mut self, bytes: &[u8], x: isize, y: isize) -> Result<(), &str> {
        let mut c = 0;
        let width = WIDTH as isize;
        let height = HEIGHT as isize;

        let mut i: usize;
        if x < 0 { 
            i = (width + x - 1) as usize;
        } else if x >= width { 
            i = (x - width) as usize; 
        } else { 
            i = x as usize 
        }

        let mut j: usize;
        if y < 0 {
            j = (height + y - 1) as usize;
        } else if y >= height { 
            j = (y - height) as usize; 
        } else { 
            j = y as usize
        }

        while c < bytes.len() {
            self.matrix[j][i] ^= bytes[c];
            i = i + 1;
            if i == WIDTH {
                i = 0;
                j += 1;
                if j == HEIGHT {
                    j = 0;
                }
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
    fn test_screen_draw_bytes_x_outbound_left_success() {
        let mut screen = Screen::new();
        screen.draw_bytes(&[1,2,3], -4, 5).unwrap();
        assert_eq!(screen.matrix[5][WIDTH - 5], 1);
        assert_eq!(screen.matrix[5][WIDTH - 4], 2);
        assert_eq!(screen.matrix[5][WIDTH - 3], 3);
    }

    #[test]
    fn test_screen_draw_bytes_x_outbound_right_success() {
        let mut screen = Screen::new();
        screen.draw_bytes(&[1,2,3], (WIDTH + 4) as isize, 5).unwrap();
        assert_eq!(screen.matrix[5][4], 1);
        assert_eq!(screen.matrix[5][5], 2);
        assert_eq!(screen.matrix[5][6], 3);
    }

    #[test]
    fn test_screen_draw_bytes_y_outbound_left_success() {
        let mut screen = Screen::new();
        screen.draw_bytes(&[1,2,3], 0, -5).unwrap();
        assert_eq!(screen.matrix[HEIGHT - 6][0], 1);
        assert_eq!(screen.matrix[HEIGHT - 6][1], 2);
        assert_eq!(screen.matrix[HEIGHT - 6][2], 3);
    }

    #[test]
    fn test_screen_draw_bytes_y_outbound_right_success() {
        let mut screen = Screen::new();
        screen.draw_bytes(&[1,2,3], 0, (HEIGHT + 4) as isize).unwrap();
        assert_eq!(screen.matrix[4][0], 1);
        assert_eq!(screen.matrix[4][1], 2);
        assert_eq!(screen.matrix[4][2], 3);
    }

 
    #[test]
    fn test_screen_draw_bytes_endofx_success() {
        let mut screen = Screen::new();
        screen.draw_bytes(&[1,2,3], (WIDTH - 1) as isize, 5).unwrap();
        assert_eq!(screen.matrix[5][WIDTH - 1], 1);
        assert_eq!(screen.matrix[6][0], 2);
        assert_eq!(screen.matrix[6][1], 3);
    }

    #[test]
    fn test_screen_draw_bytes_endofy_success() {
        let mut screen = Screen::new();
        screen.draw_bytes(&[1,2,3], (WIDTH - 1) as isize, (HEIGHT - 1) as isize).unwrap();
        assert_eq!(screen.matrix[HEIGHT - 1][WIDTH - 1], 1);
        assert_eq!(screen.matrix[0][0], 2);
        assert_eq!(screen.matrix[0][1], 3);
    }
    #[test]
    fn test_screen_clear_success() {
        let mut screen = Screen::new();
        screen.draw_bytes(&[1,2,3], 0, 0).unwrap();
        screen.clear();
        assert_eq!(screen.matrix[0][0], 0);
    }
}