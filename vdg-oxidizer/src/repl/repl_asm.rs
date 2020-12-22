use linefeed::{
    Interface, 
    terminal::DefaultTerminal,
    reader::ReadResult,
};
use byteorder::*;
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
                            Executable::Instruction(bytes) => {
                                self.vm.add_bytes(bytes);
                                if let Ok(done) = self.vm.run_once() {
                                    if done {
                                        std::process::exit(0)
                                    } 
                                }
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
            self.interface.add_history(line.clone());
            if let Some(exec) = self.lexer.parse(line.to_lowercase())? {
                return Ok(Some(exec))
            }
        }
        return Ok(None)
    }

    fn exec_cmd(&self, cmd: ReplCmd) {
        match cmd {
            ReplCmd::Registers => {
                self.vm.dump_registers();
            }
            ReplCmd::Program => {
                self.vm.dump_program();
            }
            ReplCmd::Help => {
                unimplemented!()
            }
            ReplCmd::Info => {
                unimplemented!()
            }
            ReplCmd::Quit => {
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

    pub fn parse(&mut self, line: String) -> ParseResult<Option<Executable>> {
        let inst: Vec<&str> = line.trim().split(' ').collect();
        let len = inst.len() as u8;
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
        let mut code: Vec<u8> = Vec::new();
        match inst[0] {
            "hlt"  => {
                if len > 1 {
                    return Err(AsmLexErr::IncorrectOperandNo(0, len))
                }
                code.push(0x00);
            }
            "mov"  => {
                if len != 3 {
                    return Err(AsmLexErr::IncorrectOperandNo(2, len))
                }
                code.push(0x01);
                code.push(parse_as_number(&inst[1][1..])? as u8);
                if inst[2].starts_with("&") { //if is pointer
                    code.push(1);
                    code.extend_from_slice(&u32_to_bytes(
                        parse_as_number(&inst[2][1..])? as u32
                    ));
                } else {
                    code.push(0);
                    code.extend_from_slice(&i32_to_bytes(
                        parse_as_number(&inst[2])?
                    ));
                }
            }
            "jmp"  => {}
            "jmpf" => {}
            "jmpb" => {}
            "cmp"  => {}
            "lt"   => {}
            "gt"   => {}
            "le"   => {}
            "ge"   => {}
            "jeq"  => {}
            "jne"  => {}
            "aloc" => {}
            "dalc" => {}
            "add"  => {}
            "sub"  => {}
            "mul"  => {}
            "div"  => {}
            "igl"  => {}
            _ => {}
        }
        Ok(Some(Executable::Instruction(code)))
    }

    fn parse_command(&mut self, cmd: Vec<&str>) -> Result<ReplCmd, ()> {
        match cmd[0] {
            ".info" => { return Ok(ReplCmd::Info) }
            ".registers" => { return Ok(ReplCmd::Registers) }
            ".program" => { return Ok(ReplCmd::Program) }
            ".quit" => { return Ok(ReplCmd::Quit) }
            ".help" => { return Ok(ReplCmd::Help) }
            _ => {
                return Err(())
            }
        }
    }
}

fn parse_as_number(text: &str) -> Result<i32, AsmLexErr> {
    if let Ok(num) = text.parse::<i32>() {
        return Ok(num)
    } else {
        return Err(AsmLexErr::CouldNotParse(text.to_string()))
    }
}

fn i32_to_bytes(num: i32) -> [u8; 4] {
    let mut buf = [0, 0, 0, 0];
    buf.as_mut().write_i32::<LittleEndian>(num).unwrap();
    buf
}

fn u32_to_bytes(num: u32) -> [u8; 4] {
    let mut buf = [0, 0, 0, 0];
    buf.as_mut().write_u32::<LittleEndian>(num).unwrap();
    buf
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum Token {
    LineStart,
    Opcode(u8),
    Pointer(usize),
    Register(usize),
    Literal(i32),
}

#[derive(Debug, Clone)]
enum Executable {
    Instruction(Vec<u8>),
    Command(ReplCmd),
}

#[derive(Debug, Clone, Copy)]
enum ReplCmd {
    Info,
    Registers,
    Program,
    Quit,
    Help,
}

#[derive(Debug, Clone)]
pub enum AsmLexErr {
    ReadError,
    UnexpectedOperand(String),
    IncorrectOperandNo(u8, u8),
    InvalidRegister(u8),
    CouldNotParse(String),
}

impl std::error::Error for AsmLexErr {}

impl fmt::Display for AsmLexErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ReadError => {
                write!(f, "Error: unable to read line")
            }
            Self::UnexpectedOperand(op) => {
                write!(f, "Error: Unexpected operand {}", op)
            }
            Self::IncorrectOperandNo(expected, given) => {
                write!(f, "Error: {} operands expected, found {}",
                    expected, given
                )
            }
            Self::InvalidRegister(num) => {
                write!(f, "Error: invalid register {}", num)
            }
            Self::CouldNotParse(text) => {
                write!(f, "Error: could not parse `{}` as a number", text)
            }
        }
    }
}

impl From<io::Error> for AsmLexErr {
    fn from(_from: io::Error) -> Self {
        Self::ReadError
    }
}