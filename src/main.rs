use clap::Parser;
use std::fs;
use std::io::prelude::*;

mod cli;
use cli::*;
mod parser;

const DEFAULT_OUTPUT_NAME: &str = "out";

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Assemble(args) => {
            if let Some(intermediate_representation) =
                parser::IntermediateRepresentation::new(&args.source)
            {
                if args.coe {
                    let output_file_path = args
                        .output
                        .unwrap_or(DEFAULT_OUTPUT_NAME.to_string() + ".coe");
                    println!("INFO: Writing coe output to: {}", output_file_path);

                    let mut output_file =
                        std::io::LineWriter::new(fs::File::create(output_file_path).unwrap());
                    output_file
                        .write_all(
                            b"memory_initialization_radix=16;\nmemory_initialization_vector=\n",
                        )
                        .ok()
                        .unwrap();
                    let file_bytes = intermediate_representation.to_bytes();
                    for (idx, byte) in file_bytes.iter().enumerate() {
                        if idx == file_bytes.len() - 1 {
                            output_file
                                .write_all(&format!("{:#04x};\n", byte).as_bytes()[2..])
                                .ok()
                                .unwrap();
                        } else {
                            output_file
                                .write_all(&format!("{:#04x},\n", byte).as_bytes()[2..])
                                .ok()
                                .unwrap();
                        }
                    }
                } else {
                    let output_file_path = args
                        .output
                        .unwrap_or(DEFAULT_OUTPUT_NAME.to_string() + ".bin");
                    println!("INFO: Writing bin output to: {}", output_file_path);
                    let mut output_file = fs::File::create(output_file_path).unwrap();
                    output_file
                        .write_all(intermediate_representation.to_bytes().as_slice())
                        .unwrap();
                }
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
        let intermediate_representation_opt =
            parser::IntermediateRepresentation::new("./test/test.tasm");
        assert!(intermediate_representation_opt.is_some());

        let intermediate_representation = intermediate_representation_opt.unwrap();
        assert_eq!(intermediate_representation.bytes_size(), 6);
        assert_eq!(
            intermediate_representation.to_bytes(),
            vec![0b00011001, 0x34, 0x12, 0b00010001, 255, 0]
        );
    }
}
