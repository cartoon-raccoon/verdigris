use std::convert::TryInto;
use byteorder::*;

use crate::vm::instruction::Opcode;

#[derive(Debug, Clone)]
pub struct VM {
    registers: [i32; 32],
    program: Vec<u8>,
    memory: VMMemory,
    pc: usize,
    remainder: i32,
    eq: bool,
}

impl VM {
    pub fn new(prog: Vec<u8>) -> Self {
        VM {
            registers: [0; 32],
            program: prog,
            memory: VMMemory::new(),
            pc: 0,
            remainder: 0,
            eq: false,
        }
    }

    pub fn run(&mut self) -> Result<(), VMError> {
        while !self.execute()? {}
        Ok(())
    }

    pub fn run_once(&mut self) -> Result<bool, VMError> {
        self.execute()
    }

    #[inline]
    #[allow(unreachable_code)]
    fn execute(&mut self) -> Result<bool, VMError> {
        if self.pc >= self.program.len() {
            return Err(VMError::SegFault)
        }
        match self.decode_opcode() {
            Opcode::Hlt => {
                println!("Halting VM");
                Ok(true)
            }
            Opcode::Mov => { //todo: enable support for pointers
                let register = self.next_8_bits() as usize;
                let flag = self.next_8_bits() as usize;
                let value = if flag == 0 {
                    self.read_i32()
                } else if flag == 1 {
                    unimplemented!("pointer calling conventions not implemented");
                    self.read_i64();
                } else if flag == 2 {
                    self.registers[self.next_8_bits() as usize]
                } else {
                    eprintln!("Error decoding opcode");
                    return Err(VMError::OpcodeErr)
                };
                self.registers[register] = value;

                Ok(false)
            }
            Opcode::Jmp => {
                unimplemented!()
            }
            Opcode::Jmpf => {
                unimplemented!()
            }
            Opcode::Jmpb => {
                unimplemented!()
            }
            Opcode::Cmp => {
                unimplemented!()
            }
            Opcode::Lt => {
                unimplemented!()
            }
            Opcode::Gt => {
                unimplemented!()
            }
            Opcode::Le => {
                unimplemented!()
            }
            Opcode::Ge => {
                unimplemented!()
            }
            Opcode::Jeq => {
                unimplemented!()
            }
            Opcode::Jne => {
                unimplemented!()
            }
            Opcode::Aloc => {
                let flag = self.next_8_bits();
                let value = if flag == 0 {
                    self.read_i32()
                } else if flag == 1 {
                    unimplemented!("pointer size not yet worked out");
                    self.read_i64();
                } else if flag == 2 {
                    self.registers[self.next_8_bits() as usize]
                } else {
                    return Err(VMError::OpcodeErr)
                };
                self.memory.allocate_heap(value as usize);
                Ok(false)
            }
            Opcode::Push => {
                unimplemented!("Calling conventions not yet specified")
            }
            Opcode::Pop => {
                unimplemented!("Calling conventions not yet specified")
            }
            Opcode::Call => {
                unimplemented!("Calling conventions not yet specified")
            }
            Opcode::Ret => {
                unimplemented!("Calling conventions not yet specified")
            }
            Opcode::Dalc => {
                unimplemented!()
            }
            Opcode::Add => {
                unimplemented!()
            }
            Opcode::Sub => {
                unimplemented!()
            }
            Opcode::Mul => {
                unimplemented!()
            }
            Opcode::Div => {
                unimplemented!()
            }
            Opcode::Igl => {
                return Err(VMError::IglOpcode)
            }
            oc @ _ => {
                unimplemented!("{:?} opcode not yet supported", oc)
            }
        }
    }

    #[inline]
    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;

