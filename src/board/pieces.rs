mod kind;
mod piece;

use crate::shared::Set;
pub(crate) use kind::Kind;
pub(crate) use piece::{Coord, Piece, Status};
use std::ops::{Index, IndexMut};
use Kind::*;

#[cfg(test)]
use Status::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Pieces {
    elements: [Set<Piece>; 8],
}

impl Pieces {
    #[cfg(test)]
    pub(crate) fn first() -> Self {
        Pieces {
            elements: [
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

    #[cfg(test)]
    pub(crate) fn all_catched() -> Self {
        let all_catched_2 = Set::from((0..2).map(|_| Piece::catched(false)));
        let all_catched_4 = Set::from((0..4).map(|_| Piece::catched(false)));
        Pieces {
            elements: [
                Set::from((0..18).map(|_| Piece::catched(false))),
                all_catched_4.clone(),
                all_catched_4.clone(),
                all_catched_4.clone(),
                all_catched_4.clone(),
                all_catched_2.clone(),
                all_catched_2.clone(),
                all_catched_2.clone(),
            ],
        }
    }

    pub(crate) fn new() -> Self {
        Pieces {
            elements: [
                Set::with_capacity(18),
                Set::with_capacity(4),
                Set::with_capacity(4),
                Set::with_capacity(4),
                Set::with_capacity(4),
                Set::with_capacity(2),
                Set::with_capacity(2),
                Set::with_capacity(2),
            ],
        }
    }

    fn assert_len(&self, kind: Kind, expected: usize) {
        let len = self[kind].len();
        assert_eq!(len, expected, "{kind}.len() == {len} expected {expected}",);
    }

    pub(crate) fn validate_len(&self) {
        self.assert_len(Fu, 18);
        self.assert_len(Kyousha, 4);
        self.assert_len(Keima, 4);
        self.assert_len(Gin, 4);
        self.assert_len(Kin, 4);
        self.assert_len(Kaku, 2);
        self.assert_len(Hisha, 2);

        let king_len = self[King].len();
        assert!(
            king_len == 1 || king_len == 2,
            "{King}.len() == {king_len} expected 1 or 2",
        );
    }

    pub(crate) fn map<F>(&self, f: F) -> Pieces
    where
        F: Fn(&Piece) -> Piece,
    {
        Pieces {
            elements: [
                self[Fu].map(&f),
                self[Kyousha].map(&f),
                self[Keima].map(&f),
                self[Gin].map(&f),
                self[Kin].map(&f),
                self[Kaku].map(&f),
                self[Hisha].map(&f),
                self[King].map(&f),
            ],
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
