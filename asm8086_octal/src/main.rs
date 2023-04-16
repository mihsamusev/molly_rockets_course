use std::{io, fs};
use std::io::Read;

fn read_bytes(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = fs::File::open(filename)?;
    let metadata = fs::metadata(filename)?;
    let mut buffer= vec![0; metadata.len() as usize];
    file.read_exact(&mut buffer)?;

    Ok(buffer)
}

#[derive(Debug)]
enum Operand {
    Rb,
    Rw,
    Eb, // effective address byte
    Ew, // effective word
    SR, // segment register
}

#[derive(Debug)]
enum Asm8086 {
    Mov(Operand, Operand),
    Unknown
}

fn opcode_to_instruction(opcode_byte: u8) -> Asm8086 {
    match opcode_byte {
        0o210 => Asm8086::Mov(Operand::Eb, Operand::Rb),
        0o211 => Asm8086::Mov(Operand::Ew, Operand::Rw),
        0o212 => Asm8086::Mov(Operand::Rb, Operand::Eb),
        0o213 => Asm8086::Mov(Operand::Rw, Operand::Ew),
        0o214 => Asm8086::Mov(Operand::Ew, Operand::SR),
        0o216 => Asm8086::Mov(Operand::SR, Operand::Ew),
        _  => Asm8086::Unknown
    }
}

fn resolve_mov_operands(byte: u8) -> (u8, u8, u8) {
   let x = (byte & 0b11000000) >> 6; 
   let r = byte & (0b00111000) >> 3;
   let m = byte & 0b00000111; 
   (x, r, m)
}
fn main() -> io::Result<()>{
     //let mut instructions = Vec::new();
    let mut bytes = read_bytes("multiple_mov.bin")?;
    bytes.as_mut_slice().reverse();
    while let Some(byte) =  bytes.pop() {
        match opcode_to_instruction(byte) {
            Asm8086::Mov(dest, src) => { 
                println!("{byte:#o} is MOV {dest:?}, {src:?}");
                if let Some(second_byte) = bytes.pop() {
                    let xrm = resolve_mov_operands(second_byte);
                    println!("xrm = {xrm:?}")
                } else {
                    panic!("Could not finish parsing...")
                }
            },
            Asm8086::Unknown => println!("no idea about {byte:#o}")
        }
    } 
    Ok(())
}

