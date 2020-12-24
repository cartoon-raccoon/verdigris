use byteorder::*;
use std::io;
use std::fmt;

use crate::vm::instruction::Opcode;

pub type ParseResult<T> = Result<T, AsmLexErr>;

pub struct AsmLexer {
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
                    return Err(AsmLexErr::IncorrectOperandNo(0, len - 1))
                }
                code.push(Opcode::Hlt as u8);
            }
            "mov"  => {
                if len != 3 {
                    return Err(AsmLexErr::IncorrectOperandNo(2, len - 1))
                }
                code.push(Opcode::Mov as u8);
                code.push(parse_as_register(&inst[1][1..])? as u8);
                if inst[2].starts_with("&") { //if is pointer
                    code.push(1);
                    code.extend_from_slice(&u32_to_bytes(
                        parse_as_number(&inst[2][1..])? as u32
                    ));
                } else if inst[2].starts_with("$") { //is register
                    code.push(2);
                    code.push(parse_as_register(&inst[2][1..])? as u8);
                } else { //is literal
                    code.push(0);
                    code.extend_from_slice(&i32_to_bytes(
                        parse_as_number(&inst[2])?
                    ));
                }
            }
            "jmp"  => {
                if len != 2 {
                    return Err(AsmLexErr::IncorrectOperandNo(1, len - 1))
                }
                code.push(Opcode::Jmp as u8);
                if inst[1].starts_with("&") { //is pointer
                    code.push(1);
                    code.extend_from_slice(&u32_to_bytes(
                        parse_as_number(&inst[1][1..])? as u32
                    ));
                } else if let Ok(num) = parse_as_number(inst[1]) {
                    //is literal
                    code.push(0);
                    code.extend_from_slice(&i32_to_bytes(num));
                } else { //is label
                    unimplemented!("jumping to labels not yet implemented")
                }
            }
            op @ "jmpf" | op @ "jmpb" => {
                if len != 2 {
                    return Err(AsmLexErr::IncorrectOperandNo(1, len - 1))
                }
                code.push(if op == "jmpf" {
                        Opcode::Jmpf as u8
                    } else {
                        Opcode::Jmpb as u8
                    }
                );
                if let Ok(num) = parse_as_number(inst[1]) {
                    code.extend_from_slice(&i32_to_bytes(num));
                } else {
                    return Err(AsmLexErr::UnexpectedOperand(inst[1].to_string()))
                }
            }
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

fn parse_as_register(text: &str) -> Result<u8, AsmLexErr> {
    if let Ok(num) = text.parse::<u32>() {
        if num > 31 {
            return Err(AsmLexErr::InvalidRegister(num))
        }
        return Ok(num as u8)
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
pub enum Token {
    LineStart,
    Opcode(u8),
    Pointer(usize),
    Register(usize),
    Literal(i32),
}

#[derive(Debug, Clone)]
pub enum Executable {
    Instruction(Vec<u8>),
    Command(ReplCmd),
}

#[derive(Debug, Clone, Copy)]
pub enum ReplCmd {
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
    InvalidRegister(u32),
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