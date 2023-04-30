use std::fmt::{Display};
use asm8086_octal::bytes_io;


#[derive(Debug, Clone, Copy, PartialEq)]
enum ByteRegister {
    AL,
    CL,
    DL,
    BL,
    AH,
    CH,
    DH,
    BH,
}

use ByteRegister::*;
impl ByteRegister {
    const VALUES: [Self; 8] = [AL, CL, DL, BL, AH, CH, DH, BH];
    fn from_r(r: u8) -> Self {
        ByteRegister::VALUES[r as usize]
    }
}

impl Display for ByteRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match *self {
            AL => "al",
            CL => "cl",
            DL => "dl",
            BL => "bl",
            AH => "ah",
            CH => "ch",
            DH => "dh",
            BH => "bh",
        };
        write!(f, "{}", text)
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
enum WordRegister {
    AX,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
}

use WordRegister::*;
impl WordRegister {
    const VALUES: [Self; 8] = [AX, CX, DX, BX, SP, BP, SI, DI];
    fn from_r(r: u8) -> Self {
        WordRegister::VALUES[r as usize]
    }
}

impl Display for WordRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match *self {
            AX => "ax",
            CX => "cx",
            DX => "dx",
            BX => "bx",
            SP => "sp",
            BP => "bp",
            SI => "si",
            DI => "di",
        };
        write!(f, "{}", text)
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
enum Pointer {
    BX_SI(Disp),
    BX_DI(Disp),
    BP_SI(Disp),
    BP_DI(Disp),
    SI(Disp),
    DI(Disp),
    Direct(Disp),
    BP(Disp),
    BX(Disp),
    Unread,
}

impl Pointer {
    fn with_disp(r: u8, disp: Disp) -> Self {
        match r {
            0 => Pointer::BX_SI(disp),
            1 => Pointer::BX_DI(disp),
            2 => Pointer::BP_SI(disp),
            3 => Pointer::BP_DI(disp),
            4 => Pointer::SI(disp),
            5 => Pointer::DI(disp),
            6 => Pointer::BP(disp),
            7 => Pointer::BX(disp),
            _ => Pointer::Unread,
        }
    }

    fn direct(disp: Disp) -> Self {
        Pointer::Direct(disp)
    }
}

