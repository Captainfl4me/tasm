mod generic;
mod instruction;
mod flag;
mod label;

pub use instruction::*;
pub use label::*;
pub use flag::*;

use generic::trim_line;

pub enum TokenType {
    Instruction(Instruction),
    Flag(Flag),
    Label(Label),
}

pub fn lex_line(line: &str) -> Result<Option<TokenType>, String> {
    let line = trim_line(line);
    if line.is_empty() {
        return Ok(None);
    }

    let flag_opt = Flag::new(line)?;
    if let Some(flag) = flag_opt {
        return Ok(Some(TokenType::Flag(flag)));
    }

    let instruction_opt = Instruction::new(line)?;
    if let Some(instruction) = instruction_opt {
        return Ok(Some(TokenType::Instruction(instruction)));
    }
    
    let label_opt = Label::new(line)?;
    if let Some(label) = label_opt {
        return Ok(Some(TokenType::Label(label)))
    }

    Err("Line cannot be lex".to_string())
}
