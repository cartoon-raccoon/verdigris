use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum TokenType {
    //core keywords
    Let, Struct, Function, Trait, 
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
    Dot,        // .

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
        String::from(self.tokentype.clone())
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
        let mut keywords = HashMap::<&str, TokenType>::new();

        keywords.insert("let ", TokenType::Let);
        keywords.insert("struct ", TokenType::Struct);
        keywords.insert("fxn ", TokenType::Function);
        keywords.insert("trait ", TokenType::Trait);
        keywords.insert("impl ", TokenType::Impl);
        keywords.insert("enum ", TokenType::Enum);
        keywords.insert("return ", TokenType::Return);
        keywords.insert("self ", TokenType::This);

        let mut buffer = String::new();
        let mut prevc: char;
        while let Some(c) = input.chars().next() {
            //TODO: Add peeking to next char
            self.current += 1;
            if c == '\n' { 
                self.line += 1;
                buffer.clear();
                continue; 
            }
            let (start, current, line) = (self.start, self.current, self.line);
            let token = |tokentype| {
                Token {
                    tokentype: tokentype,
                    substring: String::from(&input[start..current]),
                    line: line,
                    column: current,
                }
            };
            
            buffer.push(c);
            //matching delimiters
            match buffer.as_str() {
                ";" => { self.tokens.push(token(TokenType::StmtEnd)); },
                "(" => { self.tokens.push(token(TokenType::LeftCBkt)); },
                ")" => { self.tokens.push(token(TokenType::RightCBkt)); },
                "[" => { self.tokens.push(token(TokenType::LeftSqBkt)); },
                "]" => { self.tokens.push(token(TokenType::RightSqBkt)); },
                "{" => { self.tokens.push(token(TokenType::OpenBlock)); },
                "}" => { self.tokens.push(token(TokenType::CloseBlock)); },
                "," => { self.tokens.push(token(TokenType::Comma)); },
                "." => { self.tokens.push(token(TokenType::Dot)); },
                _ => {}
            }

            if keywords.contains_key(buffer.as_str()) {
                self.tokens.push(token(keywords[buffer.as_str()].clone()));
                buffer.clear();
            }

            prevc = c;
            //scan and accumulate tokens
        }

        //Adding EOF token to designate end of parsing
        self.tokens.push(Token {
            tokentype: TokenType::EOF,
            substring: String::from(&input[self.start..self.current]),
            line: self.line,
            column: self.current,
        });

    }
}
