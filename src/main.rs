use std::{env, fs::File, io::Write};

mod args_parser;
mod commands;
mod parser;

fn main() {
    let mut args = args_parser::Arguments::build(env::args()).unwrap();
    let mut output_file = File::create(args.output_file_path).unwrap();

    for input_file in &mut args.input_file_paths {
        let mut parser = parser::Parser::new(input_file).unwrap();

        while parser.has_more_command() {
            let (c, command) = parser.advance();
            let results = command.generate();

            writeln!(output_file, "// {}", c).unwrap();

            for i in &results {
                writeln!(output_file, "{}", i).unwrap();
            }
        }
    }
}
