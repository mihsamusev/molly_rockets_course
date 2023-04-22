use std::fmt::Display;
use std::{io, fs};
use std::io::Read;

fn read_bytes(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = fs::File::open(filename)?;
    let metadata = fs::metadata(filename)?;
    let mut buffer= vec![0; metadata.len() as usize];
    file.read_exact(&mut buffer)?;

    Ok(buffer)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum HalfRegister {
    AL,
    CL,
    DL,
    BL,
    AH,
    CH,
    DH,
    BH,
    Unresolved
}

use HalfRegister::*;
impl HalfRegister {
    const VALUES: [Self; 8] = [AL, CL, DL, BL, AH, CH, DH, BH];
    fn from_r(r: u8) -> Self {
        HalfRegister::VALUES[r as usize]
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
enum FullRegister {
    AX,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
    Unresolved
}

use FullRegister::*;
impl FullRegister {
    const VALUES: [Self; 8] = [AX, CX, DX, BX, SP, BP, SI, DI];
    fn from_r(r: u8) -> Self {
        FullRegister::VALUES[r as usize]
    }
}



#[derive(Debug, Clone, Copy, PartialEq)]
enum Operand {
    Rb(HalfRegister),
    Rw(FullRegister),
    Eb(HalfRegister), // effective address byte
    Ew(FullRegister), // effective word
    SR, // segment register
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Operand::*;
        match self {
            Rb(reg) if (*reg) != HalfRegister::Unresolved => write!(f, "{reg:?}"),
            Eb(reg) if (*reg) != HalfRegister::Unresolved => write!(f, "{reg:?}"),
            Rw(reg) if (*reg) != FullRegister:: Unresolved => write!(f, "{reg:?}"),
            Ew(reg) if (*reg) != FullRegister:: Unresolved => write!(f, "{reg:?}"),
            _ => write!(f, "{self:?}")
        }
    }
}


#[derive(Debug)]
enum Asm8086 {
    Mov(Operand, Operand),
    Unknown
}

fn opcode_to_instruction(opcode_byte: u8) -> Asm8086 {
    use Operand::*;
    match opcode_byte {
        0o210 => Asm8086::Mov(Eb(HalfRegister::Unresolved), Rb(HalfRegister::Unresolved)),
        0o211 => Asm8086::Mov(Ew(FullRegister::Unresolved), Rw(FullRegister::Unresolved)),
        0o212 => Asm8086::Mov(Rb(HalfRegister::Unresolved), Eb(HalfRegister::Unresolved)),
        0o213 => Asm8086::Mov(Rw(FullRegister::Unresolved), Ew(FullRegister::Unresolved)),
        0o214 => Asm8086::Mov(Ew(FullRegister::Unresolved), SR),
        0o216 => Asm8086::Mov(SR, Ew(FullRegister::Unresolved)),
        _  => Asm8086::Unknown
    }
}

fn resolve_mov_operands(byte: u8) -> (u8, u8, u8) {
   let x = (byte & 0b11000000) >> 6; 
   let r_or_s = (byte & 0b00111000) >> 3;
   let m = byte & 0b00000111; 
   (x, r_or_s, m)
}

fn resolve_register_mode(register_operand: Operand, r: u8) -> Operand {
    match (register_operand, r) {
        (Operand::Rb(HalfRegister::Unresolved), 0..=7) => Operand::Rb(HalfRegister::from_r(r)),
        (Operand::Eb(HalfRegister::Unresolved), 0..=7) => Operand::Eb(HalfRegister::from_r(r)),
        (Operand::Rw(FullRegister::Unresolved), 0..=7) => Operand::Rw(FullRegister::from_r(r)),
        (Operand::Ew(FullRegister::Unresolved), 0..=7) => Operand::Ew(FullRegister::from_r(r)),
        _ => register_operand
    }
}


fn resolve_operands_address(operand: Operand, x: u8, r_or_s: u8, m: u8) -> Operand {
    use Operand::*;
    match operand {
        Rb(_) | Rw(_) => resolve_register_mode(operand, r_or_s),
        Eb(_) | Ew(_) => resolve_register_mode(operand, m),
        _ => operand
    }
}
fn main() -> io::Result<()>{
     //let mut instructions = Vec::new();
    let mut bytes = read_bytes("multiple_mov.bin")?;
    bytes.as_mut_slice().reverse();
    while let Some(first_bit) =  bytes.pop() {
        match opcode_to_instruction(first_bit) {
            Asm8086::Mov(dest, src) => { 
                print!("{first_bit:#o} is MOV {dest:?}, {src:?}\t");
                if let Some(second_bit) = bytes.pop() {
                    let (x, r_or_s, m)= resolve_mov_operands(second_bit);
                    let src = resolve_operands_address(src, x, r_or_s, m);
                    let dest = resolve_operands_address(dest, x, r_or_s, m);
                    println!("MOV {dest} {src}")
                } else {
                    panic!("Could not finish parsing...")
                }
            },
            Asm8086::Unknown => println!("unable to parse opcode bit {first_bit:#o}")
        }
    } 
    Ok(())
}

