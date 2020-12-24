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
    pub fn new() -> Self {
        Lexer {
            code: "".chars().peekable(),
            state: None,
        }
    }

    //todo: add support for individual chars with single quotes
    pub fn tokenize(&mut self, code: &'a str) -> Result<Vec<Token>, AsmLexErr> {
        self.code = code.chars().peekable();
        let mut tokens = Vec::new();
        let mut buffer = String::new();
        while let Some(c) = self.code.next() {
            if c.is_whitespace() && c != ' ' {
                if !buffer.is_empty() {
                    if let Ok(num) = parse_as_number(&buffer) {
                        tokens.push(Token::NumLiteral(num))
                    } else {
                        tokens.push(
                            Token::Opcode(self.consume_opcode(&buffer).ok_or(
                                AsmLexErr::UnexpectedToken(buffer.clone())
                            )?)
                        );
                    }
                    buffer.clear();
                }
                continue
            }
            match c {
                '$' => {
                    let token = self.consume_register()?;
                    self.state = Some(token.clone());
                    tokens.push(token);
                    continue
                }
                '[' => {
                    let token = self.consume_pointer()?;
                    self.state = Some(token.clone());
                    tokens.push(token);
                    continue
                }
                ':' => {
                    let token = Token::LabelDeclStart(buffer.clone());
                    self.state = Some(token.clone());
                    tokens.push(token);
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
                    if !buffer.is_empty() {
                        if let Ok(num) = parse_as_number(&buffer) {
                            tokens.push(Token::NumLiteral(num))
                        } else {
                            tokens.push(
                                Token::Opcode(self.consume_opcode(&buffer).ok_or(
                                    AsmLexErr::UnexpectedToken(buffer.clone())
                                )?)
                            );
                        }
                        buffer.clear();
                    }
                }
                _ => {
                    buffer.push(c);
                }
            }
        }
        if !buffer.is_empty() {
            tokens.push(self.consume_last_token(buffer)?);
        }
        Ok(tokens)
    }

    pub fn parse() {

    }

    pub fn consume_opcode(&mut self, code: &str) -> Option<Opcode> {
        let token = match code {
            "hlt"  => Some(Opcode::Hlt),
            "mov"  => Some(Opcode::Mov),
            "jmp"  => Some(Opcode::Jmp),
            "jmpf" => Some(Opcode::Jmpf),
            "jmpb" => Some(Opcode::Jmpb),
            "cmp"  => Some(Opcode::Cmp),
            "lt"   => Some(Opcode::Lt),
            "gt"   => Some(Opcode::Gt),
            "le"   => Some(Opcode::Le),
            "ge"   => Some(Opcode::Ge),
            "jeq"  => Some(Opcode::Jeq),
            "jne"  => Some(Opcode::Jne),
            "aloc" => Some(Opcode::Aloc),
            "dalc" => Some(Opcode::Dalc),
            "add"  => Some(Opcode::Add),
            "sub"  => Some(Opcode::Sub),
            "mul"  => Some(Opcode::Mul),
            "div"  => Some(Opcode::Div),
            _      => None,
        };
        token
    }

    fn consume_register(&mut self) -> Result<Token, AsmLexErr> {
        let mut buf = String::new();
        loop {
            let c = self.code.next();
            if c == None || c.unwrap().is_whitespace() {
                return Ok(Token::Register(parse_as_register(&buf)?))
            } else if let Some(c) = c {
                buf.push(c);
            }
        }
    }

    fn consume_pointer(&mut self) -> Result<Token, AsmLexErr> {
        let mut buf = String::new();
        loop {
            let c = self.code.next();
            if c == Some(']') || c == None {
                return Ok(Token::Pointer(buf))
            } else if let Some(c) = c {
                buf.push(c);
            }
        }
    }

    fn consume_label_use(&mut self) -> Result<Token, AsmLexErr> {
        let mut buf = String::new();
        loop {
            let c = self.code.next();
            if c == None || c.unwrap().is_whitespace() {
                return Ok(Token::LabelUse(buf))
            } else if let Some(c) = c {
                buf.push(c);
            }
        }
    }

    fn consume_str_lit(&mut self) -> Result<Token, AsmLexErr> {
        let mut buf = String::new();
        loop {
            let c = self.code.next();
            if c == Some('"') {
                return Ok(Token::StrLiteral(buf))
            } else if c == Some('\\') {
                if let Some(c) = self.code.next() {
                    buf.push(c);
                } else {
                    return Err(AsmLexErr::UnexpectedEOF)
                }
                continue
            } else if let Some(c) = c {
                buf.push(c);
            } else if c == None {
                return Err(AsmLexErr::UnexpectedEOF)
            }
        }
    }

    fn consume_last_token(&mut self, mut last: String) -> Result<Token, AsmLexErr> {
        if last.starts_with("$") {
            return Ok(Token::Register(parse_as_register(&last[1..])?))
        } else if last.starts_with("[") {
            if last.ends_with("]") {
                last.pop();
                return Ok(Token::Pointer(last[1..].to_string()))
            } else {
                return Err(AsmLexErr::UnexpectedEOF)
            }
        } else if last.ends_with(":") { // is label: if last, something is wrong
            return Err(AsmLexErr::UnexpectedToken(last))
        } else if last.starts_with("@") {
            return Ok(Token::LabelUse(last[1..].to_string()))
        } else {
            if let Ok(num) = parse_as_number(&last) {
                return Ok(Token::NumLiteral(num))
            } else {
                return Ok(
                    Token::Opcode(self.consume_opcode(&last).ok_or(
                        AsmLexErr::UnexpectedToken(last)
                    )?)
                );
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

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Opcode(Opcode),
    Pointer(String),
    Register(u8),
    NumLiteral(i32),
    StrLiteral(String),
    LabelDeclStart(String),
    LabelDeclEnd,
    LabelUse(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    name: String,
    code: Vec<Token>,
}

#[derive(Debug, Clone, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_instruction() {
        let test_str = "\tmov $2 500\n\tadd $5 [main + 24] $10\n\tsub [50] 40 $2";
        let mut tokenizer = Lexer::new();
        let tokens = tokenizer.tokenize(test_str).unwrap();
        assert_eq!(tokens, vec![
            Token::Opcode(Opcode::Mov),
            Token::Register(2),
            Token::NumLiteral(500),
            Token::Opcode(Opcode::Add),
            Token::Register(5),
            Token::Pointer(String::from("main + 24")),
            Token::Register(10),
            Token::Opcode(Opcode::Sub),
            Token::Pointer(String::from("50")),
            Token::NumLiteral(40),
            Token::Register(2),
        ])
    }

    #[test]
    fn test_label_and_strlit() {
        let test_str = "string: {\"hello\"}";
        let mut tokenizer = Lexer::new();
        let tokens = tokenizer.tokenize(test_str).unwrap();
        assert_eq!(tokens, vec![
            Token::LabelDeclStart(String::from("string")),
            Token::StrLiteral(String::from("hello")),
            Token::LabelDeclEnd
        ])
    }

    #[test]
    fn test_label_usage() {
        let test_str = "label: { mov $3 500 }\nmov $5 @label";
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize(test_str).unwrap();
        assert_eq!(tokens, vec![
            Token::LabelDeclStart(String::from("label")),
            Token::Opcode(Opcode::Mov),
            Token::Register(3),
            Token::NumLiteral(500),
            Token::LabelDeclEnd,
            Token::Opcode(Opcode::Mov),
            Token::Register(5),
            Token::LabelUse(String::from("label"))
        ])
    }

    #[test]
    fn test_unexpected() {
        let test_str = "{";
        let mut lexer = Lexer::new();
        let res = lexer.tokenize(test_str);
        assert_eq!(Err(AsmLexErr::UnexpectedToken(String::from("{"))), res)
    }

    #[test]
    fn test_backtick() {
        let test_str = "label: { \"say \\\"hello world!\\\"\"}";
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize(test_str).unwrap();
        assert_eq!(tokens, vec![
            Token::LabelDeclStart(String::from("label")),
            Token::StrLiteral(String::from("say \"hello world!\"")),
            Token::LabelDeclEnd,
        ])
    }
}