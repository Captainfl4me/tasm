use clap::Parser;
use std::fs;
use std::io::prelude::*;

mod cli;
use cli::*;
mod token;

const DEFAULT_OUTPUT_NAME: &str = "out.bin";

fn main() {    
    let cli = Cli::parse();

    match cli.command {
        Commands::Assemble(args) => {
            let output_file_path = args.output.unwrap_or(DEFAULT_OUTPUT_NAME.to_string());

            println!("Assembling: {}", args.source);
            let source_code = fs::read_to_string(args.source).unwrap();
            if let Some(intermediate_representation) = token::IntermediateRepresentation::new(source_code.as_str()) {
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
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_break() {
        let intermediate_representation_opt = token::IntermediateRepresentation::new("halt\n");
        assert!(intermediate_representation_opt.is_some());

        let intermediate_representation = intermediate_representation_opt.unwrap();
        assert_eq!(intermediate_representation.bytes_size(), 1);
        assert_eq!(intermediate_representation.to_bytes(), vec![0]);
    }

    #[test]
    fn test_load_immediate() {
        let intermediate_representation_opt = token::IntermediateRepresentation::new("load x,#255\r\nhalt\r\n");
        assert!(intermediate_representation_opt.is_some());

        let intermediate_representation = intermediate_representation_opt.unwrap();
        assert_eq!(intermediate_representation.bytes_size(), 3);
        assert_eq!(intermediate_representation.to_bytes(), vec![0b00010001, 255, 0]);
    }

    #[test]
    fn test_load_relative() {
        let intermediate_representation_opt = token::IntermediateRepresentation::new("load x,$1234\r\nhalt\r\n");
        assert!(intermediate_representation_opt.is_some());

        let intermediate_representation = intermediate_representation_opt.unwrap();
        assert_eq!(intermediate_representation.bytes_size(), 4);
        assert_eq!(intermediate_representation.to_bytes(), vec![0b00011001, 0x34, 0x12, 0]);
    }
}

