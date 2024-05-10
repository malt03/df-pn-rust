// mod node;

// use crate::{Board, Result};
// use node::{NormalNode, Position::*};
// use std::collections::HashSet;

// pub fn get_checkmate_board(board: Board, n: i32) -> Result<Option<Board>> {
//     let mut root = NormalNode::new(board, Offense, NextBoardKind::Normal);
//     for _ in 0..n {
//         let history = HashSet::new();
//         root.calc_pndn(&history)?;
//         if root.pndn.pn == 0 || root.pndn.dn == 0 {
//             break;
//         }
//     }

//     if root.pndn.pn != 0 {
//         return Ok(None);
//     }

//     let Some(best) = root.children().pop_front() else {
//         return Ok(None);
//     };
//     Ok(best.board())
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::common::shared::{
//         board::{assert_eq_board, pair::Index::*},
//         piece, Coord, Piece,
//     };
//     use crate::rules::five::{Five, FiveKind::*};

//     #[test]
//     fn test_uchifudume() -> Result<Five, ()> {
//         let mut base = Board::all_catched();
//         base[(King, First)] = Piece::moved(Coord::new(0, 4), false);
//         base[(King, Second)] = Piece {
//             status: piece::Status::EnemyBoard,
//             coord: Coord::new(4, 0),
//             is_changed: false,
//         };
//         base[(Fu, First)] = Piece::catched();

//         let mut b = base.clone();
//         b[(Hisha, First)] = Piece::moved(Coord::new(3, 2), true);
//         b.reload_board_map();
//         assert_eq!(get_checkmate_board(b.reversed(), 10)?, None);

//         let mut b = base.clone();
//         b[(Fu, Second)] = Piece::moved(Coord::new(2, 1), true);
//         b[(Kaku, First)] = Piece::moved(Coord::new(1, 4), false);
//         b[(Kin, Second)] = Piece::catched();
//         b[(Kin, First)] = Piece {
//             status: piece::Status::EnemyBoard,
//             coord: Coord::new(3, 0),
//             is_changed: false,
//         };
//         b.reload_board_map();
//         assert_eq_board(
//             get_checkmate_board(b.reversed(), 100)?.unwrap(),
//             "
//             銀飛銀角飛
//             ---------------
//                      ￬金￬王
//                   ￪と   ￪歩

//             ￪王￪角
//             ---------------
//             金",
//         );

//         Ok(())
//     }

//     #[test]
//     fn test_get_checkmate_board() -> Result<Five, ()> {
//         let mut base = Board::all_catched();
//         base[(King, First)] = Piece::moved(Coord::new(0, 4), false);

//         let mut b = base.clone();
//         b[(King, Second)] = Piece {
//             status: piece::Status::EnemyBoard,
//             coord: Coord::new(4, 0),
//             is_changed: false,
//         };
//         b[(Fu, First)] = Piece::moved(Coord::new(4, 2), false);
//         b[(Kin, First)] = Piece::catched();
//         b[(Hisha, First)] = Piece::catched();
//         b.reload_board_map();
//         assert_eq_board(
//             get_checkmate_board(b.reversed(), 10)?.unwrap(),
//             "
//             銀角歩金銀角飛
//             ---------------
//                         ￬王
//                         ￪金
//                         ￪歩

//             ￪王
//             ---------------
//             飛",
//         );

//         let mut b = base.clone();
//         b[(King, Second)] = Piece {
//             status: piece::Status::EnemyBoard,
//             coord: Coord::new(4, 2),
//             is_changed: false,
//         };
//         b[(Kin, First)] = Piece::catched();
//         b[(Kin, Second)] = Piece::catched();
//         b[(Gin, First)] = Piece::catched();
//         b[(Gin, Second)] = Piece::catched();
//         b[(Hisha, Second)] = Piece::moved(Coord::new(2, 4), false);
//         b[(Fu, First)] = Piece::moved(Coord::new(4, 4), false);
//         b.reload_board_map();
//         assert_eq_board(
//             get_checkmate_board(b.reversed(), 2000)?.unwrap(),
//             "
//             角飛歩角
//             ---------------

//                         ￬王
//                         ￪金
//             ￪王   ￪飛   ￪歩
//             ---------------
//             銀金銀",
//         );

//         Ok(())
//     }
// }
