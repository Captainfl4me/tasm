use std::collections::HashMap;

mod lexer;
use lexer::*;

pub struct IntermediateRepresentation<'a> {
    instructions: HashMap<u16, Instruction<'a>>,
}

impl<'a> IntermediateRepresentation<'a> {
    pub fn new(str: &'a str) -> Option<Self> {
        let mut labels: HashMap<&'a str, u16> = HashMap::new();
        let mut instructions: HashMap<u16, Instruction<'a>> = HashMap::new();

        let mut current_addr: u16 = 0;
        for (line_index, line_raw) in str.lines().enumerate() {
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
                                    }
                                }
                            },
                            TokenType::Label(label) => {
                                labels.insert(label.name, current_addr);
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
            if let Some(InstructionLinkedData::NotResolvedRelative(label)) = instruction.linked_data
            {
                if let Some(label_addr) = labels.get(label) {
                    instruction.linked_data = Some(InstructionLinkedData::Relative(*label_addr));
                } else {
                    eprintln!("ERR: Label {} didn't exist!", label);
                    return None;
                }
            }
        }

        Some(Self {
            instructions,
        })
    }

    pub fn bytes_size(&self) -> u16 {
        let min_instruction_address = self.instructions.keys().min().unwrap();
        let max_instruction_address = self.instructions.keys().max().unwrap();

        (max_instruction_address - min_instruction_address) + self.instructions[max_instruction_address].size
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let size = self.bytes_size() as usize;
        let addr_map = {
            let mut vec = self.instructions.keys().collect::<Vec<_>>();
            vec.sort();
            vec
        };
        let mut memory = vec![0; size];
        println!("INFO: Memory of size {}", size);

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
