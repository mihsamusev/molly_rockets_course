use std::io::{Read, self};
use std::fs::{File, self};
use itertools::Itertools;

const MOV_OPCODE: u8 = 0b100010;
#[derive(Debug, PartialEq)]
enum Register {
    AX,
    AL,
    AH,
    BX,
    BL,
    BH,
    CX,
    CL,
    CH,
    DX,
    DH,
    DL,
    SP, // stack pointer
    BP, // base pointer
    SI, // source index
    DI, // dest index
}

impl Register {
    fn calculate(reg: u8, w: bool) -> Self {
        match (reg, w) {
            (0o0, true) => Self::AX,
            (0o1, true) => Self::CX,
            (0o2, true) => Self::DX,
            (0o3, true) => Self::BX,
            (0o4, true) => Self::SP,
            (0o5, true) => Self::BP,
            (0o6, true) => Self::SI,
            (0o7, true) => Self::DI,
            (0o0, false) => Self::AL,
            (0o1, false) => Self::CL,
            (0o2, false) => Self::DL,
            (0o3, false) => Self::BL,
            (0o4, false) => Self::AH,
            (0o5, false) => Self::CH,
            (0o6, false) => Self::DH,
            (0o7, false) => Self::BH,
            _ => panic!(),
        }
    }
}
#[derive(Debug, PartialEq)]
enum Asm8086 {
    Mov(Register, Register),
    Nop,
}

fn decode_first_byte(byte: u8) -> (u8, bool, bool) {
    let opcode = byte >> 2;
    let is_dest = (byte & 2) == 2; // figure out how to output deez
    let w = (byte & 1) == 1; // figure out how to output deez
    (opcode, is_dest, w)
}

fn decode_second_byte(byte: u8) -> (u8, u8, u8) {
    let mode = byte >> 6 & 0x000f;
    let reg = byte >> 3 & 0b00000111;
    let rm = byte & 0b00000111;
    (mode, reg, rm)
}
fn decode_word(bytes: u16) -> (u8, bool, bool, u8, u8, u8) {
    let first = (bytes >> 8 & 0x00ff) as u8;
    let second = (bytes & 0x00ff) as u8;

    let (opcode, is_dest, w) = decode_first_byte(first);
    let (mode, reg, rm) = decode_second_byte(second);
    (opcode, is_dest, w, mode, reg, rm)
}

fn decompile_word(bytes: u16) -> Asm8086 {
    let (opcode, is_dest, w, mode, reg, rm) = decode_word(bytes);
    match opcode {
        MOV_OPCODE => {
            let mut source = Register::calculate(reg, w);
            let mut dest = Register::calculate(rm, w);
            if is_dest {
                (source, dest) = (dest, source);
            }
            Asm8086::Mov(dest, source)
        }
        _ => Asm8086::Nop,
    }
}

fn read_bytes(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let metadata = fs::metadata(filename)?;
    let mut buffer= vec![0; metadata.len() as usize];
    file.read_exact(&mut buffer)?;

    Ok(buffer)
}

struct PairU8(u8, u8);

impl From<PairU8> for u16 {
    fn from(value: PairU8) -> Self {
        ((value.0 as u16) << 8) | (value.1 as u16)
    }
}
fn main() -> io::Result<()>{
    let bytes = read_bytes("multiple_mov.bin")?;
    for (left, right) in bytes.into_iter().tuples() {
        let word: u16 = PairU8(left, right).into();
        let instruction = decompile_word(word);
        println!("{:?}", instruction);
    }

    Ok(())
}
