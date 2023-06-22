use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Reg {
    AX,
    BX,
    CX,
    DX,
    SP,
    BP,
    SI,
    DI,
}

impl Reg {
    pub fn names() -> [&'static str; 8] {
        ["AX", "BX", "CX", "DX", "SP", "BP", "SI", "DI"]
    }

    pub fn index(&self) -> usize {
        match self {
            Self::AX => 0,
            Self::BX => 1,
            Self::CX => 2,
            Self::DX => 3,
            Self::SP => 4,
            Self::BP => 5,
            Self::SI => 6,
            Self::DI => 7,
        }
    }
}
impl FromStr for Reg {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "ax" => Ok(Self::AX),
            "bx" => Ok(Self::BX),
            "cx" => Ok(Self::CX),
            "dx" => Ok(Self::DX),
            "sp" => Ok(Self::SP),
            "bp" => Ok(Self::BP),
            "si" => Ok(Self::SI),
            "di" => Ok(Self::DI),
            _ => Err(String::from("didnt find register")),
        }
    }
}
