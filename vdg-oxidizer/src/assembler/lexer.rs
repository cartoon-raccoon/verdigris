use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

use crate::instruction::Opcode;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    code: Peekable<Chars<'a>>,
    state: Option<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Lexer {
            code: code.chars().peekable(),
            state: None,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, AsmLexErr> {
        let mut tokens = Vec::new();
        let mut buffer = String::new();
        while let Some(c) = self.code.next() {
            match c {
                '$' => {
                    tokens.push(self.consume_register()?);
                    continue
                }
                '&' => {
                    tokens.push(self.consume_pointer()?);
                    continue
                }
                ':' => {
                    tokens.push(Token::LabelDeclStart(buffer.clone()));
                    buffer.clear();
                    continue
                }
                '{' => {
                    if let Some(&Token::LabelDeclStart(_)) = tokens.last()  {
                        continue
                    } else {
                        return Err(AsmLexErr::UnexpectedToken(c.to_string()))
                    }
                }
                '}' => {
                    tokens.push(Token::LabelDeclEnd);
                    continue
                }
                '@' => {
                    tokens.push(self.consume_label_use()?);
                    continue
                }
                '"' => {
                    tokens.push(self.consume_str_lit()?)
                }
                ' ' => {
                    // see what to do later?
                }
                _ => {
                    buffer.push(c);
                }
            }
        }
        Ok(tokens)
    }

    pub fn parse() {

    }

    fn consume_register(&mut self) -> Result<Token, AsmLexErr> {
        let mut buf = String::new();
        loop {
            let c = self.code.next();
            if c == Some(' ') {
                return Ok(Token::Register(parse_as_register(&buf)?))
            } else if let Some(c) = c {
                buf.push(c);
            } else if let None = c {
                return Err(AsmLexErr::UnexpectedEOF)
            }
        }
    }

    fn consume_pointer(&mut self) -> Result<Token, AsmLexErr> {
        let mut buf = String::new();
        loop {
            let c = self.code.next();
            if c == Some(' ') {
                return Ok(Token::Pointer(parse_as_number(&buf)? as usize))
            } else if let Some(c) = c {
                buf.push(c);
            } else if let None = c {
                return Err(AsmLexErr::UnexpectedEOF)
            }
        }
    }

    fn consume_label_use(&mut self) -> Result<Token, AsmLexErr> {
        let mut buf = String::new();
        loop {
            let c = self.code.next();
            if c == Some(' ') {
                return Ok(Token::LabelUse(buf))
            } else if let Some(c) = c {
                buf.push(c);
            } else if let None = c {
                return Err(AsmLexErr::UnexpectedEOF)
            }
        }
    }

    fn consume_str_lit(&mut self) -> Result<Token, AsmLexErr> {
        let mut buf = String::new();
        loop {
            let c = self.code.next();
            if c == Some('"') {
                return Ok(Token::StrLiteral(buf))
            } else if let Some(c) = c {
                buf.push(c);
            } else if let None = c {
                return Err(AsmLexErr::UnexpectedEOF)
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

#[derive(Debug, Clone)]
pub enum Token {
    Opcode(Opcode),
    Pointer(usize),
    Register(u8),
    NumLiteral(i32),
    StrLiteral(String),
    LabelDeclStart(String),
    LabelDeclEnd,
    LabelUse(String),
}

#[derive(Debug, Clone)]
pub struct Label {
    name: String,
    code: Vec<Token>,
}

#[derive(Debug, Clone)]
pub enum AsmLexErr {
    UnexpectedOperand(String),
    UnexpectedToken(String),
    UnexpectedEOF,
    IncorrectOperandNo(u8, u8),
    InvalidRegister(u32),
    CouldNotParse(String),
}

impl std::error::Error for AsmLexErr {}

impl fmt::Display for AsmLexErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnexpectedOperand(op) => {
                write!(f, "Error: Unexpected operand {}", op)
            }
            Self::UnexpectedToken(token) => {
                write!(f, "Error: Unexpected token {}", token)
            }
            Self::UnexpectedEOF => {
                write!(f, "Error: Reached end of file while parsing")
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