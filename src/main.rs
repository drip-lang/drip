use crate::parser::Parser;
use std::io;
use std::io::Write;

pub mod lexer;
pub mod parser;
pub mod repl;
pub mod syntax;

fn main() -> color_eyre::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        let parse = Parser::new(&input).parse()?;
        println!("{}", parse.debug_tree());

        input.clear();
    }
}
