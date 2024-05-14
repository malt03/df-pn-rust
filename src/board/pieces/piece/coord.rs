use crate::BOARD_SIZE;
use bincode::{Decode, Encode};
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Encode, Decode)]
pub(crate) struct Coord {
    pub(crate) x: i8,
    pub(crate) y: i8,
}

impl Coord {
    pub(crate) fn new(x: i8, y: i8) -> Coord {
        Coord { x, y }
    }

    pub(crate) fn is_out_of_board(&self) -> bool {
        self.x < 0 || self.x >= BOARD_SIZE as i8 || self.y < 0 || self.y >= BOARD_SIZE as i8
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<i8> for Coord {
    type Output = Coord;

    fn mul(self, other: i8) -> Coord {
        Coord {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
