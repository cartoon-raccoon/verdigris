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
    LabelStart(String, Context),
    LabelEnd(Context),
    Directive(AsmDir, Context),
    StrLiteral(String, Context),
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
                    if let Some(&Parsed::Directive(AsmDir::String, _)) = parsed.last() {
                        return Err(AsmParseErr::UnexpectedToken(
                            format!("str: {}\nString literals should always follow a .string directive.", 
                                text
                                ), con
                            )
                        )
                    }
                    parsed.push(Parsed::StrLiteral(text, con));
                }
                LabelDeclStart(name, con) => {
                    parsed.push(Parsed::LabelStart(name, con));
                }
                LabelDeclEnd(con) => {
                    parsed.push(Parsed::LabelEnd(con));
                }
                Directive(dir, con) => {
                    parsed.push(Parsed::Directive(dir, con));
                }
                _ => {
                    panic!("Operand tokens should not appear in the main parse fn")
                }
            }
        }
        
        Ok(parsed)
    }

    fn create_instruction(&mut self, op: Opcode, con: Context) -> Result<Parsed, AsmParseErr> {
        use AsmParseErr::*;
        use Opcode::*;

        let inst: Instruction;
        let operands = self.get_operands()?;
        let len = if operands.len() > 3 {
            return Err(TooManyOperands(con))
        } else {
            operands.len() as u8
        };
        let mut final_ops: (Option<Operand>, Option<Operand>, Option<Operand>) 
                    = (None, None, None);

        match op {
            Hlt => {
                inst = Instruction::from_parsed(Hlt, (None, None, None));
            }
            Mov => {
                if len != 2 {
                    return Err(IncorrectOperandNo(2, len, con))
                }
                
                // checking operand 1
                if let Operand::Register(reg) = &operands[0] {
                    final_ops.0 = Some(Operand::Register(*reg));
                } else if let Operand::Pointer(ptr) = &operands[0] {
                    final_ops.0 = Some(Operand::Pointer(ptr.clone()))
                } else {
                    return Err(InvalidOperand(operands[0].clone(), con))
                }

                // checking operand 2
                if let Operand::NumLiteral(num) = &operands[1] {
                    final_ops.1 = Some(Operand::NumLiteral(*num));
                } else if let Operand::Pointer(ptr) = &operands[1] {
                    final_ops.1 = Some(Operand::Pointer(ptr.clone()));
                } else if let Operand::Register(reg) = &operands[1] {
                    final_ops.1 = Some(Operand::Register(*reg))
                } else {
                    return Err(InvalidOperand(operands[1].clone(), con))
                }
                inst = Instruction::from_parsed(Mov, final_ops);
            }
            Jmp => {
                if len != 1 {
                    return Err(IncorrectOperandNo(1, len, con))
                }

                // checking operand 1
                if let Operand::NumLiteral(num) = &operands[0] {
                    final_ops.0 = Some(Operand::NumLiteral(*num));
                } else if let Operand::Pointer(ptr) = &operands[0] {
                    final_ops.0 = Some(Operand::Pointer(ptr.clone()));
                } else if let Operand::LabelUse(lab) = &operands[0] {
                    final_ops.0 = Some(Operand::LabelUse(lab.clone()));
                } else if let Operand::Register(reg) = &operands[0] {
                    final_ops.0 = Some(Operand::Register(*reg));
                } else {
                    return Err(InvalidOperand(operands[0].clone(), con))
                }
                inst = Instruction::from_parsed(Jmp, final_ops);
            }
            op @ Jmpf | op @ Jmpb => {
                if len != 1 {
                    return Err(IncorrectOperandNo(1, len, con))
                }

                if let Operand::NumLiteral(num) = &operands[0] {
                    final_ops.0 = Some(Operand::NumLiteral(*num));
                } else if let Operand::Register(reg) = &operands[0] {
                    final_ops.0 = Some(Operand::Register(*reg));
                } else {
                    return Err(InvalidOperand(operands[0].clone(), con))
                }

                inst = Instruction::from_parsed(op, final_ops);
            }
            op @ Cmp | op @ Lt | op @ Gt | op @ Le | op @ Ge => {
                if len != 2 {
                    return Err(IncorrectOperandNo(2, len, con))
                }
                
                if let Operand::NumLiteral(num) = &operands[0] {
                    final_ops.0 = Some(Operand::NumLiteral(*num));
                } else if let Operand::Register(reg) = &operands[0] {
                    final_ops.0 = Some(Operand::Register(*reg));
                } else {
                    return Err(InvalidOperand(operands[0].clone(), con))
                }

                if let Operand::NumLiteral(num) = &operands[1] {
                    final_ops.1 = Some(Operand::NumLiteral(*num));
                } else if let Operand::Register(reg) = &operands[1] {
                    final_ops.1 = Some(Operand::Register(*reg));
                } else {
                    return Err(InvalidOperand(operands[1].clone(), con))
                }

                inst = Instruction::from_parsed(op, final_ops);
            } 
            Push | Pop | Call | Ret => {
                unimplemented!("Calling conventions not yet specified")
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
        let test_code2 = "mov $3 $23 mov $5 69";
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

        let cmp_inst2_1 = Instruction::from_parsed(
            Opcode::Mov,
            (
                Some(Operand::Register(3)),
                Some(Operand::Register(23)),
                None
            )
        );

        let cmp_inst2_2 = Instruction::from_parsed(
            Opcode::Mov,
            (
                Some(Operand::Register(5)),
                Some(Operand::NumLiteral(69)),
                None
            )
        );

        assert_eq!(parsed1, vec![Parsed::Instruction(cmp_inst1)]);
        assert_eq!(parsed2, vec![
            Parsed::Instruction(cmp_inst2_1),
            Parsed::Instruction(cmp_inst2_2),
        ]);
        assert!(parsed3.is_err())
    }

    #[test]
    fn test_jmp_opcode_passing() {
        let test_lit = "jmp 600";
        let test_ptr = "jmp [main + 60]";
        let test_lab = "jmp @main";
        let test_reg = "jmp $20";

        let mut lexer = Lexer::new();
        let tokens_lit = lexer.tokenize(test_lit).unwrap();
        let tokens_ptr = lexer.tokenize(test_ptr).unwrap();
        let tokens_lab = lexer.tokenize(test_lab).unwrap();
        let tokens_reg = lexer.tokenize(test_reg).unwrap();

        let mut parser = Parser::new();
        let parsed_lit = parser.parse(tokens_lit).unwrap();
        let parsed_ptr = parser.parse(tokens_ptr).unwrap();
        let parsed_lab = parser.parse(tokens_lab).unwrap();
        let parsed_reg = parser.parse(tokens_reg).unwrap();

        let inst_lit = Instruction::from_parsed(
            Opcode::Jmp,
            (
                Some(Operand::NumLiteral(600)),
                None,
                None,
            )
        );

        let inst_ptr = Instruction::from_parsed(
            Opcode::Jmp,
            (
                Some(Operand::Pointer(String::from("main + 60"))),
                None,
                None,
            )
        );

        let inst_lab = Instruction::from_parsed(
            Opcode::Jmp,
            (
                Some(Operand::LabelUse(String::from("main"))),
                None,
                None,
            )
        );

        let inst_reg = Instruction::from_parsed(
            Opcode::Jmp,
            (
                Some(Operand::Register(20)),
                None,
                None,
            )
        );

        assert_eq!(parsed_lit, vec![Parsed::Instruction(inst_lit)]);
        assert_eq!(parsed_ptr, vec![Parsed::Instruction(inst_ptr)]);
        assert_eq!(parsed_lab, vec![Parsed::Instruction(inst_lab)]);
        assert_eq!(parsed_reg, vec![Parsed::Instruction(inst_reg)]);
    }

    #[test]
    fn test_jmpf_and_jmpb_parsing() {
        let test_jmpf_lit = "jmpf 10";
        let test_jmpf_reg = "jmpf $9";
        let test_jmpb_lit = "jmpb 15";
        let test_jmpb_reg = "jmpb $5";
        let test_jmpf_err = "jmpf [main + 2]";
        let test_jmpb_err = "jmpb [main + 2]";


        let mut lexer = Lexer::new();
        let tok_jmpf_lit = lexer.tokenize(test_jmpf_lit).unwrap();
        let tok_jmpf_reg = lexer.tokenize(test_jmpf_reg).unwrap();
        let tok_jmpb_lit = lexer.tokenize(test_jmpb_lit).unwrap();
        let tok_jmpb_reg = lexer.tokenize(test_jmpb_reg).unwrap();
        let tok_jmpf_err = lexer.tokenize(test_jmpf_err).unwrap();
        let tok_jmpb_err = lexer.tokenize(test_jmpb_err).unwrap();

        let mut parser = Parser::new();
        let p_jmpf_lit = parser.parse(tok_jmpf_lit).unwrap();
        let p_jmpf_reg = parser.parse(tok_jmpf_reg).unwrap();
        let p_jmpb_lit = parser.parse(tok_jmpb_lit).unwrap();
        let p_jmpb_reg = parser.parse(tok_jmpb_reg).unwrap();
        let p_jmpf_err = parser.parse(tok_jmpf_err);
        let p_jmpb_err = parser.parse(tok_jmpb_err);

        let inst_jmpf_lit = Instruction::from_parsed(
            Opcode::Jmpf,
            (
                Some(Operand::NumLiteral(10)),
                None,
                None,
            )
        );

        let inst_jmpf_reg = Instruction::from_parsed(
            Opcode::Jmpf,
            (
                Some(Operand::Register(9)),
                None,
                None,
            )
        );

        let inst_jmpb_lit = Instruction::from_parsed(
            Opcode::Jmpb,
            (
                Some(Operand::NumLiteral(15)),
                None,
                None,
            )
        );

        let inst_jmpb_reg = Instruction::from_parsed(
            Opcode::Jmpb,
            (
                Some(Operand::Register(5)),
                None,
                None,
            )
        );

        assert_eq!(p_jmpf_lit, vec![Parsed::Instruction(inst_jmpf_lit)]);
        assert_eq!(p_jmpf_reg, vec![Parsed::Instruction(inst_jmpf_reg)]);
        assert_eq!(p_jmpb_lit, vec![Parsed::Instruction(inst_jmpb_lit)]);
        assert_eq!(p_jmpb_reg, vec![Parsed::Instruction(inst_jmpb_reg)]);
        assert!(p_jmpf_err.is_err());
        assert!(p_jmpb_err.is_err());
    }

    #[test]
    fn test_comparison_parsing() {
        let test_code = "cmp $3 50";
        let test_err  = "cmp [420] @hello";

        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize(test_code).unwrap();
        let tokens_err = lexer.tokenize(test_err).unwrap();

        let mut parser = Parser::new();
        let parsed = parser.parse(tokens).unwrap();
        let parsed_err = parser.parse(tokens_err);

        let inst = Instruction::from_parsed(
            Opcode::Cmp,
            (
                Some(Operand::Register(3)),
                Some(Operand::NumLiteral(50)),
                None
            )
        );

        assert_eq!(parsed, vec![Parsed::Instruction(inst)]);
        assert!(parsed_err.is_err());
    }
}