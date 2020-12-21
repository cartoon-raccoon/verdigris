mod lexer;
mod vm;

use lexer::Lexer;

fn main() {
    let scanner = Lexer::new();
    println!("Welcome to Verdigris!");
}
