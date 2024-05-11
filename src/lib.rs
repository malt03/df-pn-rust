mod board;
mod error;
mod result;
mod shared;

pub(crate) use board::{Coord, NextBoardKind, Piece, PieceKind, BOARD_SIZE};
pub(crate) use error::Error;
pub(crate) use result::Result;

#[cfg(test)]
pub(crate) use board::PieceStatus;

pub use board::{Board, CheckmateResult};
