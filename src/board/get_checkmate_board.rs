mod node;

use super::Board;
use crate::db;
use node::NormalNode;
use std::{collections::HashSet, path::Path};

pub enum CheckmateResult<T> {
    Checkmate(T, usize),
    NotCheckmate(T, usize),
    Unproven,
}

impl<T> CheckmateResult<T> {
    pub fn is_checkmate(&self) -> bool {
        match self {
            CheckmateResult::Checkmate(_, _) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
impl<T> CheckmateResult<T> {
    fn is_not_checkmate(&self) -> bool {
        match self {
            CheckmateResult::NotCheckmate(_, _) => true,
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
    pub fn get_checkmate_boards<P>(
        &self,
        db_path: P,
        reset_db: bool,
        n: Option<usize>,
        max_depth: Option<usize>,
    ) -> CheckmateResult<Vec<Board>>
    where
        P: AsRef<Path>,
    {
        let db = db::open_with_cache(db_path, reset_db).unwrap();

        let mut root = NormalNode::new(&db, self.reversed());
        let mut i = 0;
        loop {
            let history = HashSet::new();
            root.calc_pndn(&db, &history, max_depth);
            if root.pndn.pn == 0 || root.pndn.dn == 0 {
                break;
            }

            if i % 50000 == 0 {
                root.dump_single_best_board(&db);
                println!("{i}");
            }

            i += 1;
            if let Some(n) = n {
                if i == n {
                    break;
                }
            }
        }

        if root.pndn.pn == 0 {
            let mut best_boards = root.best_boards(&db);
            best_boards.pop();
            CheckmateResult::Checkmate(best_boards, i)
        } else if root.pndn.dn == 0 {
            let mut best_boards = root.best_boards(&db);
            best_boards.pop();
            CheckmateResult::NotCheckmate(best_boards, i)
        } else {
            CheckmateResult::Unproven
        }
    }

    #[cfg(test)]
    fn get_checkmate_board<P>(
        &self,
        db_path: P,
        reset_db: bool,
        n: usize,
        max_depth: Option<usize>,
    ) -> CheckmateResult<Board>
    where
        P: AsRef<Path>,
    {
        match self.get_checkmate_boards(db_path, reset_db, Some(n), max_depth) {
            CheckmateResult::Checkmate(mut boards, n) => {
                CheckmateResult::Checkmate(boards.pop().unwrap(), n)
            }
            CheckmateResult::NotCheckmate(mut boards, n) => {
                CheckmateResult::NotCheckmate(boards.pop().unwrap(), n)
            }
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
        assert!(b
            .get_checkmate_board("/tmp/df_pn.test.rocksdb", true, 200, None)
            .is_not_checkmate(),);
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
            &b.get_checkmate_board("/tmp/df_pn.test.rocksdb", true, 10, None)
                .unwrap(),
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
