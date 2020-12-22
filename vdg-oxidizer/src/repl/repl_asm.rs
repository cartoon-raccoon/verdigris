use linefeed::{
    Interface, 
    terminal::DefaultTerminal,
    reader::ReadResult,
};
use std::io;
use std::fmt;

use crate::vm::VM;
use crate::vm::instruction::Opcode;

/// The repl for the low-level IR (assembly) of Verdigris.
pub struct Repl {
    vm: VM,
    interface: Interface<DefaultTerminal>,
    lexer: AsmLexer,
}

impl Repl {
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
        println!("Oxidizer Shell v0.1.0");
        println!("Type .help for a list of commands.");
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
            self.lexer.parse(&line.to_lowercase());
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

    pub fn parse(&mut self, line: &String) -> Option<Opcode> {
        let inst: Vec<&str> = line.split(' ').collect();
        if inst.is_empty() {
            return None
        }
        if inst[0].starts_with('.') {
            if let Err(_) = self.command(inst) {
                eprintln!("Invalid command, issue .help for a list of commands")
            }
            return None
        }
        match inst[0] {
            "hlt" => {}
            _ => {}
        }
        unimplemented!()
    }

    fn command(&mut self, cmd: Vec<&str>) -> Result<ReplAsmCmd, ()> {
        match cmd[0] {
            ".info" => {}
            ".registers" => {}
            ".history" => {}
            ".program" => {}
            ".quit" => {}
            ".help" => {}
            _ => {
                return Err(())
            }
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

enum ReplAsmCmd {
    Info,
    Registers,
    History,
    Program,
    Quit,
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