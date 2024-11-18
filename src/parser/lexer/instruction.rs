use super::generic::parse_number;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Opcode {
    Break = 0,
    Load = 1,
    Transfer = 2,
    Store = 3,
    Push = 4,
    Pull = 5,
    Math = 6,
    Jump = 7,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum AddressingMode {
    Immediate = 0,
    Relative = 1,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Registers {
    Ra = 0,
    Rx = 1,
    Ry = 2,
    Rb = 3,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum MathOperand {
    Increment = 0,
    Add = 1,
    Sub = 2,
    And = 3,
    Or = 4,
    Eor = 5,
    ShiftLeft = 6,
    ShiftRight = 7,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum BranchCondition {
    NoCondition = 0,
    CarryFlagClear = 1,
    CarryFlagSet = 2,
    ZeroFlagClear = 3,
    ZeroFlagSet = 4,
    NegativeFlagClear = 5,
    NegativeFlagSet = 6,
    OverflowFlagClear = 7,
    OverflowFlagSet = 8,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum InstructionData {
    NoData = 0,
    Registers(Registers),
    MathOperand(MathOperand),
    BranchCondition(BranchCondition),
    DoubleRegisters(Registers, Registers),
}

pub enum InstructionLinkedData {
    Immediate(u8),
    Relative(u16),
    NotResolvedRelative(String),
}

pub struct Instruction {
    pub opcode: Opcode,
    pub addressing_mode: AddressingMode,
    pub data: InstructionData,
    pub size: u16,
    pub linked_data: Option<InstructionLinkedData>,
}
impl Instruction {
    pub fn new(str: &str) -> Result<Option<Self>, String> {
        let (keyword, data) = {
            let line_splited = str.split(" ").collect::<Vec<&str>>();
            let data = line_splited.get(1).map(|s| s.trim_ascii());

            (line_splited[0], data)
        };

        match keyword {
            "halt" => {
                if data.is_some() {
                    return Err("Data part of instruction should be none".to_string());
                }

                Ok(Some(Instruction {
                opcode: Opcode::Break,
                addressing_mode: AddressingMode::Immediate,
                data: InstructionData::NoData,
                size: 1,
                linked_data: None,
                }))
            },
            "load" => {
                if data.is_none() {
                    return Err("Data part of instruction is none".to_string());
                }

                if let Some((register_str, data_str)) = data.unwrap().split_once(",") {
                    let register: Option<Registers> = match register_str {
                        "rx" => Some(Registers::Rx),
                        "ry" => Some(Registers::Ry),
                        "ra" => Some(Registers::Ra),
                        "rb" => Some(Registers::Rb),
                        _ => None,
                    };
                    if register.is_none() {
                        return Err(format!("Unknow register: {}", register_str));
                    }
                    if data_str.is_empty() {
                        return Err("Value to load is none".to_string());
                    }

                    let addressing_mode;
                    let linked_data;
                    let data_trimmed = data_str.trim_ascii().to_string();
                    if data_trimmed.starts_with("#") {
                        if let Some(value) =
                            parse_number::<u8>(data_trimmed.replace("#", "").as_str())
                        {
                            addressing_mode = AddressingMode::Immediate;
                            linked_data = InstructionLinkedData::Immediate(value);
                        } else {
                            return Err("Immediate value cannot be parsed".to_string());
                        }
                    } else if let Some(value) =
                        parse_number::<u16>(data_trimmed.replace("#", "").as_str())
                    {
                        addressing_mode = AddressingMode::Relative;
                        linked_data = InstructionLinkedData::Relative(value);
                    } else {
                        addressing_mode = AddressingMode::Relative;
                        linked_data = InstructionLinkedData::NotResolvedRelative(
                            data_str.trim_ascii().to_string(),
                        );
                    }
                    Ok(Some(Instruction {
                        opcode: Opcode::Load,
                        data: InstructionData::Registers(register.unwrap()),
                        size: match addressing_mode {
                            AddressingMode::Relative => 3,
                            AddressingMode::Immediate => 2,
                        },
                        linked_data: Some(linked_data),
                        addressing_mode,
                    }))
                } else {
                    Err("format LOAD <reg>,<#value/address> is not matched!".to_string())
                }
            }
            "tf" => {
                if data.is_none() {
                    return Err("Data part of instruction is none".to_string());
                }

                if let Some((register_str_1, register_str_2)) = data.unwrap().split_once(",") {
                    let register_1: Option<Registers> = match register_str_1 {
                        "rx" => Some(Registers::Rx),
                        "ry" => Some(Registers::Ry),
                        "ra" => Some(Registers::Ra),
                        "rb" => Some(Registers::Rb),
                        _ => None,
                    };
                    if register_1.is_none() {
                        return Err(format!("Unknow register: {}", register_str_1));
                    }

                    let register_2: Option<Registers> = match register_str_2 {
                        "rx" => Some(Registers::Rx),
                        "ry" => Some(Registers::Ry),
                        "ra" => Some(Registers::Ra),
                        "rb" => Some(Registers::Rb),
                        _ => None,
                    };
                    if register_2.is_none() {
                        return Err(format!("Unknow register: {}", register_str_2));
                    }

                    Ok(Some(Instruction {
                        opcode: Opcode::Transfer,
                        addressing_mode: AddressingMode::Immediate,
                        data: InstructionData::DoubleRegisters(
                            register_1.unwrap(),
                            register_2.unwrap(),
                        ),
                        size: 1,
                        linked_data: None,
                    }))
                } else {
                    Err("format TF <reg>,<reg> is not matched!".to_string())
                }
            }
            "store" => {
                if data.is_none() {
                    return Err("Data part of instruction is none".to_string());
                }

                if let Some((register_str, data_str)) = data.unwrap().split_once(",") {
                    let register: Option<Registers> = match register_str {
                        "rx" => Some(Registers::Rx),
                        "ry" => Some(Registers::Ry),
                        "ra" => Some(Registers::Ra),
                        "rb" => Some(Registers::Rb),
                        _ => None,
                    };
                    if register.is_none() {
                        return Err(format!("Unknow register: {}", register_str));
                    }
                    if data_str.is_empty() {
                        return Err("Value of address is none".to_string());
                    }

                    let addressing_mode;
                    let linked_data;
                    let data_trimmed = data_str.trim_ascii().to_string();
                    if data_trimmed.starts_with("#") {
                        return Err("Store cannot be immediate, expect address".to_string());
                    } else if let Some(value) =
                        parse_number::<u16>(data_trimmed.replace("#", "").as_str())
                    {
                        addressing_mode = AddressingMode::Relative;
                        linked_data = InstructionLinkedData::Relative(value);
                    } else {
                        addressing_mode = AddressingMode::Relative;
                        linked_data = InstructionLinkedData::NotResolvedRelative(
                            data_str.trim_ascii().to_string(),
                        );
                    }
                    Ok(Some(Instruction {
                        opcode: Opcode::Store,
                        data: InstructionData::Registers(register.unwrap()),
                        size: 3,
                        linked_data: Some(linked_data),
                        addressing_mode,
                    }))
                } else {
                    Err("format STORE <reg>,<address> is not matched!".to_string())
                }
            }
            "push" => {
                if data.is_none() {
                    return Err("Data part of instruction is none".to_string());
                }

                let register_str = data.unwrap();
                let register: Option<Registers> = match register_str {
                    "rx" => Some(Registers::Rx),
                    "ry" => Some(Registers::Ry),
                    "ra" => Some(Registers::Ra),
                    "rb" => Some(Registers::Rb),
                    _ => None,
                };
                if register.is_none() {
                    return Err(format!("Unknow register: {}", register_str));
                }

                Ok(Some(Instruction {
                    opcode: Opcode::Push,
                    data: InstructionData::Registers(register.unwrap()),
                    size: 1,
                    linked_data: None,
                    addressing_mode: AddressingMode::Immediate,
                }))
            }
            "pull" => {
                if data.is_none() {
                    return Err("Data part of instruction is none".to_string());
                }

                let register_str = data.unwrap();
                let register: Option<Registers> = match register_str {
                    "rx" => Some(Registers::Rx),
                    "ry" => Some(Registers::Ry),
                    "ra" => Some(Registers::Ra),
                    "rb" => Some(Registers::Rb),
                    _ => None,
                };
                if register.is_none() {
                    return Err(format!("Unknow register: {}", register_str));
                }

                Ok(Some(Instruction {
                    opcode: Opcode::Pull,
                    data: InstructionData::Registers(register.unwrap()),
                    size: 1,
                    linked_data: None,
                    addressing_mode: AddressingMode::Immediate,
                }))
            }
            "incr" | "add" | "sub" | "and" | "or" | "eor" | "shift_right" | "shift_left" => {
                if data.is_some() {
                    return Err(
                        "Math operand only apply between fixed register from file (RA and RB)"
                            .to_string(),
                    );
                }

                let math_op = match keyword {
                    "incr" => Some(MathOperand::Increment),
                    "add" => Some(MathOperand::Add),
                    "sub" => Some(MathOperand::Sub),
                    "and" => Some(MathOperand::And),
                    "or" => Some(MathOperand::Or),
                    "eor" => Some(MathOperand::Eor),
                    "shift_right" => Some(MathOperand::ShiftRight),
                    "shift_left" => Some(MathOperand::ShiftLeft),
                    _ => None,
                };

                Ok(Some(Instruction {
                    opcode: Opcode::Math,
                    data: InstructionData::MathOperand(math_op.unwrap()),
                    size: 1,
                    linked_data: None,
                    addressing_mode: AddressingMode::Immediate,
                }))
            }
            "jump" | "bcc" | "bcs" | "bzc" | "bzs" | "bnc" | "bns" | "boc" | "bos" => {
                let addressing_mode;
                let linked_data;
                let data_trimmed = data.unwrap().to_string();
                if data_trimmed.starts_with("#") {
                    return Err("Jump cannot be immediate, expect address".to_string());
                } else if let Some(value) =
                    parse_number::<u16>(data_trimmed.replace("#", "").as_str())
                {
                    addressing_mode = AddressingMode::Relative;
                    linked_data = Some(InstructionLinkedData::Relative(value));
                } else {
                    addressing_mode = AddressingMode::Relative;
                    linked_data = Some(InstructionLinkedData::NotResolvedRelative(
                        data.unwrap().to_string(),
                    ));
                }

                let branch_condition = match keyword {
                    "jump" => Some(BranchCondition::NoCondition),
                    "bcc" => Some(BranchCondition::CarryFlagClear),
                    "bcs" => Some(BranchCondition::CarryFlagSet),
                    "bzc" => Some(BranchCondition::ZeroFlagClear),
                    "bzs" => Some(BranchCondition::ZeroFlagSet),
                    "bnc" => Some(BranchCondition::NegativeFlagClear),
                    "bns" => Some(BranchCondition::NegativeFlagSet),
                    "boc" => Some(BranchCondition::OverflowFlagClear),
                    "bos" => Some(BranchCondition::OverflowFlagSet),
                    _ => None,
                };

                Ok(Some(Instruction {
                    opcode: Opcode::Jump,
                    data: InstructionData::BranchCondition(branch_condition.unwrap()),
                    size: 3,
                    linked_data,
                    addressing_mode,
                }))
            }
            _ => Ok(None),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrong_inst() {
        let inst = Instruction::new("azerty");
        assert!(inst.is_ok());
        assert!(inst.ok().unwrap().is_none());
    }

    #[test]
    fn test_halt() {
        let inst = Instruction::new("halt");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0]);
        assert_eq!(inst.size, 1);
    }

    #[test]
    fn test_load() {
        let inst = Instruction::new("load");
        assert!(inst.is_err());

        let inst = Instruction::new("load aze");
        assert!(inst.is_err());

        let inst = Instruction::new("load rz,#300");
        assert!(inst.is_err());

        let inst = Instruction::new("load rx,");
        assert!(inst.is_err());

        let inst = Instruction::new("load rx,#300");
        assert!(inst.is_err());

        let inst = Instruction::new("load ra,#5");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![1, 5]);
        assert_eq!(inst.size, 2);

        let inst = Instruction::new("load ra,$abac");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b1001, 0xac, 0xab]);
        assert_eq!(inst.size, 3);

        let inst = Instruction::new("load ra,flag_test");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b1001, 0, 0]);
        assert_eq!(inst.size, 3);
    }

    #[test]
    fn test_transfer() {
        let inst = Instruction::new("tf");
        assert!(inst.is_err());

        let inst = Instruction::new("tf aze");
        assert!(inst.is_err());

        let inst = Instruction::new("tf rx,");
        assert!(inst.is_err());

        let inst = Instruction::new("tf rx,rz");
        assert!(inst.is_err());

        let inst = Instruction::new("tf rz,rx");
        assert!(inst.is_err());

        let inst = Instruction::new("tf rx,ry");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b10010010]);
        assert_eq!(inst.size, 1);

        let inst = Instruction::new("tf ra,rb");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b11000010]);
        assert_eq!(inst.size, 1);

        let inst = Instruction::new("tf ra,ra");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b00000010]);
        assert_eq!(inst.size, 1);
    }

    #[test]
    fn test_store() {
        let inst = Instruction::new("store");
        assert!(inst.is_err());

        let inst = Instruction::new("store rx,");
        assert!(inst.is_err());

        let inst = Instruction::new("store rx,#5");
        assert!(inst.is_err());

        let inst = Instruction::new("store rz,#5");
        assert!(inst.is_err());

        let inst = Instruction::new("store rz,flag_test");
        assert!(inst.is_err());

        let inst = Instruction::new("store ra,flag_test");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b00001011, 0, 0]);
        assert_eq!(inst.size, 3);

        let inst = Instruction::new("store ra,$abac");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b00001011, 0xac, 0xab]);
        assert_eq!(inst.size, 3);
    }

    #[test]
    fn test_push() {
        let inst = Instruction::new("push");
        assert!(inst.is_err());

        let inst = Instruction::new("push rz");
        assert!(inst.is_err());

        let inst = Instruction::new("push #5");
        assert!(inst.is_err());

        let inst = Instruction::new("push ra");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b00000100]);
        assert_eq!(inst.size, 1);

        let inst = Instruction::new("push rx");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b00010100]);
        assert_eq!(inst.size, 1);

        let inst = Instruction::new("push ry");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b00100100]);
        assert_eq!(inst.size, 1);

        let inst = Instruction::new("push rb");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b00110100]);
        assert_eq!(inst.size, 1);
    }

    #[test]
    fn test_pull() {
        let inst = Instruction::new("pull");
        assert!(inst.is_err());

        let inst = Instruction::new("pull rz");
        assert!(inst.is_err());

        let inst = Instruction::new("pull #5");
        assert!(inst.is_err());

        let inst = Instruction::new("pull ra");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b00000101]);
        assert_eq!(inst.size, 1);

        let inst = Instruction::new("pull rx");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b00010101]);
        assert_eq!(inst.size, 1);

        let inst = Instruction::new("pull ry");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b00100101]);
        assert_eq!(inst.size, 1);

        let inst = Instruction::new("pull rb");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b00110101]);
        assert_eq!(inst.size, 1);
    }

    #[test]
    fn test_math() {
        let inst = Instruction::new("add ra");
        assert!(inst.is_err());

        let inst = Instruction::new("incr");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b00000110]);
        assert_eq!(inst.size, 1);

        let inst = Instruction::new("add");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b00010110]);
        assert_eq!(inst.size, 1);

        let inst = Instruction::new("sub");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b00100110]);
        assert_eq!(inst.size, 1);

        let inst = Instruction::new("and");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b00110110]);
        assert_eq!(inst.size, 1);

        let inst = Instruction::new("or");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b01000110]);
        assert_eq!(inst.size, 1);

        let inst = Instruction::new("eor");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b01010110]);
        assert_eq!(inst.size, 1);

        let inst = Instruction::new("shift_left");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b01100110]);
        assert_eq!(inst.size, 1);

        let inst = Instruction::new("shift_right");
        assert!(inst.is_ok());
        let inst = inst.ok().unwrap().unwrap();
        assert_eq!(inst.to_bytes(), vec![0b01110110]);
        assert_eq!(inst.size, 1);
    }
}
