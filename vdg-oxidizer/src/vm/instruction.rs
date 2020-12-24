use crate::assembler::Operand;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    Hlt,  // Halt execution
    Mov,  // Load register
    Jmp,  // Jump to a location in program
    Jmpf, // Jump forward by x bytes
    Jmpb, // Jump backward by x bytes
    Cmp,  // Compare and set flag if equal
    Lt,   // Compare and set flag if lhs < rhs
    Gt,   // Compare and set flag if lhs > rhs
    Le,   // Compare and set flag if lhs <= rhs
    Ge,   // Compare and set flag if lhs >= rhs
    Jeq,  // Jump if flag is set
    Jne,  // Jump if flag is not set
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
            0x03 => Opcode::Jmpf,
            0x04 => Opcode::Jmpb,
            0x05 => Opcode::Cmp,
            0x06 => Opcode::Lt,
            0x07 => Opcode::Gt,
            0x08 => Opcode::Le,
            0x09 => Opcode::Ge,
            0x0a => Opcode::Jeq,
            0x0b => Opcode::Jne,
            0x0c => Opcode::Aloc,
            0x0d => Opcode::Dalc,
            0x10 => Opcode::Add,
            0x11 => Opcode::Sub,
            0x12 => Opcode::Mul,
            0x13 => Opcode::Div,
            _ => Opcode::Igl,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub inst: Opcode,
    pub op1: Option<Operand>,
    pub op2: Option<Operand>,
    pub op3: Option<Operand>,
}


impl Instruction {
    pub fn new(byte: u8) -> Self {
        Instruction {
            inst: Opcode::from(byte),
            op1: None,
            op2: None,
            op3: None,
        }
    }

    pub fn from_parsed(
        opcode: Opcode, 
        tokens: (
            Option<Operand>, 
            Option<Operand>, 
            Option<Operand>)
        ) -> Self {
        Self {
            inst: opcode,
            op1: tokens.0,
            op2: tokens.1,
            op3: tokens.2,
        }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        unimplemented!()
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

