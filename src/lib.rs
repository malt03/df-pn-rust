mod board;
mod error;
mod result;
mod shared;

pub(crate) use board::{Coord, NextBoardKind, Piece, PieceKind, BOARD_SIZE};

#[cfg(test)]
pub(crate) use board::PieceStatus;

pub use board::Board;
pub use error::Error;
pub use result::Result;
