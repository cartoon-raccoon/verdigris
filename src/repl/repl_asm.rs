use linefeed::{
    Interface, 
    terminal::DefaultTerminal,
    reader::ReadResult,
};
use std::io;
use std::fmt;

use crate::vm::VM;

/// The repl for the low-level IR (assembly) of Verdigris.
pub struct ReplAsm {
    vm: VM,
    interface: Interface<DefaultTerminal>,
    lexer: AsmLexer,
}

impl ReplAsm {
    pub fn new() -> Self {
        let mut lr = Interface::new("vdg-asm").unwrap();
        lr.set_prompt("vdg-asm >>>").unwrap();
        Self {
            vm: VM::new(vec![]),
            interface: lr,
            lexer: AsmLexer::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.read_line() {
                Ok(line) => {
                    unimplemented!()
                }
                Err(e) => {
                    eprintln!("{}", e);
                    continue
                }
            }
        }
    }

    pub fn read_line(&mut self) -> Result<String, AsmLexErr> {
        if let ReadResult::Input(line) = self.interface.read_line()? {
            self.lexer.parse(&line);
        }
        unimplemented!()
    }
}

struct AsmLexer {
    pos: usize,
    state: Token,
}

impl AsmLexer {
    pub fn new() -> Self {
        Self {
            pos: 0,
            state: Token::LineStart,
        }
    }

    pub fn parse(&mut self, line: &String) {
        let inst: Vec<&str> = line.split(' ').collect();
        if inst.is_empty() {
            return;
        }
        match inst[0] {
            "info" => {}
            _ => {}
        }
        unimplemented!()
    }

    fn consume_until_next(&mut self) {

    }
}

#[derive(Debug, Clone, Copy)]
enum Token {
    LineStart,
    Opcode,
    Pointer,
    Literal,
}

#[derive(Debug, Clone, Copy)]
pub enum AsmLexErr {
    ReadError,
    SyntaxError,
}

impl std::error::Error for AsmLexErr {}

impl fmt::Display for AsmLexErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ReadError => {
                write!(f, "Error: unable to read line")
            }
            Self::SyntaxError => {
                write!(f, "Syntax error")
            }
        }
    }
}

impl From<io::Error> for AsmLexErr {
    fn from(_from: io::Error) -> Self {
        Self::ReadError
    }
}