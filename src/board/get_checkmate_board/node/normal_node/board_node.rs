use super::Position;
use crate::{
    db::{get_entity, put_entity, Entity, Key, DB},
    Board, NextBoardKind, Result,
};
use bincode::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
pub(crate) struct BoardNode {
    pub(crate) key: Key,
    pub(crate) board: Board,
    child_keys: Option<Vec<(Key, NextBoardKind)>>,
}

impl Entity for BoardNode {
    fn get_key(&self) -> Key {
        self.key
    }
}

impl BoardNode {
    pub(crate) fn get(db: &DB, key: &Key) -> BoardNode {
        get_entity::<BoardNode>(db, &key).unwrap()
    }

    pub(crate) fn get_or_insert(db: &DB, board: Board) -> BoardNode {
        let key = board.key();
        match get_entity(db, &key) {
            Some(node) => node,
            None => {
                let node = BoardNode::new(key, board);
                put_entity(db, &node);
                node
            }
        }
    }

    fn new(key: Key, board: Board) -> Self {
        Self {
            key,
            board,
            child_keys: None,
        }
    }

    fn is_valid_board(board: &Board, position: Position) -> bool {
        match position {
            Position::Offense => !board.is_checked(),
            Position::Defense => !board.is_checked() && board.is_checking(),
        }
    }

    pub(crate) fn get_child_nodes(
        db: &DB,
        next_position: Position,
        key: &Key,
    ) -> Result<Vec<(BoardNode, NextBoardKind)>> {
        let mut node = get_entity::<BoardNode>(db, key).unwrap();
        match &node.child_keys {
            Some(child_keys) => Ok(child_keys
                .into_iter()
                .map(|(key, next_board_kind)| (get_entity(db, key).unwrap(), *next_board_kind))
                .collect()),
            None => {
                let child_boards = node.board.reversed().create_all_next_boards()?;
                let mut child_keys = Vec::new();
                for (board, next_board_kind) in child_boards {
                    if !Self::is_valid_board(&board, next_position) {
                        continue;
                    }
                    let node = Self::get_or_insert(db, board);
                    child_keys.push((node.key, next_board_kind));
                }

                node.child_keys = Some(child_keys.clone());
                put_entity(db, &node);

                Ok(child_keys
                    .into_iter()
                    .map(|(key, next_board_kind)| (get_entity(db, &key).unwrap(), next_board_kind))
                    .collect())
            }
        }
    }
}
