const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn get_index(x: u8, y: u8) -> usize {
    (y as usize) * WIDTH + (x as usize)
}

pub struct Screen {
    memory: [u8; WIDTH * HEIGHT],
}

impl Screen {
    pub fn new() -> Screen {
        Screen { 
            memory: [0; WIDTH * HEIGHT],
        }
    }

    pub fn memory(&self) -> *const u8 {
        self.memory.as_ptr()
    }

    pub fn draw(&mut self, sprite: &[u8], x: u8, y: u8) -> bool {
        let mut collision = false;

        let mut idx = get_index(x, y);
        idx = (WIDTH + idx) % WIDTH;
        
        for byte in sprite.iter() {
            let bit_before = self.memory[idx];
            let bit_after = bit_before ^ byte;
            if bit_before != bit_after && bit_before == 1 {
                collision = true;
            }
            self.memory[idx] = bit_after;
        }

        collision
    }

    pub fn clear(&mut self) {
        self.memory = [0; WIDTH * HEIGHT];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_get_index() {
        assert_eq!(WIDTH + 4, get_index(4, 1));
    }

    #[test]
    fn test_screen_draw_success() {
        let mut screen = Screen::new();
        screen.draw(&[1,2,3], 0, 5);
        let idx = get_index(0, 5);
        assert_eq!(screen.memory[idx], 1);
        assert_eq!(screen.memory[idx + 1], 2);
        assert_eq!(screen.memory[idx + 2], 3);
    }
/*
    #[test]
    fn test_screen_draw_x_outbound_left_success() {
        let mut screen = Screen::new();
        screen.draw(&[1,2,3], -4, 5);
        
        assert_eq!(screen.memory[5][WIDTH - 5], 1);
        assert_eq!(screen.memory[5][WIDTH - 4], 2);
        assert_eq!(screen.memory[5][WIDTH - 3], 3);
    }

    #[test]
    fn test_screen_draw_x_outbound_right_success() {
        let mut screen = Screen::new();
        screen.draw(&[1,2,3], (WIDTH + 4) as isize, 5);
        assert_eq!(screen.memory[5][4], 1);
        assert_eq!(screen.memory[5][5], 2);
        assert_eq!(screen.memory[5][6], 3);
    }

    #[test]
    fn test_screen_draw_y_outbound_left_success() {
        let mut screen = Screen::new();
        screen.draw(&[1,2,3], 0, -5);
        assert_eq!(screen.memory[HEIGHT - 6][0], 1);
        assert_eq!(screen.memory[HEIGHT - 6][1], 2);
        assert_eq!(screen.memory[HEIGHT - 6][2], 3);
    }

    #[test]
    fn test_screen_draw_bytes_y_outbound_right_success() {
        let mut screen = Screen::new();
        screen.draw(&[1,2,3], 0, (HEIGHT + 4) as isize);
        assert_eq!(screen.memory[4][0], 1);
        assert_eq!(screen.memory[4][1], 2);
        assert_eq!(screen.memory[4][2], 3);
    }
*/

    #[test]
    fn test_screen_draw_bytes_collision_success() {
        let mut screen = Screen::new();
        screen.draw(&[1,2,3], 0, 0);
        let collision = screen.draw(&[1,2,3], 0, 0);
        assert_eq!(collision, true);
    }

    #[test]
    fn test_screen_clear_success() {
        let mut screen = Screen::new();
        screen.draw(&[1,2,3], 0, 0);
        screen.clear();
        assert_eq!(screen.memory[0], 0);
    }
}