use crate::assembler::Operand;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    //* Standard opcodes
    Hlt,  // Halt execution
    Mov,  // Load register
    Jmp,  // Jump to a location in program
    Jmpf, // Jump forward by x bytes
    Jmpb, // Jump backward by x bytes

    //* Comparison and conditional jumps
    Cmp,  // Compare and set flag if equal
    Lt,   // Compare and set flag if lhs < rhs
    Gt,   // Compare and set flag if lhs > rhs
    Le,   // Compare and set flag if lhs <= rhs
    Ge,   // Compare and set flag if lhs >= rhs
    Jeq,  // Jump if flag is set
    Jne,  // Jump if flag is not set

    //* Memory Management
    Aloc, // Allocate some memory on the heap
    Dalc, // Deallocate the memory on the heap

    //* Function operations
    Push, // Push onto the stack
    Pop,  // Pop from the stack
    Call, // Call a label or routine
    Ret,  // Return

    //* I/O operations
    Prt,  // Print a bytestream (write to stdout)
    Open, // Opens a file and stores it as a raw FD
    Clse, // Closes given FD
    Read, // Read from a file desc
    Wrt,  // Write to a file desc

    //* Numerical and bitwise operations
    Inc,  // Increment
    Dec,  // Decrement
    Add,  // Add
    Sub,  // Subtract
    Mul,  // Multiply
    Div,  // Divide
    And,  // Bitwise and
    Not,  // Bitwise not
    Or,   // Bitwise or
    Xor,  // Bitwise xor
    Bsl,  // Bitshift left
    Bsr,  // Bitshift right

    //* Illegal
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

            0x0e => Opcode::Push,
            0x0f => Opcode::Pop,
            0x10 => Opcode::Call,
            0x11 => Opcode::Ret,

            0x12 => Opcode::Prt,
            0x13 => Opcode::Open,
            0x14 => Opcode::Clse,
            0x15 => Opcode::Read,
            0x16 => Opcode::Wrt,

            0x20 => Opcode::Inc,
            0x21 => Opcode::Dec,
            0x22 => Opcode::Add,
            0x23 => Opcode::Sub,
            0x24 => Opcode::Mul,
            0x25 => Opcode::Div,
            0x26 => Opcode::And,
            0x27 => Opcode::Not,
            0x28 => Opcode::Or,
            0x29 => Opcode::Xor,
            0x2a => Opcode::Bsl,
            0x2b => Opcode::Bsr,
            _    => Opcode::Igl,
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

