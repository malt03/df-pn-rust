mod node;

use super::{Board, NextBoardKind};
use crate::Result;
use node::{NormalNode, Position::*};
use std::collections::HashSet;

impl Board {
    pub fn get_checkmate_boards(
        &self,
        n: usize,
        max_depth: Option<usize>,
    ) -> Result<Option<(Vec<Board>, usize)>> {
        let mut root = NormalNode::new(self.reversed(), Offense, NextBoardKind::Normal);
        let mut count = 0;
        for i in 0..n {
            count = i;
            let history = HashSet::new();
            root.calc_pndn(&history, max_depth)?;
            if root.pndn.pn == 0 || root.pndn.dn == 0 {
                break;
            }
        }

        if root.pndn.pn != 0 {
            return Ok(None);
        }

        let mut best_boards = root.best_boards();
        best_boards.pop();
        Ok(Some((best_boards, count)))
    }

    pub fn get_checkmate_board(&self, n: usize, max_depth: Option<usize>) -> Result<Option<Board>> {
        let Some((mut boards, _)) = self.get_checkmate_boards(n, max_depth)? else {
            return Ok(None);
        };
        Ok(boards.pop())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{board::assert_eq_board, Coord, Piece, PieceKind::*, PieceStatus::*};

    #[test]
    fn test_uchifudume() -> Result<()> {
        let mut b = Board::all_catched();
        b[King][0] = Piece::moved(Coord::new(0, 8), false);
        b[King][1] = Piece::new(8, 0, EnemyBoard, false);
        b[Fu][0] = Piece::catched(true);
        b[Hisha][0] = Piece::moved(Coord::new(7, 2), true);
        b.reload_board_map();
        assert_eq!(b.get_checkmate_board(10, None)?, None);

        Ok(())
    }

    #[test]
    fn test_get_checkmate_board() -> Result<()> {
        let mut b = Board::all_catched();
        b[King][0] = Piece::moved(Coord::new(0, 8), false);

        b[King][1] = Piece::new(4, 0, EnemyBoard, false);
        b[Fu][0] = Piece::moved(Coord::new(4, 2), false);
        b[Kin][0] = Piece::catched(true);
        b[Hisha][0] = Piece::catched(true);
        b.reload_board_map();
        assert_eq_board(
            &b.get_checkmate_board(10, None)?.unwrap(),
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

        Ok(())
    }
}
