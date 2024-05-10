mod board;
mod checkmate_checker;
mod create_all_next_boards;
mod create_all_next_boards_test;
mod error;
mod result;
mod shared;

pub(crate) use board::{Board, Coord, Piece, PieceKind, PieceStatus, BOARD_SIZE};
pub(crate) use error::Error;
pub(crate) use result::Result;
