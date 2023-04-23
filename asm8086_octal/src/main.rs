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
        0o240 => Asm8086::Mov(Eb(Address::ByteRegister(AL)), D(Disp::D16Unread)),
        0o241 => Asm8086::Mov(Ew(Address::WordRegister(AX)), D(Disp::D16Unread)),
        0o242 => Asm8086::Mov(D(Disp::D16Unread), Eb(Address::ByteRegister(AL))),
        0o243 => Asm8086::Mov(D(Disp::D16Unread), Eb(Address::WordRegister(AX))),
        0o261 => Asm8086::Mov(Rb(Address::ByteRegister(CL)), D(Disp::D8Unread)),
        0o265 => Asm8086::Mov(Rb(Address::ByteRegister(CH)), D(Disp::D8Unread)),
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

fn next_byte_disp(bytes: &mut Vec<u8>) -> Result<Disp, String> {
    let low_byte = next_byte(bytes)?;
    Ok(Disp::D8(low_byte as i8))
}

fn next_word_disp(bytes: &mut Vec<u8>) -> Result<Disp, String> {
    let low_byte = next_byte(bytes)?;
    let high_byte = next_byte(bytes)?;
    let disp_word = to_word(low_byte, high_byte);
    Ok(Disp::D16(disp_word))
}

fn parse_bytes(mut bytes: Vec<u8>) -> Result<(), String> {
    bytes.as_mut_slice().reverse();
    while let Ok(first_byte) = next_byte(&mut bytes) {
        let opcode = opcode_to_instruction(first_byte);
        match opcode {
            Asm8086::Mov(reg, Operand::D(Disp::D8Unread)) => {
                let disp_byte = next_byte(&mut bytes)?;
                let disp = Disp::D8(disp_byte as i8);
                println!("[{first_byte:#o}][{disp_byte:#o}]\nmov {reg}, {disp}")
            }
            Asm8086::Mov(reg, Operand::D(Disp::D16Unread)) => {
                let low_byte = next_byte(&mut bytes)?;
                let high_byte = next_byte(&mut bytes)?;
                let disp_word = to_word(low_byte, high_byte);
                let disp = Disp::D16(disp_word);
                println!("[{first_byte:#o}][{first_byte:#o}][{first_byte:#o}]\nmov {reg}, {disp}")
            },
            Asm8086::Mov(Operand::D(Disp::D16Unread), reg) => {
                let low_byte = next_byte(&mut bytes)?;
                let high_byte = next_byte(&mut bytes)?;
                let disp_word = to_word(low_byte, high_byte);
                let disp = Disp::D16(disp_word);
                println!("[{first_byte:#o}][{first_byte:#o}][{first_byte:#o}]\nmov {disp}, {reg}")
            }
            Asm8086::Mov(dest, src) => {
                let second_byte = next_byte(&mut bytes)?;
                let (mode, r_or_s, m) = resolve_mov_operands(second_byte);
                //println!("{mode:?} {r_or_s} {m}");
                let disp = match (mode, m) {
                    (Mod::MemoryNoDisp, 6) => {
                        let low_byte = next_byte(&mut bytes)?;
                        let high_byte = next_byte(&mut bytes)?;
                        let disp_word = to_word(low_byte, high_byte);
                        println!("[{first_byte:#o}][{second_byte:#o}][{low_byte:#o}][{high_byte:#o}]");
                        Disp::D16(disp_word)
                    }
                    (Mod::Memory8BitDisp, _) => {
                        let low_byte = next_byte(&mut bytes)?;
                        println!("[{first_byte:#o}][{second_byte:#o}][{low_byte:#o}]");
                        Disp::D8(low_byte as i8)
                    }
                    (Mod::Memory16BitDisp, _) => {
                        let low_byte = next_byte(&mut bytes)?;
                        let high_byte = next_byte(&mut bytes)?;
                        println!(
                            "[{first_byte:#o}][{second_byte:#o}][{low_byte:#o}][{high_byte:#o}]"
                        );
                        Disp::D16(to_word(low_byte, high_byte))
                    }
                    _ => {
                        println!("[{first_byte:#o}][{second_byte:#o}]");
                        Disp::None
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
    
fn main() -> Result<(), String> {
    let bytes = read_bytes_cli()?;
    //let bytes = vec![0o213, 0o56, 0o5, 0o0]; // mov bp, 5
    //let bytes = vec![0o241, 0o373, 0o11]; // mov ax, [2555]
    parse_bytes(bytes)?;
    Ok(())
}
// │ bits 16
// 18   │ 
// 19   │ ; Signed displacements
// 20   │ mov ax, [bx + di - 37]
// 21   │ mov [si - 300], cx
// 22   │ mov dx, [bx - 32]
// 23   │ 
// 24   │ ; Explicit sizes
// 25   │ mov [bp + di], byte 7
// 26   │ mov [di + 901], word 347
// 27   │ 
// 28   │ ; Direct address
// 29   │ mov bp, [5]
// 30   │ mov bx, [3458]
// 31   │ 
// 32   │ ; Memory-to-accumulator test
// 33   │ mov ax, [2555]
// 34   │ mov ax, [16]
// 35   │ 
// 36   │ ; Accumulator-to-memory test
// 37   │ mov [2554], ax
// 38   │ mov [15], ax
// ───────┴───────────────