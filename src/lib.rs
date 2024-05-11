mod board;
mod checkmate_checker;
mod error;
mod result;
mod shared;

pub(crate) use board::{
    get_vectors, Coord, NextBoardKind, Piece, PieceKind, PieceStatus, BOARD_SIZE, CONTROL_MAP,
};
pub(crate) use error::Error;
pub(crate) use result::Result;

pub use board::Board;
