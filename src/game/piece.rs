use super::side_color::SideColor;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Piece {
    Pawn(SideColor),
    Knight(SideColor),
    Bishop(SideColor),
    Rook(SideColor),
    Queen(SideColor),
    King(SideColor),
}
