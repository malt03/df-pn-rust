mod control_map;

use std::collections::HashSet;

use crate::{Board, Coord, Error, Piece, PieceKind, PieceStatus, Result, BOARD_SIZE};
use control_map::{get_vectors, CONTROL_MAP};
use PieceKind::*;
use PieceStatus::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum NextBoardKind {
    Normal,
    Uchifu,
}

impl Board {
    fn append_moved_boards(
        &self,
        boards: &mut Vec<(Board, NextBoardKind)>,
        kind: PieceKind,
        i: usize,
        p: &Piece,
        vec: Coord,
    ) -> Result<bool> {
        let new_coord = p.coord + vec;
        if new_coord.is_out_of_board() {
            return Ok(false);
        }
        let is_changable_pos = (p.coord.y == 0 || new_coord.y == 0) && !p.is_changed;
        let is_changable = is_changable_pos && kind.is_changable();
        let is_force_change = is_changable_pos && kind.is_force_change();

        let target_piece_three = self.piece_at(&new_coord);

        if let Some((target_piece, target_kind, target_i)) = target_piece_three {
            if target_kind == King && target_piece.status == EnemyBoard {
                let mut cloned = self.clone();
                cloned[kind][i] = Piece::moved(new_coord, p.is_changed);
                cloned[King][target_i] = Piece::catched(true);
                return Err(Error::CatchKing(cloned));
            }
            if self[target_kind][target_i].status == MyBoard {
                return Ok(false);
            }
        }
        let catch_if_needed = |board: &mut Board| {
            if let Some((_, k, i)) = target_piece_three {
                board[k][i] = Piece::catched(true);
            }
        };
        if is_changable || is_force_change {
            let mut cloned = self.clone();
            cloned[kind][i] = Piece::moved(new_coord, true);
            catch_if_needed(&mut cloned);
            cloned.reload_board_map();
            boards.push((cloned, NextBoardKind::Normal));
        }
        if !is_force_change {
            let mut cloned = self.clone();
            cloned[kind][i] = Piece::moved(new_coord, p.is_changed);
            catch_if_needed(&mut cloned);
            cloned.reload_board_map();
            boards.push((cloned, NextBoardKind::Normal));
        }

        Ok(!target_piece_three.is_some())
    }

    fn append_vector_moved_boards(
        &self,
        boards: &mut Vec<(Board, NextBoardKind)>,
        kind: PieceKind,
        i: usize,
        vectors: &[Coord],
    ) -> Result<()> {
        let p = &self[kind][i];
        for vector in vectors {
            for n in 1..=(BOARD_SIZE as i8 - 1) {
                if !self.append_moved_boards(boards, kind, i, p, *vector * n)? {
                    break;
                }
            }
        }
        Ok(())
    }

    fn append_put_boards(
        &self,
        boards: &mut Vec<(Board, NextBoardKind)>,
        kind: PieceKind,
        i: usize,
        empty_cells: &Vec<Coord>,
    ) {
        let mut put = |empty_coord: Coord, next_board_kind: NextBoardKind| {
            let mut cloned = self.clone();
            cloned[kind][i] = Piece::moved(empty_coord, false);
            cloned.reload_board_map();
            boards.push((cloned, next_board_kind));
        };
        if kind == Fu {
            for empty_coord in empty_cells.iter() {
                if empty_coord.y == 0 {
                    continue;
                }
                let mut is_nifu = false;
                for y in 0..9 {
                    let Some((p, kind, _)) = self.piece_at(&Coord::new(empty_coord.x, y)) else {
                        continue;
                    };
                    if kind == Fu && p.status == MyBoard && !p.is_changed {
                        is_nifu = true;
                        break;
                    }
                }
                if is_nifu {
                    continue;
                }
                put(*empty_coord, NextBoardKind::Uchifu);
            }
        } else if kind.is_force_change() {
            for empty_coord in empty_cells.iter() {
                if empty_coord.y == 0 {
                    continue;
                }
                put(*empty_coord, NextBoardKind::Normal);
            }
        } else {
            for empty_coord in empty_cells.iter() {
                put(*empty_coord, NextBoardKind::Normal);
            }
        }
    }

    pub(crate) fn create_all_next_boards(&self) -> Result<Vec<(Board, NextBoardKind)>> {
        let mut boards = Vec::new();
        let empty_cells = (0..BOARD_SIZE)
            .flat_map(|y| {
                (0..BOARD_SIZE)
                    .filter(move |x: &usize| self.board_map[y][*x].is_none())
                    .map(move |x| Coord::new(x as i8, y as i8))
            })
            .collect();

        let mut put_kinds = HashSet::new();
        for (k, i, p) in self.pieces.iter() {
            match p.status {
                MyBoard => {
                    for vec in &CONTROL_MAP[k][p.is_changed] {
                        self.append_moved_boards(&mut boards, k, i, p, *vec)?;
                    }
                }
                MyHand => {
                    if put_kinds.contains(&k) {
                        continue;
                    }
                    self.append_put_boards(&mut boards, k, i, &empty_cells);
                    put_kinds.insert(k);
                }
                _ => {}
            }
        }

        for kind in [Kaku, Hisha] {
            for (i, p) in self.pieces[kind].iter().enumerate() {
                if p.status != MyBoard {
                    continue;
                }
                self.append_vector_moved_boards(&mut boards, Kaku, i, get_vectors(kind))?;
            }
        }

        return Ok(boards);
    }
}
