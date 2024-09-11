use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Copy)]
#[repr(u8)]
enum Opcode {
    Break = 0,
    Load = 1,
    Transfer = 2,
    Store = 3,
    Push = 4,
    Pull = 5,
    Math = 6,
    Jump = 7,
}

#[derive(Clone, Copy)]
#[repr(u8)]
enum AddressingMode {
    Immediate = 0,
    Relative = 1,
}

#[derive(Clone, Copy)]
#[repr(u8)]
enum Registers {
    Accumulator = 0,
    XReg = 1,
    YReg = 2,
}

#[derive(Clone, Copy)]
#[repr(u8)]
enum MathOperand {
    Increment = 0,
    Add = 1,
    Sub = 2,
    ShiftLeft = 3,
    ShiftRight = 4,
}

#[derive(Clone, Copy)]
#[repr(u8)]
enum BranchCondition {
    CarryFlag = 0,
    ZeroFlag = 1,
    NegativeFlag = 2,
    OverflowFlag = 3,
}

#[derive(Clone, Copy)]
#[repr(u8)]
enum InstructionData {
    NoData = 0,
    Registers(Registers),
    MathOperand(MathOperand),
    BranchCondition(BranchCondition),
    DoubleRegisters(Registers, Registers),
}

struct Instruction<'a> {
    opcode: Opcode,
    addressing_mode: AddressingMode,
    data: InstructionData,
    size: u16,
    linked_data: Option<InstructionLinkedData<'a>>,
}
impl<'a> Instruction<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let instruction_data: u8 = match &self.data {
            InstructionData::NoData => 0,
            InstructionData::Registers(reg) => *reg as u8,
            InstructionData::MathOperand(op) => *op as u8,
            InstructionData::BranchCondition(br) => *br as u8,
            InstructionData::DoubleRegisters(reg_0, reg_1) => {
                (*reg_0 as u8) | ((*reg_1 as u8) << 2)
            }
        };
        let instruction =
            (self.opcode as u8) | ((self.addressing_mode as u8) << 3) | (instruction_data << 4);
        let mut bytes_vec = vec![instruction];

        if let Some(data) = &self.linked_data {
            match data {
                InstructionLinkedData::Immediate(val) => {
                    bytes_vec.append(&mut val.to_le_bytes().to_vec())
                }
                InstructionLinkedData::Relative(val) => {
                    bytes_vec.append(&mut val.to_le_bytes().to_vec())
                }
                InstructionLinkedData::NotResolvedRelative(_) => {
                    bytes_vec.append(&mut vec![0u8, 0u8])
                }
            }
        }

        bytes_vec
    }
}
enum InstructionLinkedData<'a> {
    Immediate(u8),
    Relative(u16),
    NotResolvedRelative(&'a str),
}

fn parse_label_name(str: &str) -> Option<&str> {
    let re = Regex::new(r"^[a-z_-]*:").unwrap();
    if re.captures(str).is_some() {
        let label_name = str.split(":").collect::<Vec<&str>>()[0];
        return Some(label_name);
    }
    None
}

fn parse_org_flag(str: &str) -> Option<u16> {
    let re = Regex::new(r"^.org\s$[0-9]").unwrap();
    if re.captures(str).is_some() {
        let org_addr = str.split("$").collect::<Vec<&str>>()[1];
        return Some(org_addr.parse::<u16>().unwrap());
    }
    None
}

fn parse_instruction(str: &str) -> Option<Instruction> {
    let line_splited = str.split(" ").collect::<Vec<&str>>();
    let keyword = line_splited[0];
    match keyword {
        "halt" => Some(Instruction {
            opcode: Opcode::Break,
            addressing_mode: AddressingMode::Immediate,
            data: InstructionData::NoData,
            size: 1,
            linked_data: None,
        }),
        "load" => {
            let (register_str, data_str) = line_splited[1].split_once(",").unwrap();
            let register: Option<Registers> = match register_str {
                "x" => Some(Registers::XReg),
                "y" => Some(Registers::YReg),
                "a" => Some(Registers::Accumulator),
                _ => None,
            };
            register.as_ref()?;

            let addressing_mode;
            let linked_data;
            let data_trimmed = data_str.trim_ascii().to_string();
            match data_trimmed.chars().next().unwrap() {
                '#' => {
                    let value = data_trimmed.replace("#", "").parse::<u8>().unwrap();
                    addressing_mode = AddressingMode::Immediate;
                    linked_data = InstructionLinkedData::Immediate(value);
                }
                '$' => {
                    let value = data_trimmed.replace("$", "").parse::<u16>().unwrap();
                    addressing_mode = AddressingMode::Relative;
                    linked_data = InstructionLinkedData::Relative(value);
                }
                _ => {
                    addressing_mode = AddressingMode::Relative;
                    linked_data = InstructionLinkedData::NotResolvedRelative(data_str.trim_ascii());
                }
            }
            Some(Instruction {
                opcode: Opcode::Load,
                data: InstructionData::Registers(register.unwrap()),
                size: match addressing_mode {
                    AddressingMode::Relative => 3,
                    AddressingMode::Immediate => 2,
                },
                linked_data: Some(linked_data),
                addressing_mode,
            })
        }
        "tf" => {
            let (register_str_1, register_str_2) = line_splited[1].split_once(",").unwrap();
            let register_1: Option<Registers> = match register_str_1 {
                "x" => Some(Registers::XReg),
                "y" => Some(Registers::YReg),
                "a" => Some(Registers::Accumulator),
                _ => None,
            };
            register_1.as_ref()?;
            let register_2: Option<Registers> = match register_str_2 {
                "x" => Some(Registers::XReg),
                "y" => Some(Registers::YReg),
                "a" => Some(Registers::Accumulator),
                _ => None,
            };
            register_2.as_ref()?;

            Some(Instruction {
                opcode: Opcode::Load,
                addressing_mode: AddressingMode::Immediate,
                data: InstructionData::DoubleRegisters(register_1.unwrap(), register_2.unwrap()),
                size: 1,
                linked_data: None,
            })
        }
        _ => None,
    }
}

pub struct IntermediateRepresentation<'a> {
    labels: HashMap<&'a str, u16>,
    instructions: HashMap<u16, Instruction<'a>>,
}

impl<'a> IntermediateRepresentation<'a> {
    pub fn new(str: &'a str) -> Option<Self> {
        let mut labels: HashMap<&'a str, u16> = HashMap::new();
        let mut instructions: HashMap<u16, Instruction<'a>> = HashMap::new();

        let mut current_addr: u16 = 0;
        for line in str.lines() {
            if let Some(label) = parse_label_name(line) {
                labels.insert(label, current_addr);
            } else if let Some(org_addr) = parse_org_flag(line) {
                current_addr = org_addr;
            } else if let Some(instruction) = parse_instruction(line) {
                let incr_addr = instruction.size;
                instructions.insert(current_addr, instruction);
                current_addr += incr_addr;
            } else {
                eprintln!("ERR: Unknown line of code: {}", line);
                return None;
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
            labels,
            instructions,
        })
    }

    pub fn bytes_size(&self) -> u16 {
        let min_instruction_address = self.instructions.keys().min().unwrap();
        let max_instruction_address = self.instructions.keys().max().unwrap();

        max_instruction_address - min_instruction_address + 1
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
                ..(addr - offset_addr + current_instruction.size) as usize;

            memory[bytes_addr_slice].copy_from_slice(bytes_rep.as_slice());
        }

        memory
    }
}