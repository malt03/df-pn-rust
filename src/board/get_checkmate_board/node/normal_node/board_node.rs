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
    pub(crate) is_checking: bool,
    pub(crate) is_checked: bool,
    child_keys: Option<Vec<(Key, NextBoardKind)>>,
}

// let mut hasher = std::collections::hash_map::DefaultHasher::new();
//         board.hash(&mut hasher);
//         let key = hasher.finish();

impl BoardNode {
    pub(crate) fn is_valid(&self, position: Position) -> bool {
        match position {
            Position::Offense => !self.is_checked,
            Position::Defense => !self.is_checked && self.is_checking,
        }
    }

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
            is_checking: board.is_checking(),
            is_checked: board.is_checked(),
            board,
            child_keys: None,
        }
    }

    pub(crate) fn get_child_nodes<'store>(
        store: &'store mut Store,
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

    // pub(crate) fn get_children<'a>(
    //     key: Key,
    //     store: &'a mut HashMap<Key, BoardNode>,
    // ) -> Result<Vec<(&'a BoardNode, NextBoardKind)>> {
    //     match &self.child_keys {
    //         Some(child_keys) => Ok(child_keys
    //             .iter()
    //             .map(|(key, next_board_kind)| (store.get(key).unwrap(), *next_board_kind))
    //             .collect()),
    //         None => {
    //             let child_boards = self.board.create_all_next_boards()?;
    //             for (board, next_board_kind) in &child_boards {
    //                 let mut hasher = std::collections::hash_map::DefaultHasher::new();
    //                 board.hash(&mut hasher);
    //                 let key = hasher.finish();
    //                 let node = store
    //                     .entry(key)
    //                     .or_insert(BoardNode::new(key, board.clone()));
    //             }

    //             unimplemented!()
    //             // &self.child_keys.as_ref().unwrap()
    //         }
    //     }
    // }
}
