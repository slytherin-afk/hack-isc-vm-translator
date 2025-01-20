use std::path::{Path, PathBuf};

pub struct Arguments {
    pub input_file_path: PathBuf,
    pub output_file_path: PathBuf,
}

impl Arguments {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        let mut output_file_path = None;
        let mut input_file_path = None;
        let mut iterator = args.into_iter();

        while let Some(arg) = iterator.next() {
            if arg == "-o" {
                output_file_path = iterator.next()
            } else {
                input_file_path = Some(arg);
            }
        }

        let input_file_path = input_file_path.expect("Input file path");
        let output_file_path = output_file_path.expect("Output file path");

        Ok(Arguments {
            input_file_path: Path::new(&input_file_path).to_owned(),
            output_file_path: Path::new(&output_file_path).to_owned(),
        })
    }
}
