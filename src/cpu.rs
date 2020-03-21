use ram::RAM;

public struct CPU {
    opcode: u16,
    v: [u8; 16],
    i: u16,
    sound_timer: u8,
    delay_timer: u8,
    pc: u16,
    sp: u8,
    ram: RAM
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
        opcode: 0,
        v: [0; 16],
        i: 0x200,
        sound_timer: 0,
        delay_timer: 0,
        pc: 0x200,
        sp: 0,
        ram: [0; 4096],
        running: false
        }
    }

    pub fn run_program(mut self, program: &mut Vec<u8>) {
        self.start();
        self.load_program(&mut program);

        while self.running {
            self.run_opcode();
            self.next();
        }
    }

    fn start(mut self) {
        self.running = true;
    }

    fn load_program(mut self, program: Vec<u8>) {
        let offset = 0x200;
        for i in 0..program.len() {
            self.ram.write_byte((offset + i) as u16, program[i]);
        }
    }

    fn run_opcode(&mut self) {
        let word: u16 = &self.memory[&self.pc];
        let opcode = (word >> 12) as u8;

        match opcode {
            0x13 => {
                
            }
        }
    }
}