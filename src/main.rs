mod lexer;
mod vm;
mod instruction;

use lexer::Lexer;

fn main() {
    let scanner = Lexer::new();
    println!("Welcome to Verdigris!");
}