impl Display for Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Pointer::*;
        match *self {
            BX_SI(disp) => write!(f, "[bx + si{}]", disp),
            BX_DI(disp) => write!(f, "[bx + di{}]", disp),
            BP_SI(disp) => write!(f, "[bp + si{}]", disp),
            BP_DI(disp) => write!(f, "[bp + di{}]", disp),
            SI(disp) => write!(f, "[sp{}]", disp),
            DI(disp) => write!(f, "[di{}]", disp),
            Direct(disp) => write!(f, "[{}]", disp),
            BP(disp) => write!(f, "[bp{}]", disp),
            BX(disp) => write!(f, "[bx{}]", disp),
            Unread => write!(f, "Unread"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Address {
    ByteRegister(ByteRegister),
    WordRegister(WordRegister),
    Pointer(Pointer),
    ByteRegisterUnread,
    WordRegisterUnread,
    PointerUnread,
    Unread,
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Address::*;
        match *self {
            ByteRegister(register) => write!(f, "{}", register),
            WordRegister(register) => write!(f, "{}", register),
            Pointer(pointer) => write!(f, "{}", pointer),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operand {
    Rb(Address),
    Rw(Address),
    Eb(Address), // effective address byte
    Ew(Address), // effective word
    D(Disp),
    SR, // segment register
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Disp {
    None,
    D8(i8),
    D16(i16),
    D8Unread,
    D16Unread
}

impl Display for Disp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Disp::*;
        match *self {
            None | D8(0) | D16(0) => write!(f, ""),
            D8(x) if x < 0 => write!(f, " - {}", -x),
            D16(x) if x < 0 => write!(f, " - {}", -x),
            D8(x) => write!(f, " + {}", x),
            D16(x) => write!(f, " + {}", x),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Operand::*;
        match *self {
            Rb(inner) => write!(f, "{}", inner),
            Rw(inner) => write!(f, "{}", inner),
            Eb(inner) => write!(f, "{}", inner),
            Ew(inner) => write!(f, "{}", inner),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[derive(Debug)]
enum Asm8086 {
    Mov(Operand, Operand),
    Add(Operand, Operand),
    Sub(Operand, Operand),
    Cmp(Operand, Operand),
    Unknown,
}


enum Type {
    BetweenRegisters
}

fn opcode_to_instruction(opcode_byte: u8) -> Asm8086 {
    use ByteRegister::*;
    use Operand::*;
    use WordRegister::*;
    match opcode_byte {
        0o000 => Asm8086::Add(Eb(Address::ByteRegisterUnread), Rb(Address::ByteRegisterUnread)),
        0o001 => Asm8086::Add(Ew(Address::WordRegisterUnread), Rw(Address::WordRegisterUnread)),
        0o002 => Asm8086::Add(Rb(Address::ByteRegisterUnread), Eb(Address::ByteRegisterUnread)),
        0o003 => Asm8086::Add(Rw(Address::WordRegisterUnread), Ew(Address::WordRegisterUnread)),
        0o210 => Asm8086::Mov(
            Eb(Address::ByteRegisterUnread),
            Rb(Address::ByteRegisterUnread),
        ),
        0o211 => Asm8086::Mov(
            Ew(Address::WordRegisterUnread),
            Rw(Address::WordRegisterUnread),
        ),
        0o212 => Asm8086::Mov(
            Rb(Address::ByteRegisterUnread),
            Eb(Address::ByteRegisterUnread),
        ),
        0o213 => Asm8086::Mov(
            Rw(Address::WordRegisterUnread),
            Ew(Address::WordRegisterUnread),
        ),
        0o214 => Asm8086::Mov(Ew(Address::WordRegisterUnread), SR),
        0o216 => Asm8086::Mov(SR, Ew(Address::WordRegisterUnread)),
        // direct from / to accumulator
        0o240 => Asm8086::Mov(Eb(Address::ByteRegister(AL)), D(Disp::D16Unread)),
        0o241 => Asm8086::Mov(Ew(Address::WordRegister(AX)), D(Disp::D16Unread)),
        0o242 => Asm8086::Mov(D(Disp::D16Unread), Eb(Address::ByteRegister(AL))),
        0o243 => Asm8086::Mov(D(Disp::D16Unread), Eb(Address::WordRegister(AX))),
        // Direct to byte register 0o26r-Db
        0o261 => Asm8086::Mov(Rb(Address::ByteRegister(CL)), D(Disp::D8Unread)),
        0o265 => Asm8086::Mov(Rb(Address::ByteRegister(CH)), D(Disp::D8Unread)),
        // Direct to word register 0o27r-Dw
        0o271 => Asm8086::Mov(Rw(Address::WordRegister(CX)), D(Disp::D16Unread)),
        0o272 => Asm8086::Mov(Rw(Address::WordRegister(DX)), D(Disp::D16Unread)),
        _ => Asm8086::Unknown,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mod {
    MemoryNoDisp,
    Memory8BitDisp,
    Memory16BitDisp,
    Register,
}

fn byte_octals(byte: u8) -> (u8, u8, u8) {
    let first = (byte & 0b11000000) >> 6;
    let second = (byte & 0b00111000) >> 3;
    let third = byte & 0b00000111;
    (first, second, third)
}

fn resolve_mov_operands(byte: u8) -> (Mod, u8, u8) {
    let (x, r_or_s, m) = byte_octals(byte);
    let mode = match x {
        0 => Mod::MemoryNoDisp,
        1 => Mod::Memory8BitDisp,
        2 => Mod::Memory16BitDisp,
        _ => Mod::Register,
    };
    (mode, r_or_s, m)
}

fn resolve_address(operand: Operand, mode: Mod, r_or_s: u8, m: u8, disp: Disp) -> Address {
    use Operand::*;
    match (operand, mode, m) {
        (Ew(_), Mod::MemoryNoDisp, 6) => Address::Pointer(Pointer::direct(disp)),
        (Rb(_), _, _) => Address::ByteRegister(ByteRegister::from_r(r_or_s)),
        (Rw(_), _, _) => Address::WordRegister(WordRegister::from_r(r_or_s)),
        (Eb(_), Mod::MemoryNoDisp, _) => Address::Pointer(Pointer::with_disp(m, disp)),
        (Ew(_), Mod::MemoryNoDisp, _) => Address::Pointer(Pointer::with_disp(m, disp)),
        (Eb(_), Mod::Memory8BitDisp | Mod::Memory16BitDisp, _) => {
            Address::Pointer(Pointer::with_disp(m, disp))
        }
        (Ew(_), Mod::Memory8BitDisp | Mod::Memory16BitDisp, _) => {
            Address::Pointer(Pointer::with_disp(m, disp))
        }
        (Eb(_), Mod::Register, _) => Address::ByteRegister(ByteRegister::from_r(m)),
        (Ew(_), Mod::Register, _) => Address::WordRegister(WordRegister::from_r(m)),
        _ => Address::Unread,
    }
}

fn to_word(low_byte: u8, high_byte: u8) -> i16 {
    ((high_byte as i16) << 8) | (low_byte as i16)
}


fn next_byte_disp(bytes: &[u8], end_ptr: usize) -> Result<Disp, String> {
    let low_byte = *bytes.get(end_ptr).ok_or("could not parse byte")?;
    Ok(Disp::D8(low_byte as i8))
}

fn next_word_disp(bytes: &[u8], end_ptr: usize) -> Result<Disp, String> {
    let low_byte = *bytes.get(end_ptr).ok_or("could not parse byte")?;
    let high_byte = *bytes.get(end_ptr + 1).ok_or("could not parse byte")?;
    let disp_word = to_word(low_byte, high_byte);
    Ok(Disp::D16(disp_word))
}


fn parse_bytes(bytes: &[u8]) -> Result<(), String> {
    let mut start_ptr = 0;
    let mut end_ptr = 0;
        while start_ptr != bytes.len() {
        let first_byte = bytes[end_ptr];
        end_ptr += 1;
        let opcode = opcode_to_instruction(first_byte);
        match opcode {
            Asm8086::Mov(reg, Operand::D(Disp::D8Unread)) => {
                let disp = next_byte_disp(bytes, end_ptr)?;
                end_ptr += 1;
                println!("mov {reg}, [{disp}]")
            }
            Asm8086::Mov(reg, Operand::D(Disp::D16Unread)) => {
                let disp = next_word_disp(bytes, end_ptr)?;
                end_ptr += 2;
                println!("mov {reg}, [{disp}]")
            },
            Asm8086::Mov(Operand::D(Disp::D16Unread), reg) => {
                let disp = next_word_disp(bytes, end_ptr)?;
                end_ptr += 2;
                println!("mov [{disp}], {reg}")
            }
            Asm8086::Mov(dest, src) => {
                let second_byte = bytes[end_ptr];
                end_ptr += 1;
                let (mode, r_or_s, m) = resolve_mov_operands(second_byte);
                let disp = match (mode, m) {
                    (Mod::MemoryNoDisp, 6) | (Mod::Memory16BitDisp, _) => {
                        let disp = next_word_disp(bytes, end_ptr)?;
                        end_ptr += 2;
                        disp
                    }
                    (Mod::Memory8BitDisp, _) => {
                        let disp = next_byte_disp(bytes, end_ptr)?;
                        end_ptr += 1;
                        disp
                    }
                    _ => {
                        Disp::None
                    }
                };
                let src = resolve_address(src, mode, r_or_s, m, disp);
                let dest = resolve_address(dest, mode, r_or_s, m, disp);
                println!("mov {dest}, {src}");
            }
            _ => println!("unable to parse opcode bit {first_byte:#o}"),
        }
        let parsed_bytes = bytes_io::format_bytes(&bytes, start_ptr, end_ptr);
        println!("bytes {}..{} = {}", start_ptr, end_ptr, parsed_bytes);
        start_ptr = end_ptr;
    }
    Ok(())
}
    
fn main() -> Result<(), String> {
    let bytes = bytes_io::read_bytes_cli()?;
    //let bytes = vec![0o213, 0o56, 0o5, 0o0]; // mov bp, 5
    //let bytes = vec![0o241, 0o373, 0o11]; // mov ax, [2555]
    parse_bytes(&bytes)?;
    Ok(())
}
