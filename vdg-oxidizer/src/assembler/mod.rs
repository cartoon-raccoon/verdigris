pub mod assembler;
pub mod lexer;
pub mod parser;

pub use assembler::Assembler;
pub use lexer::{Lexer, Token, Context};
pub use parser::{Parser, Operand};

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum AsmParseErr {
    UnexpectedOperand(String, Context),
    UnexpectedToken(String, Context),
    UnexpectedEOF(Context),
    IncorrectOperandNo(u8, u8, Context),
    TooManyOperands(Context),
    InvalidRegister(u32, Context),
    CouldNotParse(String, Context),
    InvalidDirective(String, Context),
    InvalidOperand(Operand, Context),
    InvalidOperandConversion(Token),
}

impl std::error::Error for AsmParseErr {}

impl fmt::Display for AsmParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnexpectedOperand(op, con) => {
                write!(f, 
                    "Error: Unexpected operand {}\nLine {}, Column {}", 
                    op, con.line, con.column
                )
            }
            Self::UnexpectedToken(token, con) => {
                write!(f, 
                    "Error: Unexpected token {}\n Line {}, Column {}", 
                    token, con.line, con.column
                )
            }
            Self::UnexpectedEOF(con) => {
                write!(f, 
                    "Error: Reached end of file while parsing\nLine {}, Column {}",
                    con.line, con.column
                )
            }
            Self::IncorrectOperandNo(expected, given, con) => {
                write!(f, "Error: {} operands expected, found {}\nLine {}, Column {}",
                    expected, given, con.line, con.column
                )
            }
            Self::InvalidRegister(num, con) => {
                write!(f, 
                    "Error: invalid register {}\nLine {} Column {}", 
                    num, con.line, con.column
                )
            }
            Self::TooManyOperands(con) => {
                write!(f, 
                    "Error: too many operands; expected at most 3\nLine {} Column {}",
                    con.line, con.column
                )
            }
            Self::CouldNotParse(text, con) => {
                write!(f, 
                    "Error: could not parse `{}` as a number\nLine {} Column {}", 
                    text, con.line, con.column
                )
            }
            Self::InvalidDirective(dir, con) => {
                write!(f, 
                    "Error: invalid directive {:?}\nLine {} Column {}",
                    dir, con.line, con.column
                )
            }
            Self::InvalidOperand(op, con) => {
                write!(f,
                    "Error: invalid operand {:?}\nLine {} Column {}",
                    op, con.line, con.column
                )
            }
            Self::InvalidOperandConversion(token) => {
                write!(f,
                    "Error: invalid operand conversion {}\nLine {} Column {}",
                    token, token.context().line, token.context().column
                )
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Directive {
    Code,
    Data,
    String,
    Global,
}

impl Directive {
    pub fn try_from(from: String, con: Context) -> Result<Self, AsmParseErr> {
        match from.as_str() {
            "code" => {
                Ok(Self::Code)
            }
            "data" => {
                Ok(Self::Data)
            }
            "string" => {
                Ok(Self::String)
            }
            "global" => {
                Ok(Self::Global)
            }
            inval @ _ => {
                Err(AsmParseErr::InvalidDirective(inval.to_string(), con))
            }
        }
    }
}