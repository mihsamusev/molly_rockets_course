use crate::register::Reg;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    MovImmToReg(Reg, i16),
    MovRegToReg(Reg, Reg),
    MovMemToReg(Reg, MemAddress),
    MovRegToMem(MemAddress, Reg),
    AddImmToReg(Reg, i16),
    AddRegToReg(Reg, Reg),
    SubImmToReg(Reg, i16),
    SubRegToReg(Reg, Reg),
    CmpImmToReg(Reg, i16),
    CmpRegToReg(Reg, Reg),
    Jnz(i16),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MemAddress {
    Offset(i16),
    RegAndOffset(Reg, i16),
}

impl FromStr for MemAddress {
    type Err = String;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mem_address = text
            .trim()
            .strip_prefix("word [")
            .and_then(|s| s.strip_suffix("]"))
            .ok_or_else(|| "Unable to find '[]'")?;

        if let Ok(value) = mem_address.parse::<i16>() {
            return Ok(MemAddress::Offset(value));
        }

        let (left, right) = mem_address
            .split_once(" ")
            .ok_or_else(|| "unable to parse memory")?;
        let reg = left.parse::<Reg>()?;

        let offset_str = right
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        let offset = offset_str.parse::<i16>().map_err(|e| e.to_string())?;
        return Ok(MemAddress::RegAndOffset(reg, offset));
    }
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let (instruction, addresses) = text
            .split_once(" ")
            .ok_or_else(|| "No instruction to split by ' ' ")?;
        let (dest, src) = addresses
            .split_once(",")
            .ok_or_else(|| "Cant split source and destination")?;

        match (instruction, dest, src) {
            ("mov", first, second) => {
                println!("{}, {}", first, second);
                if let Ok(dest) = first.parse::<Reg>() {
                    if let Ok(value) = second.trim().parse::<i16>() {
                        Ok(Instruction::MovImmToReg(dest, value))
                    } else if let Ok(register) = second.parse::<Reg>() {
                        Ok(Instruction::MovRegToReg(dest, register))
                    } else if let Ok(mem_address) = second.parse::<MemAddress>() {
                        Ok(Instruction::MovMemToReg(dest, mem_address))
                    } else {
                        Err(format!("cant parse src {}", src))
                    }
                } else {
                    let mem_address = dest.parse::<MemAddress>()?;
                    if let Ok(register) = second.parse::<Reg>() {
                        Ok(Instruction::MovRegToMem(mem_address, register))
                    } else {
                        Err(format!("cant parse memory src {}", src))
                    }
                }
            }
            ("add", reg, value) => {
                let dest = reg.parse()?;
                if let Ok(value) = value.trim().parse::<i16>() {
                    Ok(Instruction::AddImmToReg(dest, value))
                } else if let Ok(register) = value.parse::<Reg>() {
                    Ok(Instruction::AddRegToReg(dest, register))
                } else {
                    Err(String::from("cant parse src"))
                }
            }
            ("sub", reg, value) => {
                let dest = reg.parse()?;
                if let Ok(value) = value.trim().parse::<i16>() {
                    Ok(Instruction::SubImmToReg(dest, value))
                } else if let Ok(register) = value.parse::<Reg>() {
                    Ok(Instruction::SubRegToReg(dest, register))
                } else {
                    Err(String::from("cant parse src"))
                }
            }
            ("cmp", reg, value) => {
                let dest = reg.parse()?;
                if let Ok(value) = value.trim().parse::<i16>() {
                    Ok(Instruction::CmpImmToReg(dest, value))
                } else if let Ok(register) = value.parse::<Reg>() {
                    Ok(Instruction::CmpRegToReg(dest, register))
                } else {
                    Err(String::from("cant parse src"))
                }
            }
            ("jnz", value, _) => {
                if let Ok(value) = value.trim().parse::<i16>() {
                    Ok(Instruction::Jnz(value))
                } else {
                    Err(String::from("cant parse jump ip"))
                }
            }
            _ => Err(String::from("cant parse instruction")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_mov_immediate_to_register() {
        assert_eq!(
            "mov bx, 2".parse(),
            Ok(Instruction::MovImmToReg(Reg::BX, 2))
        );
    }
    #[test]
    fn can_parse_mov_register_to_register() {
        assert_eq!(
            "mov bx, cx".parse(),
            Ok(Instruction::MovRegToReg(Reg::BX, Reg::CX))
        );
    }

    #[test]
    fn can_parse_jnz() {
        assert_eq!("jnz -3".parse(), Ok(Instruction::Jnz(-3)))
    }

    #[test]
    fn can_parse_memory_offset() {
        assert_eq!(
            " word [1000]".parse::<MemAddress>(),
            Ok(MemAddress::Offset(1000))
        )
    }

    #[test]
    fn can_parse_memory_register_and_offset() {
        assert_eq!(
            " word [bp + 1000]".parse::<MemAddress>(),
            Ok(MemAddress::RegAndOffset(Reg::BP, 1000))
        )
    }

    #[test]
    fn can_parse_instruction_with_memory_offset() {
        assert_eq!(
            "mov bx, word [1000]".parse(),
            Ok(Instruction::MovMemToReg(Reg::BX, MemAddress::Offset(1000)))
        )
    }
}
