use std::borrow::Borrow;
use std::fmt::Display;
use std::io::Read;
use std::{fs, io};

fn read_bytes(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = fs::File::open(filename)?;
    let metadata = fs::metadata(filename)?;
    let mut buffer = vec![0; metadata.len() as usize];
    file.read_exact(&mut buffer)?;

    Ok(buffer)
}

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
        match *self {
            AL => write!(f, "al"),
            CL => write!(f, "cl"),
            DL => write!(f, "dl"),
            BL => write!(f, "bl"),
            AH => write!(f, "ah"),
            CH => write!(f, "ch"),
            DH => write!(f, "dh"),
            BH => write!(f, "bh"),
        }
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
        match *self {
            AX => write!(f, "ax"),
            CX => write!(f, "cx"),
            DX => write!(f, "dx"),
            BX => write!(f, "bx"),
            SP => write!(f, "sp"),
            BP => write!(f, "bp"),
            SI => write!(f, "si"),
            DI => write!(f, "di"),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
enum Pointer {
    BX_SI(i16),
    BX_DI(i16),
    BP_SI(i16),
    BP_DI(i16),
    SI(i16),
    DI(i16),
    Direct(i16),
    BP(i16),
    BX(i16),
    Unread,
}

impl Pointer {
    fn from_r(r: u8, value: i16) -> Self {
        match r {
            0 => Pointer::BX_SI(value),
            1 => Pointer::BX_DI(value),
            2 => Pointer::BP_SI(value),
            3 => Pointer::BP_DI(value),
            4 => Pointer::SI(value),
            5 => Pointer::DI(value),
            6 => Pointer::BP(value),
            7 => Pointer::BX(value),
            _ => Pointer::Unread,
        }
    }
}

impl Display for Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Pointer::*;
        match *self {
            BX_SI(disp) => write!(f, "[bx + si + {}]", disp),
            BX_DI(disp) => write!(f, "[bx + di + {}]", disp),
            BP_SI(disp) => write!(f, "[bp + si + {}]", disp),
            BP_DI(disp) => write!(f, "[bp + di + {}]", disp),
            SI(disp) => write!(f, "[sp + {}]", disp),
            DI(disp) => write!(f, "[di + {}]", disp),
            Direct(disp) => write!(f, "[{}]", disp),
            BP(disp) => write!(f, "[bp + {}]", disp),
            BX(disp) => write!(f, "[bx + {}]", disp),
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
    DcUnread,
    DwUnread,
    SR, // segment register
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
    Unknown,
}

fn opcode_to_instruction(opcode_byte: u8) -> Asm8086 {
    use ByteRegister::*;
    use Operand::*;
    use WordRegister::*;
    match opcode_byte {
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
        0o261 => Asm8086::Mov(Rb(Address::ByteRegister(CL)), DcUnread),
        0o265 => Asm8086::Mov(Rb(Address::ByteRegister(CH)), DcUnread),
        0o271 => Asm8086::Mov(Rw(Address::WordRegister(CX)), DwUnread),
        0o272 => Asm8086::Mov(Rw(Address::WordRegister(DX)), DwUnread),
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

fn resolve_mov_operands(byte: u8) -> (Mod, u8, u8) {
    let x = (byte & 0b11000000) >> 6;
    let r_or_s = (byte & 0b00111000) >> 3;
    let m = byte & 0b00000111;
    let mode = match x {
        0 => Mod::MemoryNoDisp,
        1 => Mod::Memory8BitDisp,
        2 => Mod::Memory16BitDisp,
        _ => Mod::Register,
    };
    (mode, r_or_s, m)
}

fn resolve_address(operand: Operand, mode: Mod, r_or_s: u8, m: u8, disp: Option<i16>) -> Address {
    use Operand::*;
    match (operand, mode, disp) {
        (Rb(_), _, _) => Address::ByteRegister(ByteRegister::from_r(r_or_s)),
        (Rw(_), _, _) => Address::WordRegister(WordRegister::from_r(r_or_s)),
        (Eb(_), Mod::MemoryNoDisp, None) => Address::Pointer(Pointer::from_r(m, 0)),
        (Ew(_), Mod::MemoryNoDisp, None) => Address::Pointer(Pointer::from_r(m, 0)),
        (Eb(_), Mod::Memory8BitDisp | Mod::Memory16BitDisp, Some(disp)) => {
            Address::Pointer(Pointer::from_r(m, disp))
        }
        (Ew(_), Mod::Memory8BitDisp | Mod::Memory16BitDisp, Some(disp)) => {
            Address::Pointer(Pointer::from_r(m, disp))
        }
        (Eb(_), Mod::Register, _) => Address::ByteRegister(ByteRegister::from_r(m)),
        (Ew(_), Mod::Register, _) => Address::WordRegister(WordRegister::from_r(m)),
        _ => Address::Unread,
    }
}

fn to_disp(low_byte: u8, high_byte: u8) -> i16 {
    ((high_byte as i16) << 8) | (low_byte as i16)
}

fn read_bytes_cli() -> Result<Vec<u8>, String> {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => Err("Have not found binary to decompile".into()),
        _ => {
            let filename = args[1].borrow();
            read_bytes(filename).map_err(|_| format!("Unable to read file '{}'", filename))
        }
    }
}

fn next_byte(bytes: &mut Vec<u8>) -> Result<u8, String> {
    bytes.pop().ok_or("could not parse byte".into())
}

fn main() -> Result<(), String> {
    let mut bytes = read_bytes_cli()?;
    bytes.as_mut_slice().reverse();
    while let Ok(first_byte) = next_byte(&mut bytes) {
        let opcode = opcode_to_instruction(first_byte);
        match opcode {
            Asm8086::Mov(reg, Operand::DcUnread) => {
                let value_byte = next_byte(&mut bytes)?;
                let value = value_byte as i8;
                println!("[{first_byte:#o}][{value_byte:#o}]\nmov {reg}, {value}")
            }
            Asm8086::Mov(reg, Operand::DwUnread) => {
                let low_byte = next_byte(&mut bytes)?;
                let hihh_byte = next_byte(&mut bytes)?;
                let value = to_disp(low_byte, hihh_byte);
                println!("[{first_byte:#o}][{first_byte:#o}][{first_byte:#o}]\nmov {reg}, {value}")
            }
            Asm8086::Mov(dest, src) => {
                let second_byte = next_byte(&mut bytes)?;
                let (mode, r_or_s, m) = resolve_mov_operands(second_byte);
                let disp = match (mode, r_or_s) {
                    (Mod::MemoryNoDisp, 6) => {
                        println!("[{first_byte:#o}][{second_byte:#o}]");
                        Some(second_byte as i16)
                    }
                    (Mod::Memory8BitDisp, _) => {
                        let low_byte = next_byte(&mut bytes)?;
                        println!("[{first_byte:#o}][{second_byte:#o}][{low_byte:#o}]");
                        Some(low_byte as i16)
                    }
                    (Mod::Memory16BitDisp, _) => {
                        let low_byte = next_byte(&mut bytes)?;
                        let high_byte = next_byte(&mut bytes)?;
                        println!(
                            "[{first_byte:#o}][{second_byte:#o}][{low_byte:#o}][{high_byte:#o}]"
                        );
                        Some(to_disp(low_byte, high_byte))
                    }
                    _ => {
                        println!("[{first_byte:#o}][{second_byte:#o}]");
                        None
                    }
                };
                let src = resolve_address(src, mode, r_or_s, m, disp);
                let dest = resolve_address(dest, mode, r_or_s, m, disp);
                println!("mov {dest}, {src}");
            }
            Asm8086::Unknown => println!("unable to parse opcode bit {first_byte:#o}"),
        }
    }
    Ok(())
}
