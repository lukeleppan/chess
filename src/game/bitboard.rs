use core::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, PartialEq, Default)]
pub struct Bitboard(pub u64);

impl Debug for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut board = String::new();
        for rank in (0..8).rev() {
            for file in 0..8 {
                let bit = 1 << (rank * 8 + file);
                if self.0 & bit != 0 {
                    board.push('1');
                } else {
                    board.push('0');
                }
            }
            if rank != 0 {
                board.push('\n');
            }
        }
        write!(f, "\n{}", board)
    }
}

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Bitboard(value)
    }
}
