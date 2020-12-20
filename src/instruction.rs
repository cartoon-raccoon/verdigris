#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    Hlt, // Halt
    Add, // Add
    Sub, // Subtract
    Jmp, // Jump to a location in program
    Jeq, // Jump if equal
    Jne, // Jump if not equal
    Igl, // Illegal
}

impl From<u8> for Opcode {
    fn from(from: u8) -> Self {
        match from {
            0x06 => Opcode::Hlt,
            _ => Opcode::Igl,
        }
    }
}

pub struct Instruction {
    inst: Opcode,
}

impl Instruction {
    pub fn new(byte: u8) -> Self {
        Instruction {
            inst: Opcode::from(byte),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_opcode_from() {
        let hlt = 0x06;
        let igl = 0x01;
        let hltopcode = Opcode::from(hlt);
        let iglopcode = Opcode::from(igl);
        assert_eq!(hltopcode, Opcode::Hlt);
        assert_eq!(iglopcode, Opcode::Igl);
    }
}

