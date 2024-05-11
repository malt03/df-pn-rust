mod board;
mod error;
mod result;
mod shared;

pub(crate) use board::{
    get_vectors, Coord, NextBoardKind, Piece, PieceKind, PieceStatus, BOARD_SIZE, CONTROL_MAP,
};

pub use board::Board;
pub use error::Error;
pub use result::Result;
