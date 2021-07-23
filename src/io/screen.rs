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

    pub fn draw(&mut self, sprite: &[u8], x: isize, y: isize) -> bool {
        let mut collision = false;
        let width = WIDTH as isize;
        let height = HEIGHT as isize;
 
        let mut i: isize = (width + x) % width;
        let mut j: isize = (height + y) % height;
        for byte in sprite.iter() {
            let bit_before = self.matrix[j as usize][i as usize];
            let bit_after = bit_before ^ byte;
            if bit_before != bit_after && bit_before == 1 {
                collision = true;
            }
            self.matrix[j as usize][i as usize] = bit_after;

            i = i + 1;
            if i == width {
                i = 0;
                j += 1;
                if j == width {
                    j = 0;
                }
            }
        }

        collision
    }

    pub fn clear(&mut self) {
        self.matrix = [[0; WIDTH]; HEIGHT];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_draw_success() {
        let mut screen = Screen::new();
        screen.draw(&[1,2,3], 0, 5);
        assert_eq!(screen.matrix[5][0], 1);
        assert_eq!(screen.matrix[5][1], 2);
        assert_eq!(screen.matrix[5][2], 3);
    }

    #[test]
    fn test_screen_draw_x_outbound_left_success() {
        let mut screen = Screen::new();
        screen.draw(&[1,2,3], -4, 5);
        assert_eq!(screen.matrix[5][WIDTH - 5], 1);
        assert_eq!(screen.matrix[5][WIDTH - 4], 2);
        assert_eq!(screen.matrix[5][WIDTH - 3], 3);
    }

    #[test]
    fn test_screen_draw_x_outbound_right_success() {
        let mut screen = Screen::new();
        screen.draw(&[1,2,3], (WIDTH + 4) as isize, 5);
        assert_eq!(screen.matrix[5][4], 1);
        assert_eq!(screen.matrix[5][5], 2);
        assert_eq!(screen.matrix[5][6], 3);
    }

    #[test]
    fn test_screen_draw_y_outbound_left_success() {
        let mut screen = Screen::new();
        screen.draw(&[1,2,3], 0, -5);
        assert_eq!(screen.matrix[HEIGHT - 6][0], 1);
        assert_eq!(screen.matrix[HEIGHT - 6][1], 2);
        assert_eq!(screen.matrix[HEIGHT - 6][2], 3);
    }

    #[test]
    fn test_screen_draw_bytes_y_outbound_right_success() {
        let mut screen = Screen::new();
        screen.draw(&[1,2,3], 0, (HEIGHT + 4) as isize);
        assert_eq!(screen.matrix[4][0], 1);
        assert_eq!(screen.matrix[4][1], 2);
        assert_eq!(screen.matrix[4][2], 3);
    }

 
    #[test]
    fn test_screen_draw_bytes_endofx_success() {
        let mut screen = Screen::new();
        screen.draw(&[1,2,3], (WIDTH - 1) as isize, 5);
        assert_eq!(screen.matrix[5][WIDTH - 1], 1);
        assert_eq!(screen.matrix[6][0], 2);
        assert_eq!(screen.matrix[6][1], 3);
    }

    #[test]
    fn test_screen_draw_bytes_endofy_success() {
        let mut screen = Screen::new();
        screen.draw(&[1,2,3], (WIDTH - 1) as isize, (HEIGHT - 1) as isize);
        assert_eq!(screen.matrix[HEIGHT - 1][WIDTH - 1], 1);
        assert_eq!(screen.matrix[0][0], 2);
        assert_eq!(screen.matrix[0][1], 3);
    }

    #[test]
    fn test_screen_draw_bytes_collision_success() {
        let mut screen = Screen::new();
        let collision = screen.draw(&[1,2,3], (WIDTH - 1) as isize, (HEIGHT - 1) as isize);
        assert_eq!(collision, true);
    }

    #[test]
    fn test_screen_clear_success() {
        let mut screen = Screen::new();
        screen.draw(&[1,2,3], 0, 0);
        screen.clear();
        assert_eq!(screen.matrix[0][0], 0);
    }
}