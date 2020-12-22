mod lexer;
mod vm;
mod repl;

use lexer::Lexer;

fn main() {
    let scanner = Lexer::new();
    println!("Welcome to Verdigris!");
}
