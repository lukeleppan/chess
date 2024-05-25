use std::ops::Not;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SideColor {
    White,
    Black,
}

impl Not for SideColor {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self == Self::White {
            Self::White
        } else {
            Self::Black
        }
    }
}
