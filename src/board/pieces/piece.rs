mod coord;

use bincode::{Decode, Encode};
pub(crate) use coord::Coord;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Encode, Decode)]
pub(crate) enum Status {
    MyBoard,
    EnemyBoard,
    MyHand,
    EnemyHand,
}

use Status::*;

impl Status {
    pub(crate) fn on_board(&self) -> bool {
        match self {
            MyBoard | EnemyBoard => true,
            MyHand | EnemyHand => false,
        }
    }

    pub(crate) fn reversed(&self) -> Status {
        match self {
            MyBoard => EnemyBoard,
            EnemyBoard => MyBoard,
            MyHand => EnemyHand,
            EnemyHand => MyHand,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Encode, Decode)]
pub(crate) struct Piece {
    pub(crate) coord: Coord,
    pub(crate) is_changed: bool,
    pub(crate) status: Status,
}

impl Piece {
    pub(crate) fn new(x: i8, y: i8, status: Status, is_changed: bool) -> Piece {
        Piece {
            coord: Coord { x, y },
            is_changed,
            status,
        }
    }

    pub(crate) fn init(x: i8, y: i8, status: Status) -> Piece {
        Piece::new(x, y, status, false)
    }

    pub(crate) fn moved(coord: Coord, is_changed: bool) -> Piece {
        Piece {
            coord,
            is_changed,
            status: MyBoard,
        }
    }

    pub(crate) fn catched(is_mine: bool) -> Piece {
        Piece::init(0, 0, if is_mine { MyHand } else { EnemyHand })
    }
}
