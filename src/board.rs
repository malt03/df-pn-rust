mod control_map;
mod create_all_next_boards;
mod create_all_next_boards_test;
mod is_checked;
mod pieces;

use crate::shared::Set;
use colored::Colorize;
pub(crate) use control_map::{get_vectors, CONTROL_MAP};
pub(crate) use create_all_next_boards::NextBoardKind;
pub(crate) use pieces::{Coord, Kind as PieceKind, Piece, Pieces, Status as PieceStatus};
use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};
use PieceStatus::*;

const UP_ARROW: char = '￪';
const DOWN_ARROW: char = '￬';

#[derive(Clone, Debug, Hash)]
pub struct Board {
    pub(crate) pieces: Pieces,
    pub(crate) board_map: Vec<Vec<Option<(PieceKind, usize)>>>,
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.pieces == other.pieces
    }
}
impl Eq for Board {}

impl Index<PieceKind> for Board {
    type Output = Set<Piece>;

    fn index(&self, index: PieceKind) -> &Self::Output {
        &self.pieces[index]
    }
}
impl IndexMut<PieceKind> for Board {
    fn index_mut(&mut self, index: PieceKind) -> &mut Self::Output {
        &mut self.pieces[index]
    }
}

pub(crate) const BOARD_SIZE: usize = 9;

