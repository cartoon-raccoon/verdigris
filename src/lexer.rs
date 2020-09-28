#[derive(Clone, Debug)]
pub enum TokenType {
    //core keywords
    Let, Struct, Function, Method, Trait, 
    Closure, Impl, Enum, Return, This,

    //control flow and logic
    If, Else, Match,
    And,        // &&
    Or,         // ||
    True, False, Not,
    While, For,

    //error handling
    Try, Catch, Finally,

    //types and identifiers
    Ident(String),
    Strng(String),      //? Bind to a type?
    Int(usize),        //implement like python
    Float(f64),
    Bool(bool),
    List,
    HashMap,
    Nil,

    //operators
    Assgn,      // =
    Plus,       // +
    Minus,      // -
    Times,      //\*
    Divide,     // /
    Eq,         // ==
    Neq,        //\!=
    GEq,        // >=
    LEq,        // <=

    //delimiters
    StmtEnd,    // ;
    LeftCBkt,   // (
    RightCBkt,  // )
    LeftSqBkt,  // [
    RightSqBkt, // ]
    OpenBlock,  // {
    CloseBlock, // }
    Comma,      // ,

    //End of File
    EOF,

    //tokens that throw an error
    Unknown,    // Exits with an error if encountered
}

impl From<TokenType> for String {
    fn from(t: TokenType) -> String {
        String::new() //TODO: Implement this
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    tokentype: TokenType,
    //lexeme: String,
    substring: String,
    line: usize,
    column: usize,
}

impl Token {
    pub fn to_string(&self) -> String {
        String::from(self.tokentype.clone()) //might need to clone here in the future
    } 
}

#[derive(Clone, Debug)]
pub struct Lexer {
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    pub fn scan(&mut self, input: &str) { //Change this return a result type
        while let Some(c) = input.chars().next() {
            self.current += 1;
            if c == '\n' { self.line += 1; }
            let (start, current, line) = (self.start, self.current, self.line);
            let token = |tokentype| {
                Token {
                    tokentype: tokentype,
                    substring: String::from(&input[start..current]),
                    line: line,
                    column: current,
                }
            };

            //matching delimiters
            match c {
                ';' => { self.tokens.push(token(TokenType::StmtEnd)); },
                '(' => { self.tokens.push(token(TokenType::LeftCBkt)); },
                ')' => { self.tokens.push(token(TokenType::RightCBkt)); },
                '[' => { self.tokens.push(token(TokenType::LeftSqBkt)); },
                ']' => { self.tokens.push(token(TokenType::RightSqBkt)); },
                '{' => { self.tokens.push(token(TokenType::OpenBlock)); },
                '}' => { self.tokens.push(token(TokenType::CloseBlock)); },
                ',' => { self.tokens.push(token(TokenType::Comma)); },
                _ => {}
            }
            //scan and accumulate tokens
        }

    }
}
