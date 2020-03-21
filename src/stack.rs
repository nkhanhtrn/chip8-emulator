pub struct Stack {
    memory: Vec<u16>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            memory: Vec::<u16>::new(),
        }
    }

    pub fn push(&mut self, value: u16) {
        self.memory.push(value);
    }

    pub fn pop(&mut self) -> u16 {
        self.memory.pop().unwrap()
    }
}
