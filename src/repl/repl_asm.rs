use crate::vm::VM;

/// The repl for the low-level IR (assembly) of Verdigris.
pub struct ReplAsm {
    vm: VM,
    prompt: &'static str,
    lexer: AsmLexer,
}

impl ReplAsm {
    pub fn new() -> Self {
        Self {
            vm: VM::new(vec![]),
            prompt: "vdg-asm >>>",
            lexer: AsmLexer::new(),
        }
    }
}

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

    pub fn parse(&mut self, line: String) {

    }

    fn consume_until_next(&mut self) {

    }
}

enum Token {
    LineStart,
    Opcode,
    Pointer,
    Literal,
}