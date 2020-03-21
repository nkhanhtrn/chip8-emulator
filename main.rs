fn main() {
    let mut cpu = CPU::new();
    let mut program = vec![0x13, 0xC5];
    cpu.load_program(&mut program);
}

struct CPU {
    opcode: u16,
    v: [u8; 16],
    i: u16,
    sound_timer: u8,
    delay_timer: u8,
    pc: u16,
    sp: u8,
    memory: [u8; 4096]
}

impl CPU {
    fn new() -> CPU {
        CPU {
        opcode: 0,
        v: [0; 16],
        i: 0x200,
        sound_timer: 0,
        delay_timer: 0,
        pc: 0x200,
        sp: 0,
        memory: [0; 4096]
        }
    }

    fn load_program(&mut self, program: &mut Vec<u8>) {
        let mut data = vec![0; 0x200];
        let mut p = program;
        data.append(&mut p);

        for (index, &byte) in data.iter().enumerate() {
            self.memory[index] = byte;
        }
    }
}