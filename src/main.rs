use std::env;

mod args_parser;
mod commands;
mod parser;

fn main() {
    let args = args_parser::Arguments::build(env::args()).unwrap();
    let mut parser = parser::Parser::new(&args.input_file_path.as_path()).unwrap();
    let mut code = vec![];

    while parser.has_more_command() {
        let command = parser.advance();
        code.extend(command.generate());
    }

    for i in code {
        println!("{i}");
    }
}
