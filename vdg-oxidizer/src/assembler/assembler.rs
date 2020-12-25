use std::collections::HashMap;

use crate::assembler::parser::Parsed;

use crate::instruction::Instruction;

#[derive(Debug, Clone)]
pub struct Assembler {
    phase: Phase,
    program: Vec<Parsed>,
    symbols: HashMap<String, Symbol>,
}

impl Assembler {
    pub fn new() -> Self {
        Assembler {
            phase: Phase::First,
            program: Vec::new(),
            symbols: HashMap::new(),
        }
    }
    pub fn assemble(&mut self) -> Vec<u8> {
        self.first_phase().unwrap();
        self.phase = Phase::Second;
        self.second_phase()
    }

    fn first_phase(&mut self) -> Result<(), ()> {
        // do stuff
        unimplemented!()
    }

    fn second_phase(&mut self) -> Vec<u8> {
        unimplemented!()
    }
}

#[derive(Debug, Clone, Copy)]
enum Phase {
    First,
    Second,
}

#[derive(Debug, Clone)]
pub enum AsmInst {
    Instruction(Instruction),
    Directive,
    Label,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Label {
    Code,
    Data,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    name: String,
    kind: Label,
    data: Vec<Parsed>
}