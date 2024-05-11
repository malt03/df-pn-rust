use crate::BOARD_SIZE;

use super::{Board, Coord, Piece, PieceKind, PieceStatus, CONTROL_MAP};
use PieceKind::*;
use PieceStatus::*;

fn get_hisha_vec(p1: &Piece, p2: &Piece) -> Option<Coord> {
    if p1.coord.x == p2.coord.x {
        Some(Coord::new(0, if p1.coord.y < p2.coord.y { 1 } else { -1 }))
    } else if p1.coord.y == p2.coord.y {
        Some(Coord::new(if p1.coord.x < p2.coord.x { 1 } else { -1 }, 0))
    } else {
        None
    }
}
fn get_kaku_vec(p1: &Piece, p2: &Piece) -> Option<Coord> {
    if i8::abs(p1.coord.x - p2.coord.x) == i8::abs(p1.coord.y - p2.coord.y) {
        Some(Coord::new(
            if p1.coord.x < p2.coord.x { 1 } else { -1 },
            if p1.coord.y < p2.coord.y { 1 } else { -1 },
        ))
    } else {
        None
    }
}
fn get_kyousha_vec(p1: &Piece, p2: &Piece, y_vector: i8) -> Option<Coord> {
    if p1.coord.x == p2.coord.x {
        if y_vector > 0 && p1.coord.y > p2.coord.y {
            return Some(Coord::new(0, -y_vector));
        }
        if y_vector < 0 && p1.coord.y < p2.coord.y {
            return Some(Coord::new(0, -y_vector));
        }
    }
    None
}

impl Board {
    pub(crate) fn is_checking(&self) -> bool {
        self.is_check_base(MyBoard, 1)
    }

    pub(crate) fn is_checked(&self) -> bool {
        self.is_check_base(EnemyBoard, -1)
    }

    fn is_check_base(&self, move_board: PieceStatus, y_vector: i8) -> bool {
        let Some(&king) = &self.pieces[King]
            .iter()
            .find(|&p| p.status == move_board.reversed())
        else {
            return false;
        };

        for (kind, _, p) in self.pieces.iter() {
            if p.status != move_board
                || i8::abs(p.coord.x - king.coord.x) > 1
                || i8::abs(p.coord.y - king.coord.y) > 1
            {
                continue;
            }
            for control in &CONTROL_MAP[kind][p.is_changed] {
                let x = p.coord.x + control.x;
                let y = p.coord.y + control.y * y_vector;
                if x == king.coord.x && y == king.coord.y {
                    return true;
                }
            }
        }

        let hisha_kaku_kyousha = |p: &Piece, kind: PieceKind| -> bool {
            let Some(vector) = (match kind {
                Hisha => get_hisha_vec(p, &king),
                Kaku => get_kaku_vec(p, &king),
                Kyousha => get_kyousha_vec(p, &king, y_vector),
                _ => panic!("Invalid kind: {:?}", kind),
            }) else {
                return false;
            };
            for n in 1..=(BOARD_SIZE as i8 - 1) {
                let coord = p.coord + vector * n;
                if coord.is_out_of_board() {
                    break;
                }
                if coord == king.coord {
                    return true;
                }
                if self.board_map[coord.y as usize][coord.x as usize].is_some() {
                    break;
                }
            }
            false
        };

        [Hisha, Kaku, Kyousha].iter().any(|&kind| {
            self.pieces[kind]
                .iter()
                .any(|p| p.status == move_board && hisha_kaku_kyousha(p, kind))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Piece;

    #[test]
    fn test_is_check_kyousha() {
        let mut b = Board::all_catched();
        b[King][0] = Piece::init(0, 0, EnemyBoard);
        b[Kyousha][0] = Piece::init(0, 6, MyBoard);
        b.reload_board_map();
        assert_eq!(b.is_checking(), true);
        assert_eq!(b.reversed().is_checked(), true);
    }
}
