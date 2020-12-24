//! The Parser struct simply takes a token stream 
//! and outputs a stream of parsed instructions.
//! 
//! It does not process labels, strings or directives;
//! that role falls to the Assembler to put everything together.

use std::iter::Peekable;
use std::vec::IntoIter;
use std::convert::TryFrom;
use std::fmt;

use crate::assembler::{
    Token, 
    AsmParseErr, 
    Context, 
    Directive as AsmDir,
};
use crate::vm::{Instruction, Opcode};

#[derive(Debug, Clone, PartialEq)]
pub enum Parsed {
    Instruction(Instruction),
    LabelStart(String),
    LabelEnd,
    Directive(AsmDir),
    StrLiteral(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Pointer(String),
    Register(u8),
    LabelUse(String),
    NumLiteral(i32),
}

impl TryFrom<Token> for Operand {
    type Error = AsmParseErr;

    fn try_from(from: Token) -> Result<Self, Self::Error> {
        match from {
            Token::Pointer(ptr, _) => {
                return Ok(Self::Pointer(ptr))
            }
            Token::Register(reg, _) => {
                return Ok(Self::Register(reg))
            }
            Token::NumLiteral(num, _) => {
                return Ok(Self::NumLiteral(num))
            }
            Token::LabelUse(name, _) => {
                return Ok(Self::LabelUse(name))
            }
            Token::Opcode(_, _) |
            Token::StrLiteral(_,_) |
            Token::LabelDeclStart(_, _) |
            Token::LabelDeclEnd(_) |
            Token::Directive(_,_) => {
                return Err(AsmParseErr::InvalidOperandConversion(from.clone()))
            }
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Operand::*;
        match self {
            Pointer(ptr) => {
                write!(f, "[{}]", ptr)
            }
            Register(reg) => {
                write!(f, "${}", reg)
            }
            LabelUse(name) => {
                write!(f, "@{}", name)
            }
            NumLiteral(num) => {
                write!(f, "{}", num)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new().into_iter().peekable(),
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<Vec<Parsed>, AsmParseErr> {
        use Token::*;

        let mut parsed: Vec<Parsed> = Vec::new();
        self.tokens = tokens.into_iter().peekable();

        while let Some(token) = self.tokens.next() {
            match token {
                Opcode(op, con) => {
                    parsed.push(self.create_instruction(op, con)?)
                }
                StrLiteral(text, con) => {
                    if parsed.last() != Some(&Parsed::Directive(AsmDir::String)) {
                        return Err(AsmParseErr::UnexpectedToken(
                            format!("str: {}\nString literals should always follow a .string directive.", 
                                text
                                ), con
                            )
                        )
                    }
                    parsed.push(Parsed::StrLiteral(text));
                }
                LabelDeclStart(name, _con) => {
                    parsed.push(Parsed::LabelStart(name));
                }
                LabelDeclEnd(_con) => {
                    parsed.push(Parsed::LabelEnd);
                }
                Directive(dir, _con) => {
                    parsed.push(Parsed::Directive(dir));
                }
                _ => {
                    panic!("Operand tokens should not appear in the main parse fn")
                }
            }
        }
        
        Ok(parsed)
    }

    fn create_instruction(&mut self, op: Opcode, con: Context) -> Result<Parsed, AsmParseErr> {
        let inst: Instruction;
        match op {
            Opcode::Hlt => {
                inst = Instruction::from_parsed(op, (None, None, None));
            }
            Opcode::Mov => {
                let operands = self.get_operands()?;
                if operands.len() != 2 {
                    return Err(AsmParseErr::IncorrectOperandNo(
                        2, 
                        operands.len() as u8, 
                        con
                    ))
                }
                if let Operand::Register(reg) = operands[0] {
                    if let Operand::NumLiteral(num) = &operands[1] {
                        inst = Instruction::from_parsed(
                            Opcode::Mov, 
                            (
                                Some(Operand::Register(reg)),
                                Some(Operand::NumLiteral(*num)),
                                None
                            )
                        )
                    } else if let Operand::Pointer(ptr) = &operands[1] {
                        inst = Instruction::from_parsed(
                            Opcode::Mov, 
                            (
                                Some(Operand::Register(reg)),
                                Some(Operand::Pointer(ptr.clone())),
                                None
                            )
                        )
                    } else if let Operand::Register(reg2) = &operands[1] {
                        inst = Instruction::from_parsed(
                            Opcode::Mov, 
                            (
                                Some(Operand::Register(reg)),
                                Some(Operand::Register(*reg2)),
                                None
                            )
                        )
                    } else {
                        return Err(AsmParseErr::InvalidOperand(operands[1].clone(), con))
                    }
                } else {
                    return Err(AsmParseErr::InvalidOperand(operands[0].clone(), con))
                }
            }
            _ => {
                unimplemented!("Other opcodes not yet implemented")
            }
        }
        Ok(Parsed::Instruction(inst))
    }

    fn get_operands(&mut self) -> Result<Vec<Operand>, AsmParseErr> {
        let mut operands: Vec<Operand> = Vec::new();
        loop {
            let peek = self.tokens.peek();
            if let Some(peek) = peek {
                if !peek.is_operand() {
                    return Ok(operands)
                }
            } else {
                return Ok(operands)
            }
            let token = self.tokens.next();

            //by this time, the token should definitely be an operand
            if let Some(token) = token {
                operands.push(Operand::try_from(token)?)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::Lexer;

    #[test]
    fn test_hlt_opcode_parsing() {
        let test_code = "   hlt  ";
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize(test_code).unwrap();

        let mut parser = Parser::new();
        let parsed = parser.parse(tokens).unwrap();

        let inst = Instruction::from_parsed(Opcode::Hlt, (None, None, None));

        assert_eq!(parsed, vec![Parsed::Instruction(inst)])
    }

    #[test]
    fn test_mov_opcode_parsing() {
        let test_code1 = "mov $2 [500]";
        let test_code2 = "mov $3 $23";
        let fail_code1 = "mov [4] @label";

        let mut lexer = Lexer::new();
        let tokens1 = lexer.tokenize(test_code1).unwrap();
        let tokens2 = lexer.tokenize(test_code2).unwrap();
        let tokens3 = lexer.tokenize(fail_code1).unwrap();

        let mut parser = Parser::new();
        let parsed1 = parser.parse(tokens1).unwrap();
        let parsed2 = parser.parse(tokens2).unwrap();
        let parsed3 = parser.parse(tokens3);

        let cmp_inst1 = Instruction::from_parsed(
            Opcode::Mov,
            (
                Some(Operand::Register(2)),
                Some(Operand::Pointer(String::from("500"))),
                None
            )
        );

        let cmp_inst2 = Instruction::from_parsed(
            Opcode::Mov,
            (
                Some(Operand::Register(3)),
                Some(Operand::Register(23)),
                None
            )
        );

        assert_eq!(parsed1, vec![Parsed::Instruction(cmp_inst1)]);
        assert_eq!(parsed2, vec![Parsed::Instruction(cmp_inst2)]);
        assert!(parsed3.is_err())
    }
}