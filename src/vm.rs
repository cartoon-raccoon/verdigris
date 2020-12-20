use crate::instruction::Opcode;

#[derive(Debug, PartialEq, Clone)]
pub struct VM {
    registers: [i32; 32],
    program: Vec<u8>,
    pc: usize,
}

impl VM {
    pub fn new(prog: Vec<u8>) -> Self {
        VM {
            registers: [0; 32],
            program: prog,
            pc: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), ()> {
        while self.execute()? {}
        Ok(())
    }

    fn execute(&mut self) -> Result<bool, ()> {
        if self.pc >= self.program.len() {
            return Err(())
        }
        match self.decode_opcode() {
            Opcode::Hlt => {
                println!("Halting VM");
                Ok(true)
            }
            Opcode::Mov => {
                let register = self.next_8_bits() as usize;
                let value = self.next_16_bits() as i32;
                self.registers[register] = value;

                Ok(false)
            }
            Opcode::Igl => {
                eprintln!("Illegal opcode");
                return Err(())
            }
            oc @ _ => {
                eprintln!("{:?} opcode not yet supported", oc);
                return Err(())
            }
        }
    }

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

    /// Gets the next 16 bits as a u16
    fn next_16_bits(&mut self) -> u16 {
        let res = ((self.program[self.pc] as u16) << 8) 
            | self.program[self.pc + 1] as u16; 
        self.pc += 2;

        res
    }

    #[cfg(test)]
    pub fn dump_registers(&self) {
        println!("{:?}", self.registers);
    }

    #[cfg(test)]
    pub fn test_register(&self, reg: usize) -> Result<i32, ()> {
        if reg > 31 {
            return Err(())
        }

        Ok(self.registers[reg])
    }

    #[cfg(test)]
    pub fn run_once(&mut self) {
        self.execute().unwrap();
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
            pc: 0
            }, 
            test_vm)
    }

    #[test]
    fn test_vm_opcodes() {
        let test_code: Vec<u8> = vec![0x01, 0x02, 1, 244, 0x00];
        let mut test_vm = VM::new(test_code);

        test_vm.run().unwrap();
        test_vm.dump_registers();
        assert_eq!(test_vm.test_register(2).unwrap(), 500);
    }
}