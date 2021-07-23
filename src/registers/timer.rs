/*
use crate::io::sound::Sound;
trait Timer {
    counter = u8
}

impl Timer {
    fn set_counter(&mut self, new_counter: u8) {
        self.counter = new_counter;
        if self.counter > 0 {
            duration: usize = self.counter / 60;
            self.counter -= 1;
        }
    }
}
impl Default for Timer {
    fn default() -> Timer {
        Timer {
            counter: 0
        }
    }
}

#[derive(Default)]
pub struct DelayTimer impl Timer {}

#[derive(Default)]
pub struct SoundTimer impl DelayTimer {
    sound: &Sound
}
*/