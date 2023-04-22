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
enum ByteRegister {
    AL,
    CL,
    DL,
    BL,
    AH,
    CH,
    DH,
    BH,
    Unread
}

use ByteRegister::*;
impl ByteRegister {
    const VALUES: [Self; 8] = [AL, CL, DL, BL, AH, CH, DH, BH];
    fn from_r(r: u8) -> Self {
        ByteRegister::VALUES[r as usize]
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
    Unread
}

use WordRegister::*;
impl WordRegister {
    const VALUES: [Self; 8] = [AX, CX, DX, BX, SP, BP, SI, DI];
    fn from_r(r: u8) -> Self {
        WordRegister::VALUES[r as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pointer {
    BX_SI(u16),
    BX_DI(u16),
    BP_SI(u16),
    BP_DI(u16),
    SI(u16),
    DI(u16),
    Direct(u16),
    BX(u16),
    Unread
}

impl Pointer {
    fn from_r(r: u8, value: u16) -> Self {
       match r {
        0 => Pointer::BX_SI(value),
        1 => Pointer::BX_DI(value),
        2 => Pointer::BP_SI(value),
        3 => Pointer::BP_DI(value),
        4 => Pointer::SI(value),
        5 => Pointer::DI(value),
        6 => Pointer::Direct(value),
        7 => Pointer::BX(value),
        _ => Pointer::Unread
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operand {
    Rb(ByteRegister),
    Rw(WordRegister),
    Eb(ByteRegister), // effective address byte
    Ew(WordRegister), // effective word
    Db(u8),
    Dw(u16),
    Dc(i8),
    DnUnread,
    DcUnread,
    DwUnread,
    SR, // segment register
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Operand::*;
        match self {
            Rb(reg) if (*reg) != ByteRegister::Unread => write!(f, "{reg:?}"),
            Eb(reg) if (*reg) != ByteRegister::Unread => write!(f, "{reg:?}"),
            Rw(reg) if (*reg) != WordRegister:: Unread => write!(f, "{reg:?}"),
            Ew(reg) if (*reg) != WordRegister:: Unread => write!(f, "{reg:?}"),
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
        0o210 => Asm8086::Mov(Eb(ByteRegister::Unread), Rb(ByteRegister::Unread)),
        0o211 => Asm8086::Mov(Ew(WordRegister::Unread), Rw(WordRegister::Unread)),
        0o212 => Asm8086::Mov(Rb(ByteRegister::Unread), Eb(ByteRegister::Unread)),
        0o213 => Asm8086::Mov(Rw(WordRegister::Unread), Ew(WordRegister::Unread)),
        0o214 => Asm8086::Mov(Ew(WordRegister::Unread), SR),
        0o216 => Asm8086::Mov(SR, Ew(WordRegister::Unread)),
        0o261 => Asm8086::Mov(Rb(ByteRegister::CL), DcUnread),
        0o265 => Asm8086::Mov(Rb(ByteRegister::CH), DcUnread),
        0o271 => Asm8086::Mov(Rw(WordRegister::CX), DwUnread),
        0o272 => Asm8086::Mov(Rw(WordRegister::DX), DwUnread),
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
        (Operand::Rb(ByteRegister::Unread), 0..=7) => Operand::Rb(ByteRegister::from_r(r)),
        (Operand::Eb(ByteRegister::Unread), 0..=7) => Operand::Eb(ByteRegister::from_r(r)),
        (Operand::Rw(WordRegister::Unread), 0..=7) => Operand::Rw(WordRegister::from_r(r)),
        (Operand::Ew(WordRegister::Unread), 0..=7) => Operand::Ew(WordRegister::from_r(r)),
        _ => register_operand
    }
}



fn resolve_operands_address(operand: Operand, x: u8, r_or_s: u8, m: u8) -> Operand {
    use Operand::*;
    match (operand, x, m) {
        (Rb(_) | Rw(_), _, _) => resolve_register_mode(operand, r_or_s),
        (Eb(_) | Ew(_), 3, _) => resolve_register_mode(operand, m),
        _ => operand
    }
}

fn to_u16(low_bit: u8, high_bit: u8) -> u16 {
    ((high_bit as u16) << 8) | (low_bit as u16)
}

fn main() -> Result<(), String>{
     //let mut instructions = Vec::new();
    let mut bytes = read_bytes("listing_0039_more_movs.bin").expect("cant");
    bytes.as_mut_slice().reverse();
    while let Some(first_bit) =  bytes.pop() {
        let opcode =opcode_to_instruction(first_bit);  
        match opcode {
            Asm8086::Mov(reg, value) if value == Operand::DcUnread => {
                let value_bit = bytes.pop().ok_or("could not finish parsing")?;
                let value = value_bit as i8;
                println!("[{first_bit:#o}][{value_bit:#o}] = MOV {reg}, {value}")
            },
            Asm8086::Mov(reg, value) if value == Operand::DwUnread => {
                let low_bit = bytes.pop().ok_or("could not finish parsing")?;
                let high_bit = bytes.pop().ok_or("could not finish parsing")?;
                let value= to_u16(low_bit, high_bit);
                println!("[{first_bit:#o}][{low_bit:#o}][{high_bit:#o}] = MOV {reg}, {value}")
            },
            Asm8086::Mov(dest, src) => { 
                let second_bit = bytes.pop().ok_or("could not finish parsing")?;
                let (x, r_or_s, m)= resolve_mov_operands(second_bit);
                match x {
                    0 => {
                        let value = if r_or_s == 6 { second_bit} else {0};
                        Pointer::from_r(r_or_s, value as u16);
                        println!("[{first_bit:#o}][{second_bit:#o}] = MOV {dest}, {src}");
                    },
                    1 => {
                        let low_bit = bytes.pop().ok_or("could not finish parsing")?;
                        println!("[{first_bit:#o}][{second_bit:#o}][{low_bit:#o}] = MOV {dest}, {src}");
                    },
                    2 => {
                        let low_bit = bytes.pop().ok_or("could not finish parsing")?;
                        let high_bit = bytes.pop().ok_or("could not finish parsing")?;
                        let value= to_u16(low_bit, high_bit);
                        println!("[{first_bit:#o}][{second_bit:#o}][{low_bit:#0}][{high_bit:#o}] = MOV {dest}, {src}");
                   },
                    3 => {
                        let src = resolve_operands_address(src, x, r_or_s, m);
                        let dest = resolve_operands_address(dest, x, r_or_s, m);
                        println!("[{first_bit:#o}][{second_bit:#o}] = MOV {dest}, {src}");
                    },
                    _ => {}
                }
            },
            Asm8086::Unknown => println!("unable to parse opcode bit {first_bit:#o}")
        }
    } 
    Ok(())
}


//  ; Register-to-register
//   20   │ mov si, bx
//   21   │ mov dh, al
//   22   │ 
//   23   │ ; 8-bit immediate-to-register
//   24   │ mov cl, 12
//   25   │ mov ch, -12
//   26   │ 
//   27   │ ; 16-bit immediate-to-register
//   28   │ mov cx, 12
//   29   │ mov cx, -12
//   30   │ mov dx, 3948
//   31   │ mov dx, -3948
//   32   │ 
//   33   │ ; Source address calculation
//   34   │ mov al, [bx + si]
//   35   │ mov bx, [bp + di]
//   36   │ mov dx, [bp]
//   37   │ 
//   38   │ ; Source address calculation plus 8-bit displacement
//   39   │ mov ah, [bx + si + 4]
//   40   │ 
//   41   │ ; Source address calculation plus 16-bit displacement
//   42   │ mov al, [bx + si + 4999]
//   43   │ 
//   44   │ ; Dest address calculation
//   45   │ mov [bx + di], cx
//   46   │ mov [bp + si], cl
//   47   │ mov [bp], ch


