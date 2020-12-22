use crate::vm::instruction::Opcode;

#[derive(Debug, PartialEq, Clone)]
pub struct VM {
    registers: [i32; 32],
    program: Vec<u8>,
    heap: Vec<u8>,
    pc: usize,
    remainder: i32,
    eq: bool,
}

impl VM {
    pub fn new(prog: Vec<u8>) -> Self {
        VM {
            registers: [0; 32],
            program: prog,
            heap: Vec::new(),
            pc: 0,
            remainder: 0,
            eq: false,
        }
    }

    pub fn run(&mut self) -> Result<(), ()> {
        while !self.execute()? {}
        Ok(())
    }

    #[inline]
    fn execute(&mut self) -> Result<bool, ()> {
        if self.pc >= self.program.len() {
            return Err(())
        }
        match self.decode_opcode() {
            Opcode::Hlt => {
                println!("Halting VM");
                Ok(true)
            }
            Opcode::Mov => { //todo: enable support for pointers
                //let flag = self.next_8_bits() as usize;
                let register = self.next_8_bits() as usize;
                let value = self.next_16_bits() as i32;
                self.registers[register] = value;

                Ok(false)
            }
            Opcode::Jmp => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;

                Ok(false)
            }
            Opcode::Jpf => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc += target as usize;

                Ok(false)
            }
            Opcode::Jpb => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc -= target as usize;

                Ok(false)
            }
            Opcode::Cmp => {
                let lhs = self.registers[self.next_8_bits() as usize];
                let rhs = self.registers[self.next_8_bits() as usize];
                self.eq = lhs == rhs;

                Ok(false)
            }
            Opcode::Lt => {
                let lhs = self.registers[self.next_8_bits() as usize];
                let rhs = self.registers[self.next_8_bits() as usize];
                self.eq = lhs < rhs;

                Ok(false)
            }
            Opcode::Gt => {
                let lhs = self.registers[self.next_8_bits() as usize];
                let rhs = self.registers[self.next_8_bits() as usize];
                self.eq = lhs > rhs;

                Ok(false)
            }
            Opcode::Le => {
                let lhs = self.registers[self.next_8_bits() as usize];
                let rhs = self.registers[self.next_8_bits() as usize];
                self.eq = lhs <= rhs;

                Ok(false)
            }
            Opcode::Ge => {
                let lhs = self.registers[self.next_8_bits() as usize];
                let rhs = self.registers[self.next_8_bits() as usize];
                self.eq = lhs >= rhs;

                Ok(false)
            }
            Opcode::Jeq => {
                let target = self.registers[self.next_8_bits() as usize];
                if self.eq {
                    self.pc = target as usize;
                }

                Ok(false)
            }
            Opcode::Jne => {
                let target = self.registers[self.next_8_bits() as usize];
                if !self.eq {
                    self.pc = target as usize;
                }

                Ok(false)
            }
            //Aloc and Dalc go here
            Opcode::Add => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = reg1 + reg2;

                Ok(false)
            }
            Opcode::Sub => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = reg1 - reg2;

                Ok(false)
            }
            Opcode::Mul => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = reg1 * reg2;

                Ok(false)
            }
            Opcode::Div => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = reg1 / reg2;
                self.remainder = reg1 % reg2;

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

enum Eq {
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    No,
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
            heap: vec![],
            pc: 0,
            remainder: 0,
            eq: false,
            }, 
            test_vm)
    }

    #[test]
    fn test_load_opcode() {
        let mut test_code: Vec<u8> = vec![0x01, 0x02];

        u16_to_little_endian(500, &mut test_code);
        test_code.push(0x00);
        /*
        The above code means:
        mov %2 500 (0x01 (mov), 0x02 (register 2), 1, 244, (500 in little endian))
        hlt (0x00)
        */
        let mut test_vm = VM::new(test_code);

        test_vm.run().unwrap();
        test_vm.dump_registers();
        assert_eq!(test_vm.test_register(2).unwrap(), 500);
    }

    #[test]
    fn test_add_opcode() {
        // 0x01, 0x01, 250 -> mov $0x01 250
        let mut test_code: Vec<u8> = vec![0x01, 0x01];
        u16_to_little_endian(250, &mut test_code);

        // 0x01, 0x02, 250 -> mov $0x02 250
        test_code.extend(vec![0x01, 0x02]);
        u16_to_little_endian(250, &mut test_code);

        // add $0x01 $0x02 $0x03
        test_code.extend(vec![0x10, 0x01, 0x02, 0x03]);

        // hlt
        test_code.push(0x00);

        let mut test_vm = VM::new(test_code);
        test_vm.run().unwrap();
        
        assert_eq!(test_vm.test_register(3).unwrap(), 500);
    }

    // helper function to convert a u16 to a little-endian byte repr
    // and then push the bytes to the program
    fn u16_to_little_endian(input: u16, prog: &mut Vec<u8>) {
        let upper: u8 = (input >> 8) as u8;
        let lower: u8 = (input & 0xff) as u8;
        prog.push(upper);
        prog.push(lower);
    }
}