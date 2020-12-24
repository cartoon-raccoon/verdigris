use std::collections::HashMap;

use crate::instruction::Instruction;

pub struct Assembler {
    phase: Phase,
    symbols: HashMap<String, Instruction>,
}

enum Phase {
    First,
    Second,
}