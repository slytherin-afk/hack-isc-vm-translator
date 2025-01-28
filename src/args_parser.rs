use std::path::{Path, PathBuf};

pub struct Arguments {
    pub input_file_paths: Vec<PathBuf>,
    pub output_file_path: PathBuf,
}

impl Arguments {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        let mut output_file_path = None;
        let mut input_file_paths = vec![];
        let mut iterator = args.into_iter().skip(1);

        while let Some(arg) = iterator.next() {
            if arg == "-o" {
                output_file_path = iterator.next()
            } else {
                input_file_paths.push(Path::new(&arg).to_owned())
            }
        }

        if input_file_paths.len() <= 0 {
            return Err("Required a input file");
        }

        let output_file_path = output_file_path.expect("Output file path");

        Ok(Arguments {
            input_file_paths,
            output_file_path: Path::new(&output_file_path).to_owned(),
        })
    }
}
