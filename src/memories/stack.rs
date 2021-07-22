const LIMIT: usize = 16;

/*
 * Used to store address of previous counter when calling subroutine
 */
pub struct Stack {
    memory: Vec<u16>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            memory: Vec::<u16>::new(),
        }
    }

    pub fn push(&mut self, value: u16) -> Result<(), &str> {
        if self.memory.len() == LIMIT {
            return Err("stack is full");
        }
        self.memory.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> u16 {
        self.memory.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_push_success() {
        let mut stack = Stack::new();
        stack.push(1).unwrap();
        assert_eq!(stack.memory[0], 1);
        stack.push(2).unwrap();
        assert_eq!(stack.memory[1], 2);
        stack.push(3).unwrap();
        assert_eq!(stack.memory[2], 3);
    }

    #[test]
    fn test_stack_push_limit_failed() {
        let mut stack = Stack::new();
        for i in 0..LIMIT {
            stack.push(i as u16).unwrap();
        }
        if let Err(e) = stack.push(1) {
            assert_eq!("stack is full", e);
        }
    }

    #[test]
    fn test_stack_pop_success() {
        let mut stack = Stack::new();
        stack.push(1).unwrap();
        stack.push(2).unwrap();
        stack.push(3).unwrap();
        let k = stack.pop();
        assert_eq!(k, 3);
        assert_eq!(stack.memory.len(), 2);
    }

    #[test]
    #[should_panic]
    fn test_stack_pop_empty_panic() {
        let mut stack = Stack::new();
        stack.pop();
    }
}