use super::Position;
use crate::{Board, NextBoardKind, Result};
use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

pub(crate) type Key = u64;
pub(crate) type Store = HashMap<Key, BoardNode>;

#[derive(Debug)]
pub(crate) struct BoardNode {
    pub(crate) key: Key,
    pub(crate) board: Board,
    child_keys: Option<Vec<(Key, NextBoardKind)>>,
}

impl BoardNode {
    pub(crate) fn get<'store>(store: &'store Store, key: Key) -> &'store BoardNode {
        store.get(&key).unwrap()
    }

    pub(crate) fn get_or_insert<'store>(
        store: &'store mut Store,
        board: Board,
    ) -> &'store BoardNode {
        let mut hasher = DefaultHasher::new();
        board.hash(&mut hasher);
        let key = hasher.finish();

        store.entry(key).or_insert(BoardNode::new(key, board))
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

    pub(crate) fn get_child_nodes<'store>(
        store: &'store mut Store,
        next_position: Position,
        key: Key,
    ) -> Result<Vec<(&'store BoardNode, NextBoardKind)>> {
        let node = store.get(&key).unwrap();
        match &node.child_keys {
            Some(child_keys) => Ok(child_keys
                .into_iter()
                .map(|(key, next_board_kind)| (store.get(key).unwrap(), *next_board_kind))
                .collect()),
            None => {
                let child_boards = node.board.reversed().create_all_next_boards()?;
                let mut child_keys = Vec::new();
                for (board, next_board_kind) in child_boards {
                    if !Self::is_valid_board(&board, next_position) {
                        continue;
                    }
                    let mut hasher = DefaultHasher::new();
                    board.hash(&mut hasher);
                    let key = hasher.finish();
                    store
                        .entry(key)
                        .or_insert_with(|| BoardNode::new(key, board));
                    child_keys.push((key, next_board_kind));
                }

                store.get_mut(&key).unwrap().child_keys = Some(child_keys.clone());

                Ok(child_keys
                    .into_iter()
                    .map(|(key, next_board_kind)| (store.get(&key).unwrap(), next_board_kind))
                    .collect())
            }
        }
    }
}
