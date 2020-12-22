mod lexer;

use lexer::Lexer;
use vdg_oxidizer::Repl;

fn main() {
    let scanner = Lexer::new();
    let mut repl = Repl::new();
    repl.run();
}
