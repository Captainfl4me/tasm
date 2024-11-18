use std::collections::HashMap;
use std::fs::{read_to_string, exists};
use std::path::PathBuf;

mod lexer;
use lexer::*;

pub struct IntermediateRepresentation {
    labels: HashMap<String, u16>,
    instructions: HashMap<u16, Instruction>,
}

impl IntermediateRepresentation {
    pub fn new(str: &str) -> Option<Self> {
        IntermediateRepresentation::parse(str, 0)
    }

    fn parse(str: &str, offset_addr: u16) -> Option<Self> {
        let file_path = exists(str);
        if let Ok(exist) = file_path {
            if !exist {
                eprintln!("ERR: Path does not exist: {}", str);
                return None;
            }
        } else {
            eprintln!("ERR: Path cannot be determine: {}", str);
            return None;
        }

        let source_code = read_to_string(str).unwrap();
        let parent_dir_path = {
            let mut path = PathBuf::from(str);
            path.pop();
            path
        };

        let mut labels: HashMap<String, u16> = HashMap::new();
        let mut instructions: HashMap<u16, Instruction> = HashMap::new();

        let mut current_addr: u16 = offset_addr;
        for (line_index, line_raw) in source_code.lines().enumerate() {
            match lex_line(line_raw) {
                Ok(token_opt) => {
                    if let Some(token) = token_opt {
                        match token {
                            TokenType::Instruction(instruction) => {
                                let incr_addr = instruction.size;
                                instructions.insert(current_addr, instruction);
                                current_addr += incr_addr;
                            },
                            TokenType::Flag(flag) => {
                                match flag {
                                    Flag::Org(addr) => {
                                        current_addr = addr;
                                    },
                                    Flag::Include(path_str) => {
                                        let mut include_full_path = parent_dir_path.clone();
                                        include_full_path.push(path_str);
                                        let path_str = include_full_path.to_str().unwrap();
                                        println!("INFO: Compiling file {}", path_str);
                                        let nested_representation_opt = IntermediateRepresentation::parse(path_str, current_addr);
                                        if let Some(nested_representation) = nested_representation_opt {
                                            current_addr += nested_representation.bytes_size();
                                            labels.extend(nested_representation.labels);
                                            instructions.extend(nested_representation.instructions);
                                        }
                                    }
                                }
                            },
                            TokenType::Label(label) => {
                                labels.insert(label.name.to_string(), current_addr);
                                println!("INFO: label {} at current_addr {:#02x}", label.name, current_addr);
                            },
                        }

                    } else {
                        continue;
                    }
                },
                Err(err_str) => {
                    eprintln!("ERR: {} | at line {}", err_str, line_index);
                }
            }
        }

        for instruction in instructions.values_mut() {
            if let Some(InstructionLinkedData::NotResolvedRelative(label)) = &instruction.linked_data
            {
                if let Some(label_addr) = labels.get(label) {
                    instruction.linked_data = Some(InstructionLinkedData::Relative(*label_addr));
                } else {
                    eprintln!("ERR: Label {} didn't exist!", label);
                }
            }
        }

        Some(Self {
            labels,
            instructions,
        })
    }

    pub fn bytes_size(&self) -> u16 {
        if self.instructions.is_empty() {
            0
        } else {
            let min_instruction_address = self.instructions.keys().min().unwrap();
            let max_instruction_address = self.instructions.keys().max().unwrap();

            (max_instruction_address - min_instruction_address) + self.instructions[max_instruction_address].size
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let size = self.bytes_size() as usize;
        let addr_map = {
            let mut vec = self.instructions.keys().collect::<Vec<_>>();
            vec.sort();
            vec
        };
        let mut memory = vec![0; size];

        let offset_addr = addr_map[0];
        for addr in addr_map {
            let current_instruction = self.instructions.get(addr).unwrap();
            let bytes_rep = current_instruction.to_bytes();
            let bytes_addr_slice = (addr - offset_addr) as usize
                ..((addr - offset_addr) as usize + bytes_rep.len());

            memory[bytes_addr_slice].copy_from_slice(bytes_rep.as_slice());
        }

        memory
    }
}
