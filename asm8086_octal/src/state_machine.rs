

fn byte_octals(byte: u8) -> (u8, u8, u8) {
    let first = (byte & 0b11000000) >> 6;
    let second = (byte & 0b00111000) >> 3;
    let third = byte & 0b00000111;
    (first, second, third)
}

#[derive(Debug, Clone, Copy)]
enum NextPart {
    Opcode,
    DestByteAddress,
    DestWordAddress,
    SrcByteAddress,
    SrcWordAddress,
    ByteDisp,
    WordDisp,
}

#[derive(Debug, Clone, Copy)]
enum ParserState {
    Byte(u8),
    Word(u16),
    NeedByteForAddress,
    NeedByteDisp,
    NeedWordDisp,
    Done,
    Error
}

fn parse_opcode(out: &mut String, byte: u8) -> (NextPart, ParserState) {
    let (o1, o2, o3) = byte_octals(byte);
    match (o1, o2) {
       (0, 0) => out.push_str("add "),
       (2, 1) => out.push_str("mov "),
       _ => {}
    };
    match o3  {
        0 => (NextPart::DestByteAddress, ParserState::NeedByteForAddress),
        1 => (NextPart::DestWordAddress, ParserState::NeedByteForAddress),
        2 => (NextPart::SrcByteAddress, ParserState::NeedByteForAddress),
        3 => (NextPart::SrcWordAddress, ParserState::NeedByteForAddress),
        _ => (NextPart::Opcode, ParserState::Error)
    }
 
}

fn state_machine(out: &mut String, part: NextPart, parsing: ParserState) -> (NextPart, ParserState) {
    match (part, parsing) {
        (NextPart::Opcode, ParserState::Byte(byte)) => {
            parse_opcode(out, byte)
       },
        _ => (NextPart::Opcode, ParserState::Done)
    }
}

fn parse_bytes(bytes: &[u8]) {
    let mut ptr = 0;
    let mut part = NextPart::Opcode;
    let mut parsing = ParserState::NeedByteForAddress;
    let mut out = String::new();
    loop {
        let bytes_left = ptr != bytes.len();
        parsing = match (bytes_left, parsing) {
            (true, ParserState::NeedByteForAddress) => {
                let byte = bytes[ptr];
                ptr += 1;
                ParserState::Byte(byte) 
            },
            (_, _) => {
                println!("Unable to parse symbol");
                break
            }
            (false, ParserState::Done) => {
                println!("Finished sucessully");
                break
            }
            (false, _) => {
                println!("Finished with error");
                break
            }
            };
        }
        let new_state = state_machine(&mut out, part, parsing);
    }

