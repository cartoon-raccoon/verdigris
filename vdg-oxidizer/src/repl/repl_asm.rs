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
        let lr = Interface::new("vdg-asm").unwrap();
        lr.set_prompt(">>> ").unwrap();
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
                Ok(exec) => {
                    if let Some(exec) = exec {
                        match exec {
                            Executable::Command(cmd) => {
                                self.exec_cmd(cmd);
                                continue
                            }
                            Executable::Instruction => {
                                unimplemented!()
                            }
                        }
                    } else {continue}
                }
                Err(e) => {
                    eprintln!("{}", e);
                    continue
                }
            }
        }
    }

    fn read_line(&mut self) -> Result<Option<Executable>, AsmLexErr> {
        if let ReadResult::Input(line) = self.interface.read_line()? {
            if let Some(exec) = self.lexer.parse(&line.to_lowercase())? {
                return Ok(Some(exec))
            }
        }
        return Ok(None)
    }

    fn exec_cmd(&self, cmd: ReplAsmCmd) {
        match cmd {
            ReplAsmCmd::Registers => {
                self.vm.dump_registers();
            }
            ReplAsmCmd::History => {
                unimplemented!()
            }
            ReplAsmCmd::Program => {
                self.vm.dump_program();
            }
            ReplAsmCmd::Help => {
                unimplemented!()
            }
            ReplAsmCmd::Info => {
                unimplemented!()
            }
            ReplAsmCmd::Quit => {
                std::process::exit(0);
            }
        }
    }
}

pub type ParseResult<T> = Result<T, AsmLexErr>;

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

    pub fn parse(&mut self, line: &String) -> ParseResult<Option<Executable>> {
        let inst: Vec<&str> = line.trim().split(' ').collect();
        if inst.is_empty() {
            return Ok(None)
        }
        if inst[0].starts_with('.') {
            if let Ok(cmd) = self.parse_command(inst) {
                return Ok(Some(Executable::Command(cmd)));
            } else {
                eprintln!("Invalid command, type .help for a list of commands");
                return Ok(None)
            }
        }
        match inst[0] {
            "hlt" => {}
            _ => {}
        }
        unimplemented!()
    }

    fn parse_command(&mut self, cmd: Vec<&str>) -> Result<ReplAsmCmd, ()> {
        match cmd[0] {
            ".info" => { return Ok(ReplAsmCmd::Info) }
            ".registers" => { return Ok(ReplAsmCmd::Registers) }
            ".history" => { return Ok(ReplAsmCmd::History) }
            ".program" => { return Ok(ReplAsmCmd::Program) }
            ".quit" => { return Ok(ReplAsmCmd::Quit) }
            ".help" => { return Ok(ReplAsmCmd::Help) }
            _ => {
                return Err(())
            }
        }
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
    Command(ReplAsmCmd),
}

#[derive(Debug, Clone, Copy)]
enum Executable {
    Instruction,
    Command(ReplAsmCmd),
}

#[derive(Debug, Clone, Copy)]
enum ReplAsmCmd {
    Info,
    Registers,
    History,
    Program,
    Quit,
    Help,
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