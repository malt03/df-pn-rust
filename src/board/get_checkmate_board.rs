mod node;

use super::{Board, NextBoardKind};
use crate::Result;
use node::{NormalNode, Position::*};
use std::{collections::HashSet, fmt::Pointer};

impl Board {
    // pub fn get_checkmate_boards(self, n: i32) -> Result<Vec<Board>> {
    //     let mut root = NormalNode::new(self, Offense, NextBoardKind::Normal);
    //     for _ in 0..n {
    //         let history = HashSet::new();
    //         root.calc_pndn(&history)?;
    //         if root.pndn.pn == 0 || root.pndn.dn == 0 {
    //             break;
    //         }
    //     }

    //     let mut boards = Vec::new();
    //     if root.pndn.pn == 0 {
    //         for child in root.children() {
    //             if let Some(board) = child.board() {
    //                 boards.push(board);
    //             }
    //         }
    //     }

    //     Ok(boards)
    // }

    pub fn get_checkmate_board(&self, n: i32) -> Result<Option<Board>> {
        let mut root = NormalNode::new(self.reversed(), Offense, NextBoardKind::Normal);
        for _ in 0..n {
            let history = HashSet::new();
            root.calc_pndn(&history)?;
            if root.pndn.pn == 0 || root.pndn.dn == 0 {
                break;
            }
        }

        // let children = root.children();
        // loop {
        //     let Some(child) = children.pop_front() else {
        //         break;
        //     };
        //     println!("{}", child.board().unwrap());
        // }

        // unimplemented!()

        if root.pndn.pn != 0 {
            return Ok(None);
        }

        let Some(best) = root.children().pop_front() else {
            return Ok(None);
        };
        Ok(best.board())
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
        assert_eq!(b.get_checkmate_board(10)?, None);

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
            &b.get_checkmate_board(10)?.unwrap(),
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
