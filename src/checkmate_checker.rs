mod node;

use crate::{Board, NextBoardKind, Result};
use node::{NormalNode, Position::*};
use std::collections::HashSet;

pub(crate) fn get_checkmate_board(board: Board, n: i32) -> Result<Option<Board>> {
    let mut root = NormalNode::new(board, Offense, NextBoardKind::Normal);
    for _ in 0..n {
        let history = HashSet::new();
        root.calc_pndn(&history)?;
        if root.pndn.pn == 0 || root.pndn.dn == 0 {
            break;
        }
    }

    if root.pndn.pn != 0 {
        return Ok(None);
    }

    let Some(best) = root.children().pop_front() else {
        return Ok(None);
    };
    Ok(best.board())
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
        println!("{b}");
        assert_eq!(get_checkmate_board(b.reversed(), 10)?, None);

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
            &get_checkmate_board(b.reversed(), 10)?.unwrap(),
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
