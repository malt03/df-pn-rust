mod node;

use super::{Board, NextBoardKind};
use node::{NormalNode, Position::*};
use std::collections::HashSet;

pub enum CheckmateResult<T> {
    Checkmate(T, usize),
    NotCheckmate(usize),
    Unproven,
}

#[cfg(test)]
impl<T> CheckmateResult<T> {
    fn is_not_checkmate(&self) -> bool {
        match self {
            CheckmateResult::NotCheckmate(_) => true,
            _ => false,
        }
    }

    fn unwrap(self) -> T {
        match self {
            CheckmateResult::Checkmate(value, _) => value,
            _ => panic!("called `CheckmateResult::unwrap()` on a `NotCheckmate` value"),
        }
    }
}

impl Board {
    pub fn get_checkmate_boards(
        &self,
        n: usize,
        max_depth: Option<usize>,
    ) -> CheckmateResult<Vec<Board>> {
        let mut root = NormalNode::new(self.reversed(), Offense, NextBoardKind::Normal);
        let mut count = 0;
        for i in 0..n {
            count = i;
            let history = HashSet::new();
            root.calc_pndn(&history, max_depth);
            if root.pndn.pn == 0 || root.pndn.dn == 0 {
                break;
            }
        }

        if root.pndn.pn != 0 {
            return if count == n - 1 {
                CheckmateResult::Unproven
            } else {
                CheckmateResult::NotCheckmate(count)
            };
        }

        let mut best_boards = root.best_boards();
        best_boards.pop();

        CheckmateResult::Checkmate(best_boards, count)
    }

    pub fn get_checkmate_board(
        &self,
        n: usize,
        max_depth: Option<usize>,
    ) -> CheckmateResult<Board> {
        match self.get_checkmate_boards(n, max_depth) {
            CheckmateResult::Checkmate(mut boards, n) => {
                CheckmateResult::Checkmate(boards.pop().unwrap(), n)
            }
            CheckmateResult::NotCheckmate(n) => CheckmateResult::NotCheckmate(n),
            CheckmateResult::Unproven => CheckmateResult::Unproven,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{board::assert_eq_board, Coord, Piece, PieceKind::*, PieceStatus::*};

    #[test]
    fn test_uchifudume() {
        let mut b = Board::all_catched();
        b[King][0] = Piece::moved(Coord::new(0, 8), false);
        b[King][1] = Piece::new(8, 0, EnemyBoard, false);
        b[Fu][0] = Piece::catched(true);
        b[Hisha][0] = Piece::moved(Coord::new(7, 2), true);
        b.reload_board_map();
        assert!(b.get_checkmate_board(10, None).is_not_checkmate(),);
    }

    #[test]
    fn test_get_checkmate_board() {
        let mut b = Board::all_catched();
        b[King][0] = Piece::moved(Coord::new(0, 8), false);

        b[King][1] = Piece::new(4, 0, EnemyBoard, false);
        b[Fu][0] = Piece::moved(Coord::new(4, 2), false);
        b[Kin][0] = Piece::catched(true);
        b[Hisha][0] = Piece::catched(true);
        b.reload_board_map();
        assert_eq_board(
            &b.get_checkmate_board(10, None).unwrap(),
            "
歩x17 香x4 桂x4 銀x4 金x3 角x2 飛
------------------
            ￬王            
            ￪金            
            ￪歩            
                           
                           
                           
                           
                           
￪王                        
------------------
飛",
        );
    }
}