impl Board {
    pub(crate) fn new(pieces: Pieces) -> Board {
        Board {
            pieces,
            board_map: vec![vec![None; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    pub(crate) fn first() -> Board {
        let mut board = Board::new(Pieces::first());
        board.reload_board_map();
        board
    }

    pub(crate) fn all_catched() -> Board {
        let mut board = Board::new(Pieces::all_catched());
        board.reload_board_map();
        board
    }

    pub fn parse<S>(str: S) -> Board
    where
        S: AsRef<str>,
    {
        let mut pieces = Pieces::new();

        let mut lines = str.as_ref().lines();
        let enemy_hands = PieceKind::parse_hands(lines.next().unwrap());
        for (kind, n) in enemy_hands {
            pieces[kind].extend((0..n).map(|_| Piece::catched(false)));
        }
        lines.next();

        for y in 0..BOARD_SIZE as i8 {
            let line = lines.next().unwrap();
            let mut chars = line.chars();
            for x in 0..BOARD_SIZE as i8 {
                let Some(c) = chars.next() else { break };
                let status = match c {
                    ' ' => {
                        for space in (0..2).map(|_| chars.next()) {
                            let Some(space) = space else { break };
                            if space != ' ' {
                                panic!("Invalid space: {space} ({x},{y})");
                            }
                        }
                        continue;
                    }
                    UP_ARROW => MyBoard,
                    DOWN_ARROW => EnemyBoard,
                    _ => panic!("Invalid arrow: {c} ({x},{y})"),
                };

                let c = chars
                    .next()
                    .expect(format!("Empty char: ({x},{y})").as_str());
                let (kind, is_changed) = PieceKind::safe_parse(c)
                    .expect(format!("Invalid char: {c} ({x},{y})").as_str());

                pieces[kind].push(Piece::new(x, y, status, is_changed));
            }
        }

        lines.next();
        let my_hands = if let Some(line) = lines.next() {
            PieceKind::parse_hands(line)
        } else {
            HashMap::new()
        };
        for (kind, n) in my_hands {
            pieces[kind].extend((0..n).map(|_| Piece::catched(true)));
        }

        pieces.validate_len();

        let mut board = Board::new(pieces);
        board.reload_board_map();
        board
    }

    pub(crate) fn reversed(&self) -> Board {
        let pieces = self.pieces.map(|p| Piece {
            coord: Coord {
                x: BOARD_SIZE as i8 - p.coord.x - 1,
                y: BOARD_SIZE as i8 - p.coord.y - 1,
            },
            is_changed: p.is_changed,
            status: p.status.reversed(),
        });
        let mut board = Board::new(pieces);
        board.reload_board_map();
        board
    }

    pub(crate) fn reload_board_map(&mut self) {
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                self.board_map[y][x] = None;
            }
        }
        for (kind, i, p) in self.pieces.iter() {
            if p.status.on_board() {
                self.board_map[p.coord.y as usize][p.coord.x as usize] = Some((kind, i));
            }
        }
    }

    pub(crate) fn piece_at(&self, coord: &Coord) -> Option<(&Piece, PieceKind, usize)> {
        let Some((kind, i)) = self.board_map[coord.y as usize][coord.x as usize] else {
            return None;
        };
        Some((&self[kind][i], kind, i))
    }

    pub(crate) fn dump_to<W>(&self, w: &mut W, colored: bool) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        let arrow = |is_mine: bool| -> String {
            if colored {
                String::new()
            } else {
                if is_mine {
                    UP_ARROW.to_string()
                } else {
                    DOWN_ARROW.to_string()
                }
            }
        };
        let e = |kind: PieceKind, is_changed: bool| -> String {
            let t = kind.title(is_changed);
            if colored {
                t.red().to_string()
            } else {
                t.to_string()
            }
        };
        let m = |kind: PieceKind, is_changed: bool| -> String {
            let t = kind.title(is_changed);
            if colored {
                t.green().to_string()
            } else {
                t.to_string()
            }
        };

        let mut my_hands = HashMap::<PieceKind, u8>::new();
        let mut enemy_hands = HashMap::<PieceKind, u8>::new();
        for (kind, _, p) in self.pieces.iter() {
            match p.status {
                MyHand => *my_hands.entry(kind).or_insert(0) += 1,
                EnemyHand => *enemy_hands.entry(kind).or_insert(0) += 1,
                _ => {}
            }
        }

        let enemy_hands_str = PieceKind::iter()
            .filter_map(|kind| enemy_hands.get(&kind).map(|&number| (kind, number)))
            .map(|(kind, number)| {
                if number == 1 {
                    e(kind, false)
                } else {
                    format!("{}x{number}", e(kind, false))
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
        writeln!(w, "{enemy_hands_str}\n------------------")?;

        for row in &self.board_map {
            for i in row {
                let Some((kind, i)) = *i else {
                    write!(w, "{}", if colored { "  " } else { "   " })?;
                    continue;
                };
                let p = &self[kind][i];
                match p.status {
                    MyBoard => write!(w, "{}{}", arrow(true), m(kind, p.is_changed))?,
                    EnemyBoard => write!(w, "{}{}", arrow(false), e(kind, p.is_changed))?,
                    _ => panic!("Invalid status"),
                }
            }
            writeln!(w)?;
        }

        let my_hands_str = PieceKind::iter()
            .filter_map(|kind| my_hands.get(&kind).map(|&number| (kind, number)))
            .map(|(kind, number)| {
                if number == 1 {
                    m(kind, false)
                } else {
                    format!("{}x{number}", m(kind, false))
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
        write!(w, "------------------\n{my_hands_str}")?;

        Ok(())
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.dump_to(f, true)
    }
}

#[cfg(test)]
pub(crate) fn assert_eq_board(left: &Board, right: &'static str) {
    let mut output = String::new();
    left.dump_to(&mut output, false).unwrap();
    assert_eq!(output.to_string(), right.strip_prefix("\n").unwrap());
}

#[cfg(test)]
mod tests {
    use super::PieceKind::*;
    use super::*;

    #[test]
    fn test_reversed() {
        let mut board = Board::first();
        assert_eq!(board, board.reversed());

        board[Fu][0] = Piece::catched(true);
        board[Kyousha][0] = Piece::catched(false);
        board[Kaku][0] = Piece::new(1, 4, EnemyBoard, true);
        board.reload_board_map();

        assert_eq_board(
            &board,
            "
香
------------------
￬香￬桂￬銀￬金￬王￬金￬銀￬桂￬香
   ￬飛               ￬角   
￬歩￬歩￬歩￬歩￬歩￬歩￬歩￬歩￬歩
                           
   ￬馬                     
                           
   ￪歩￪歩￪歩￪歩￪歩￪歩￪歩￪歩
                     ￪飛   
   ￪桂￪銀￪金￪王￪金￪銀￪桂￪香
------------------
歩",
        );
        assert_eq_board(
            &board.reversed(),
            "
歩
------------------
￬香￬桂￬銀￬金￬王￬金￬銀￬桂   
   ￬飛                     
￬歩￬歩￬歩￬歩￬歩￬歩￬歩￬歩   
                           
                     ￪馬   
                           
￪歩￪歩￪歩￪歩￪歩￪歩￪歩￪歩￪歩
   ￪角               ￪飛   
￪香￪桂￪銀￪金￪王￪金￪銀￪桂￪香
------------------
香",
        )
    }

    #[test]
    fn test_parse() {
        let str = "香x2 金
------------------
￬香￬桂￬銀   ￬王￬金￬銀￬桂   
   ￬飛               ￬角   
￬歩￬歩￬歩￬歩￬歩￬歩￬歩￬歩￬歩
                           
   ￬馬                     
                           
   ￪歩￪歩￪歩￪歩￪歩￪歩￪歩￪歩
                     ￪飛   
   ￪桂￪銀￪金￪王￪金￪銀￪桂￪香
------------------
歩";
        let board = Board::parse(str);
        let mut output = String::new();
        board.dump_to(&mut output, false).unwrap();
        assert_eq!(output.to_string(), str);
    }
}