        opcode
    }

    /// Gets the next byte in the program as an operand
    fn next_8_bits(&mut self) -> u8 {
        let byte = self.program[self.pc];
        self.pc += 1;

        byte
    }

    fn read_i32(&mut self) -> i32 {
        let buf: [u8; 4] = self.program[self.pc..self.pc + 4].try_into().unwrap();
        self.pc += 4;
        LittleEndian::read_i32(&buf)
    }

    fn read_i64(&mut self) -> i64 {
        let buf: [u8; 8] = self.program[self.pc..self.pc + 8].try_into().unwrap();
        self.pc += 8;
        LittleEndian::read_i64(&buf)
    }

    pub fn add_bytes(&mut self, bytes: Vec<u8>) {
        self.program.extend(bytes);
    }

    pub fn dump_registers(&self) {
        println!("Register dump for Oxidizer VM");
        for i in 0..32 {
            println!("{:02}: {}", i, self.registers[i]);
        }
        println!("End of register dump")
    }

    pub fn dump_program(&self) {
        println!("Dumping loaded program vector");
        println!("{:?}", self.program);
        println!("End of program dump")
    }

    pub fn heap(&self) -> usize {
        self.memory.size()
    }

    #[cfg(test)]
    pub fn test_register(&self, reg: usize) -> Result<i32, ()> {
        if reg > 31 {
            return Err(())
        }

        Ok(self.registers[reg])
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VMError {
    IglOpcode,
    SegFault,
    OpcodeErr,
}

impl std::error::Error for VMError {}

impl std::fmt::Display for VMError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::IglOpcode => {
                write!(f, "VM Error: illegal opcode encountered")
            }
            Self::SegFault => {
                write!(f, "VM Error: illegal memory access")
            }
            Self::OpcodeErr => {
                write!(f, "VM Error: error while processing opcode")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct VMMemory {
    heap: Vec<u8>,
    stack: Vec<u8>,
    topptr: usize,
    baseptr: usize,
    heap_size: usize,
}

impl VMMemory {
    pub fn new() -> Self {
        Self {
            heap: Vec::new(),
            stack: Vec::new(),
            topptr: 0,
            baseptr: 0,
            heap_size: 0,
        }
    }

    pub fn allocate_heap(&mut self, size: usize) -> usize {
        let target = self.heap.len() + size;
        self.heap.resize(target, 0);
        self.heap_size = target;
        target
    }

    pub fn allocate_stack(&mut self, size: usize) -> (usize, usize) {
        let target = self.topptr + size;
        self.stack.resize(target, 0);
        self.topptr += size;
        assert!(self.stack.len() == self.topptr, "SP and stack size mismatch");
        (self.baseptr, self.topptr)
    }

    pub fn push_stack(&mut self, size: usize) -> (usize, usize) {
        self.allocate_stack(size);
        //todo: save baseptr to top of stack
        self.baseptr += size;
        (self.baseptr, self.topptr)
    }

    pub fn pop_stack(&mut self) -> (usize, usize) {
        unimplemented!(
            "need to implement saving base ptr first"
        )
    }

    pub fn size(&self) -> usize {
        self.heap_size + self.topptr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vm_init() {
        let test_vm = VM::new(vec![]);
        assert_eq!(VM {
            registers: [0; 32], 
            program: vec![],
            memory: VMMemory::new(),
            pc: 0,
            remainder: 0,
            eq: false,
            }, 
            test_vm)
    }

    #[test]
    fn test_mov_opcode() {
        let mut test_code: Vec<u8> = vec![0x01, 0x02, 0x00];

        test_code.extend(i32_to_bytes(500).to_vec());
        test_code.push(0x00);
        /*
        The above code means:
        mov $2 500 (0x01 (mov), 0x02 (register 2), flag LIT (0), (500 in little endian))
        hlt (0x00)
        */
        let mut test_vm = VM::new(test_code);

        test_vm.run().unwrap();
        test_vm.dump_registers();
        assert_eq!(test_vm.test_register(2).unwrap(), 500);
    }

    #[test]
    fn test_aloc_opcode() {
        // mov $2 10
        let mut test_code: Vec<u8> = vec![0x01, 0x02, 0x00];
        test_code.extend(i32_to_bytes(10).to_vec());
        // aloc $2
        test_code.extend(vec![0x0c, 0x02, 0x02]);
        // hlt
        test_code.push(0x00);

        let mut test_vm = VM::new(test_code);

        test_vm.run().unwrap();
        assert_eq!(test_vm.heap(), 10)
    }

    fn i32_to_bytes(num: i32) -> [u8; 4] {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        buf.as_mut().write_i32::<LittleEndian>(num).unwrap();

        buf
    }
}