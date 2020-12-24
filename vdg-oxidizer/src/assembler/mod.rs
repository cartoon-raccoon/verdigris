pub mod assembler;
pub mod lexer;

pub use assembler::Assembler;
pub use lexer::Lexer;

use lexer::Context;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum AsmParseErr {
    UnexpectedOperand(String, Context),
    UnexpectedToken(String, Context),
    UnexpectedEOF(Context),
    IncorrectOperandNo(u8, u8, Context),
    InvalidRegister(u32, Context),
    CouldNotParse(String, Context),
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
                    "Error: invalid register {}\nLine {}, Column {}", 
                    num, con.line, con.column
                )
            }
            Self::CouldNotParse(text, con) => {
                write!(f, 
                    "Error: could not parse `{}` as a number\nLine {} Column {}", 
                    text, con.line, con.column
                )
            }
        }
    }
}