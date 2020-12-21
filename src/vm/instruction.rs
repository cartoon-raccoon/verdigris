#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    Hlt,  // Halt execution
    Mov,  // Load register
    Jmp,  // Jump to a location in program
    Jpf,  // Jump forward by x bytes
    Jpb,  // Jump backward by x bytes
    Jeq,  // Jump if equal
    Jne,  // Jump if not equal
    Cmp,  // Compare two registers
    Aloc, // Allocate some memory on the heap
    Dalc, // Deallocate the memory on the heap
    Add,  // Add
    Sub,  // Subtract
    Mul,  // Multiply
    Div,  // Divide
    Igl,  // Illegal
}

impl From<u8> for Opcode {
    fn from(from: u8) -> Self {
        match from {
            0x00 => Opcode::Hlt,
            0x01 => Opcode::Mov,
            0x02 => Opcode::Jmp,
            0x07 => Opcode::Add,
            0x08 => Opcode::Sub,
            0x09 => Opcode::Mul,
            0x0a => Opcode::Div,
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
        let hlt = 0x00;
        let mov = 0x01;
        let igl = 0xf7;

        let hltopcode = Opcode::from(hlt);
        let movopcode = Opcode::from(mov);
        let iglopcode = Opcode::from(igl);
        assert_eq!(hltopcode, Opcode::Hlt);
        assert_eq!(movopcode, Opcode::Mov);
        assert_eq!(iglopcode, Opcode::Igl);
    }
}

