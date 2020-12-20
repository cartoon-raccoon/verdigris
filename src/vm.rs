pub struct VM {
    registers: [u8; 32],
}

impl VM {
    pub fn new() -> Self {
        VM {
            registers: [0; 32]
        }
    }
}