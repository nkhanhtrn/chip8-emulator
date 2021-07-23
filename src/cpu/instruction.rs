pub struct Instruction {
    pub op: u8,
    pub x: u8,
    pub y: u8,
    pub n: u8,
    pub nnn: u16,
    pub kk: u8,
}


impl Instruction {
    pub fn new(high_byte: u16, low_byte: u16) -> Instruction {
        // first 8 byte of hi + lo => u16 (Big-Endian)
        let instruction: u16 = (high_byte << 8) | low_byte;

        // basic opcodes: (op, x, y, n)
        let op = (instruction >> 12) as u8;
        let x = ((instruction & 0x0F00) >> 8) as u8;
        let y = ((instruction & 0x00F0) >> 4) as u8;
        let n = (instruction & 0x000F) as u8;
        // extra content
        let nnn = (instruction & 0x0FFF) as u16;
        let kk = (instruction & 0x00FF) as u8;

        Instruction { op, x, y, n, nnn, kk }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_new_success() {
        // 0x81A2
        let high_byte = 0x81;
        let low_byte = 0xA2;
        let ins = Instruction::new(high_byte, low_byte);
        assert_eq!(ins.op, 0x8);
        assert_eq!(ins.x, 0x1);
        assert_eq!(ins.y, 0xA);
        assert_eq!(ins.n, 0x2);
        assert_eq!(ins.nnn, 0x1A2);
        assert_eq!(ins.kk, 0xA2);
    }
}