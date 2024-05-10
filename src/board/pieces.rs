mod kind;
mod piece;

use crate::shared::Set;
pub(crate) use kind::Kind;
pub(crate) use piece::{Coord, Piece, Status};
use std::ops::{Index, IndexMut};
use Status::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Pieces {
    elements: Vec<Set<Piece>>,
}

impl Pieces {
    pub(crate) fn first() -> Self {
        Pieces {
            elements: vec![
                Set::from(
                    (0..9)
                        .flat_map(|i| [Piece::init(i, 6, MyBoard), Piece::init(i, 2, EnemyBoard)]),
                ),
                Set::from([
                    Piece::init(0, 8, MyBoard),
                    Piece::init(8, 8, MyBoard),
                    Piece::init(0, 0, EnemyBoard),
                    Piece::init(8, 0, EnemyBoard),
                ]),
                Set::from([
                    Piece::init(1, 8, MyBoard),
                    Piece::init(7, 8, MyBoard),
                    Piece::init(1, 0, EnemyBoard),
                    Piece::init(7, 0, EnemyBoard),
                ]),
                Set::from([
                    Piece::init(2, 8, MyBoard),
                    Piece::init(6, 8, MyBoard),
                    Piece::init(2, 0, EnemyBoard),
                    Piece::init(6, 0, EnemyBoard),
                ]),
                Set::from([
                    Piece::init(3, 8, MyBoard),
                    Piece::init(5, 8, MyBoard),
                    Piece::init(3, 0, EnemyBoard),
                    Piece::init(5, 0, EnemyBoard),
                ]),
                Set::from([Piece::init(1, 7, MyBoard), Piece::init(7, 1, EnemyBoard)]),
                Set::from([Piece::init(7, 7, MyBoard), Piece::init(1, 1, EnemyBoard)]),
                Set::from([Piece::init(4, 8, MyBoard), Piece::init(4, 0, EnemyBoard)]),
            ],
        }
    }

    pub(crate) fn map<F>(&self, f: F) -> Pieces
    where
        F: Fn(&Piece) -> Piece,
    {
        Pieces {
            elements: self.elements.iter().map(|set| set.map(&f)).collect(),
        }
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (Kind, usize, &Piece)> {
        self.elements.iter().enumerate().flat_map(|(i, set)| {
            set.iter()
                .enumerate()
                .map(move |(j, p)| (Kind::from(i), j, p))
        })
    }
}

impl Index<Kind> for Pieces {
    type Output = Set<Piece>;

    fn index(&self, index: Kind) -> &Self::Output {
        &self.elements[index as usize]
    }
}
impl IndexMut<Kind> for Pieces {
    fn index_mut(&mut self, index: Kind) -> &mut Self::Output {
        &mut self.elements[index as usize]
    }
}
