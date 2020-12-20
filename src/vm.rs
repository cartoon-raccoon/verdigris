use crate::instruction::Opcode;

#[derive(Debug, PartialEq, Clone)]
pub struct VM {
    registers: [i32; 32],
    program: Vec<u8>,
    pcounter: usize,
}

impl VM {
    pub fn new() -> Self {
        VM {
            registers: [0; 32],
            program: Vec::new(),
            pcounter: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), ()> {
        loop {
            if self.pcounter >= self.program.len() {
                return Err(())
            }
            match self.decode_opcode() {
                Opcode::Hlt => {
                    println!("Halt opcode detected");
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
    } 

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pcounter]);
        self.pcounter += 1;

        opcode
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vm_init() {
        let test_vm = VM::new();
        assert_eq!(VM {
            registers: [0; 32], 
            program: vec![],
            pcounter: 0
            }, 
            test_vm)
    }
}