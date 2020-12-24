//! The lexical analyser for Verdigris assembly code.
//! 
//! The Lexer struct only converts the raw assembly text into a token stream.
//! This token stream is then passed down to the Parser struct, which then
//! parses the tokens into discrete instructions.

use std::iter::Peekable;
use std::str::Chars;

use crate::vm::Opcode;
use crate::assembler::AsmParseErr;

/// The representation of a finite state machine for lexical analysis
/// of Verdigris bytecode asssembly.
#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    code: Peekable<Chars<'a>>,
    state: Option<Token>,
    context: Context,
}

impl<'a> Lexer<'a> {
    /// Returns a new instance of itself.
    pub fn new() -> Self {
        Lexer {
            code: "".chars().peekable(),
            state: None,
            context: Context::from(1, 1)
        }
    }

    //todo: add support for individual chars with single quotes
    //todo: add support for escaped characters (\n, \t, etc)
    /// Parses a single string into a token stream.
    pub fn tokenize(&mut self, code: &'a str) -> Result<Vec<Token>, AsmParseErr> {
        self.code = code.chars().peekable();
        let mut tokens = Vec::new();
        let mut buffer = String::new();
        while let Some(c) = self.code.next() {
            if c.is_whitespace() && c != ' ' {
                if !buffer.is_empty() {
                    if let Ok(num) = self.parse_as_number(&buffer) {
                        tokens.push(Token::NumLiteral(num, self.context))
                    } else {
                        tokens.push(
                            Token::Opcode(self.consume_opcode(&buffer).ok_or(
                                AsmParseErr::UnexpectedToken(buffer.clone(), self.context)
                            )?, self.context)
                        );
                    }
                    buffer.clear();
                }
                if c == '\n' {
                    self.context.line += 1;
                    self.context.column = 1;
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
                    let token = Token::LabelDeclStart(buffer.clone(), self.context);
                    self.state = Some(token.clone());
                    tokens.push(token);
                    buffer.clear();
                    continue
                }
                '{' => {
                    if let Some(&Token::LabelDeclStart(_, _)) = tokens.last()  {
                        continue
                    } else {
                        return Err(AsmParseErr::UnexpectedToken(c.to_string(), self.context))
                    }
                }
                '}' => {
                    tokens.push(Token::LabelDeclEnd(self.context));
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
                        if let Ok(num) = self.parse_as_number(&buffer) {
                            tokens.push(Token::NumLiteral(num, self.context))
                        } else {
                            tokens.push(
                                Token::Opcode(self.consume_opcode(&buffer).ok_or(
                                    AsmParseErr::UnexpectedToken(buffer.clone(), self.context)
                                )?, self.context)
                            );
                        }
                        buffer.clear();
                    }
                }
                _ => {
                    buffer.push(c);
                }
            }
            self.context.column += 1;
        }
        if !buffer.is_empty() {
            tokens.push(self.consume_last_token(buffer)?);
        }
        Ok(tokens)
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

    fn consume_register(&mut self) -> Result<Token, AsmParseErr> {
        let mut buf = String::new();
        loop {
            let c = self.code.next();
            self.context.column += 1;
            if c == None || c.unwrap().is_whitespace() {
                return Ok(Token::Register(self.parse_as_register(&buf)?, self.context))
            } else if let Some(c) = c {
                if c == '\n' {
                    self.context.line += 1;
                    self.context.column = 1;
                }
                buf.push(c);
            }
        }
    }

    fn consume_pointer(&mut self) -> Result<Token, AsmParseErr> {
        let mut buf = String::new();
        loop {
            let c = self.code.next();
            self.context.column += 1;
            if c == Some(']') || c == None {
                return Ok(Token::Pointer(buf, self.context))
            } else if let Some(c) = c {
                if c == '\n' {
                    self.context.line += 1;
                    self.context.column = 1;
                }
                buf.push(c);
            }
        }
    }

    fn consume_label_use(&mut self) -> Result<Token, AsmParseErr> {
        let mut buf = String::new();
        loop {
            let c = self.code.next();
            self.context.column += 1;
            if c == None || c.unwrap().is_whitespace() {
                return Ok(Token::LabelUse(buf, self.context))
            } else if let Some(c) = c {
                if c == '\n' {
                    self.context.line += 1;
                    self.context.column = 1;
                }
                buf.push(c);
            }
        }
    }

    fn consume_str_lit(&mut self) -> Result<Token, AsmParseErr> {
        let mut buf = String::new();
        loop {
            let c = self.code.next();
            self.context.column += 1;
            if c == Some('"') {
                return Ok(Token::StrLiteral(buf, self.context))
            } else if c == Some('\\') {
                if let Some(c) = self.code.next() {
                    buf.push(c);
                } else {
                    return Err(AsmParseErr::UnexpectedEOF(self.context))
                }
                continue
            } else if let Some(c) = c {
                if c == '\n' {
                    self.context.line += 1;
                    self.context.column = 1;
                }
                buf.push(c);
            } else if c == None {
                return Err(AsmParseErr::UnexpectedEOF(self.context))
            }
        }
    }

