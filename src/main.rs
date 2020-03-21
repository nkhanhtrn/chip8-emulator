use ram::RAM;
use cpu::CPU;

fn main() {
    let cpu = CPU::new();
    let mut program = vec![0x13, 0xC5];
    cpu.run_program(&mut program);
}
