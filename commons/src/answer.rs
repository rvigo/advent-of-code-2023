#[derive(Debug, PartialEq, Eq)]
pub enum Answer {
    Number(u32),
}

impl From<u32> for Answer {
    fn from(value: u32) -> Self {
        Self::Number(value as u32)
    }
}
