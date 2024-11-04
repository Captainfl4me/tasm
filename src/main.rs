use clap::Parser;
use std::fs;
use std::io::prelude::*;

mod cli;
use cli::*;
mod parser;

const DEFAULT_OUTPUT_NAME: &str = "out.bin";

fn main() {    
    let cli = Cli::parse();

    match cli.command {
        Commands::Assemble(args) => {
            let output_file_path = args.output.unwrap_or(DEFAULT_OUTPUT_NAME.to_string());

            if let Some(intermediate_representation) = parser::IntermediateRepresentation::new(&args.source) {
            let mut output_file = fs::File::create(output_file_path).unwrap();
                output_file.write_all(intermediate_representation.to_bytes().as_slice()).unwrap();
            } else {
                eprintln!("ERR: Cannot parse source code!");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_assembling() {
        let intermediate_representation_opt = parser::IntermediateRepresentation::new("./test/test.tasm");
        assert!(intermediate_representation_opt.is_some());

        let intermediate_representation = intermediate_representation_opt.unwrap();
        assert_eq!(intermediate_representation.bytes_size(), 6);
        assert_eq!(intermediate_representation.to_bytes(), vec![0b00011001, 0x34, 0x12, 0b00010001, 255, 0]);
    }
}

