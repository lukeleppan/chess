use super::{bitboard::Bitboard, side_color::SideColor};

struct Board {
    pieces: [Bitboard; 6],
    color_occupied: [Bitboard; 2],
    occupied: Bitboard,
    side_to_move: SideColor,
}
