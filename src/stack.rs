pub struct Stack {
    memory: Vec<u16>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { memory: 0 }
    }

    pub fn pop(&mut self) -> u16 {
        self.memory.pop().unwrap()
    }
}
