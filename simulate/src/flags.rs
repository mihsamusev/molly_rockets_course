#[derive(Debug)]
pub enum Flags {
    Zero,
    Sign,
    Carry,
    Oveflow
}

impl Flags {
    pub fn names() -> [&'static str; 4] {
        ["Z", "S", "C", "O"]
    }

    pub fn index(&self) -> usize {
        match self {
            Self::Zero => 0,
            Self::Sign => 1,
            Self::Carry => 2,
            Self::Oveflow => 3,
        }
    }
}