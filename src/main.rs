use drip_parser::parse;
use std::io;
use std::io::Write;

fn main() -> color_eyre::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        let parse = parse(&input);
        println!("{}", parse.debug_tree());

        let syntax = parse.syntax();

        for error in drip_ast::validation::validate(&syntax) {
            println!("{}", error);
        }

        let root = drip_ast::Root::cast(syntax).unwrap();

        dbg!(root
            .stmts()
            .filter_map(|stmt| if let drip_ast::Stmt::VariableDef(var_def) = stmt {
                Some(var_def.value())
            } else {
                None
            })
            .collect::<Vec<_>>());

        input.clear();
    }
}