    fn consume_last_token(&mut self, mut last: String) -> Result<Token, AsmParseErr> {
        if last.starts_with("$") {
            return Ok(Token::Register(self.parse_as_register(&last[1..])?, self.context))
        } else if last.starts_with("[") {
            if last.ends_with("]") {
                last.pop();
                return Ok(Token::Pointer(last[1..].to_string(), self.context))
            } else {
                return Err(AsmParseErr::UnexpectedEOF(self.context))
            }
        } else if last.ends_with(":") { // is label: if last, something is wrong
            return Err(AsmParseErr::UnexpectedToken(last, self.context))
        } else if last.starts_with("@") {
            return Ok(Token::LabelUse(last[1..].to_string(), self.context))
        } else {
            if let Ok(num) = self.parse_as_number(&last) {
                return Ok(Token::NumLiteral(num, self.context))
            } else {
                return Ok(
                    Token::Opcode(self.consume_opcode(&last).ok_or(
                        AsmParseErr::UnexpectedToken(last, self.context)
                    )?, self.context)
                );
            }
        }
    }

    fn parse_as_number(&self, text: &str) -> Result<i32, AsmParseErr> {
        if let Ok(num) = text.parse::<i32>() {
            return Ok(num)
        } else {
            return Err(AsmParseErr::CouldNotParse(text.to_string(), self.context))
        }
    }
    
    fn parse_as_register(&self, text: &str) -> Result<u8, AsmParseErr> {
        if let Ok(num) = text.parse::<u32>() {
            if num > 31 {
                return Err(AsmParseErr::InvalidRegister(num, self.context))
            }
            return Ok(num as u8)
        } else {
            return Err(AsmParseErr::CouldNotParse(text.to_string(), self.context))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Opcode(Opcode, Context),
    Pointer(String, Context),
    Register(u8, Context),
    NumLiteral(i32, Context),
    StrLiteral(String, Context),
    LabelUse(String, Context),
    LabelDeclStart(String, Context),
    LabelDeclEnd(Context),
}

impl Token {
    pub fn context(&self) -> Context {
        use Token::*;
        match self {
            Opcode(_, con) => return *con,
            Pointer(_, con) => return *con,
            Register(_, con) => return *con,
            NumLiteral(_, con) => return *con,
            StrLiteral(_, con) => return *con,
            LabelUse(_, con) => return *con,
            LabelDeclStart(_, con) => return *con,
            LabelDeclEnd(con) => return *con,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context {
    pub line: u32,
    pub column: u32,
}

impl Context {
    pub fn from(line: u32, col: u32) -> Self {
        Self {
            line: line,
            column: col,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_instruction() {
        let test_str = "\tmov $2 500\n\tadd $5 [main + 24] $10   sub [50] 40 $2";
        let mut tokenizer = Lexer::new();
        let tokens = tokenizer.tokenize(test_str).unwrap();
        assert_eq!(tokens, vec![
            Token::Opcode(Opcode::Mov, tokens[0].context()),
            Token::Register(2, tokens[1].context()),
            Token::NumLiteral(500, tokens[2].context()),
            Token::Opcode(Opcode::Add, tokens[3].context()),
            Token::Register(5, tokens[4].context()),
            Token::Pointer(String::from("main + 24"), tokens[5].context()),
            Token::Register(10, tokens[6].context()),
            Token::Opcode(Opcode::Sub, tokens[7].context()),
            Token::Pointer(String::from("50"), tokens[8].context()),
            Token::NumLiteral(40, tokens[9].context()),
            Token::Register(2, tokens[10].context()),
        ])
    }

    #[test]
    fn test_label_and_strlit() {
        let test_str = "string: {\"hello\"}";
        let mut tokenizer = Lexer::new();
        let tokens = tokenizer.tokenize(test_str).unwrap();
        assert_eq!(tokens, vec![
            Token::LabelDeclStart(String::from("string"), tokens[0].context()),
            Token::StrLiteral(String::from("hello"), tokens[1].context()),
            Token::LabelDeclEnd(tokens[2].context())
        ])
    }

    #[test]
    fn test_label_usage() {
        let test_str = "label: { mov $3 500 }\nmov $5 @label";
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize(test_str).unwrap();
        assert_eq!(tokens, vec![
            Token::LabelDeclStart(String::from("label"), tokens[0].context()),
            Token::Opcode(Opcode::Mov, tokens[1].context()),
            Token::Register(3, tokens[2].context()),
            Token::NumLiteral(500, tokens[3].context()),
            Token::LabelDeclEnd(tokens[4].context()),
            Token::Opcode(Opcode::Mov, tokens[5].context()),
            Token::Register(5, tokens[6].context()),
            Token::LabelUse(String::from("label"), tokens[7].context())
        ])
    }

    #[test]
    fn test_unexpected() {
        let test_str = "{";
        let mut lexer = Lexer::new();
        let res = lexer.tokenize(test_str);
        assert_eq!(Err(AsmParseErr::UnexpectedToken(String::from("{"), Context::from(1,1))), res)
    }

    #[test]
    fn test_backtick() {
        let test_str = "label: { \"say \\\"hello world!\\\"\"}";
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize(test_str).unwrap();
        assert_eq!(tokens, vec![
            Token::LabelDeclStart(String::from("label"), tokens[0].context()),
            Token::StrLiteral(String::from("say \"hello world!\""), tokens[1].context()),
            Token::LabelDeclEnd(tokens[2].context()),
        ])
    }
}