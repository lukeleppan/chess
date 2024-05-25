#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Square(u8);

impl Default for Square {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Square {
    pub const fn new(sq: u8) -> Self {
        Self(sq & 63)
    }
}
