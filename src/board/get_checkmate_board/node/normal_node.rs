mod board_node;

use super::{Board, ForceNotCheckmateNode, MultiSet, Node, PnDn, Position};
use crate::{Error, NextBoardKind};
use board_node::BoardNode;
pub(crate) use board_node::{Key, Store};
use core::panic;
use std::collections::HashSet;
use Position::*;

pub(crate) struct NormalNode {
    pub(crate) key: Key,
    pub(crate) pndn: PnDn,
    props: Props,
    next_board_kind: NextBoardKind,
}

struct Props {
    position: Position,
    is_children_expanded: bool,
    children: MultiSet<Node>,
}

impl Props {
    fn expand_children(
        &mut self,
        next_nodes: Vec<(&BoardNode, NextBoardKind)>,
        next_position: Position,
        history: &HashSet<Key>,
        max_depth: Option<usize>,
    ) {
        if let Some(max_depth) = max_depth {
            if history.len() == max_depth {
                self.children
                    .push_back(Node::ForceNotCheckmate(ForceNotCheckmateNode::new(
                        next_position,
                    )));
                return;
            }
        }

        for (next_node, next_board_kind) in next_nodes {
            if history.contains(&next_node.key) {
                self.children
                    .push_back(Node::ForceNotCheckmate(ForceNotCheckmateNode::new(
                        next_position,
                    )));
                continue;
            }

            let node = NormalNode::new_with_node(next_node.key, next_position, next_board_kind);
            self.children.push_back(Node::Normal(node));
        }

        self.is_children_expanded = true;
    }
}

impl NormalNode {
    pub(crate) fn board<'store>(&self, store: &'store Store) -> &'store Board {
        &BoardNode::get(store, self.key).board
    }

    pub(crate) fn children(&mut self) -> &mut MultiSet<Node> {
        &mut self.props.children
    }

    fn reload_pndn(&mut self) {
        if self.props.children.is_empty()
            && self.props.position == Position::Defense
            && self.next_board_kind == NextBoardKind::Uchifu
        {
            self.pndn = PnDn::new(0, u32::MAX);
            return;
        }

        self.pndn = PnDn::new(u32::MAX, 0);
        for child in self.props.children.iter() {
            self.pndn.update_reversed(child.pndn());
        }
    }

    pub(crate) fn new(store: &mut Store, board: Board) -> NormalNode {
        let node = BoardNode::get_or_insert(store, board);
        NormalNode::new_with_node(node.key, Offense, NextBoardKind::Normal)
    }

    pub(crate) fn new_with_node(
        board_node_key: Key,
        position: Position,
        next_board_kind: NextBoardKind,
    ) -> NormalNode {
        NormalNode {
            key: board_node_key,
            pndn: PnDn::new(1, 1),
            next_board_kind,
            props: Props {
                position,
                children: MultiSet::new(),
                is_children_expanded: false,
            },
        }
    }

    pub(crate) fn calc_pndn(
        &mut self,
        store: &mut Store,
        history: &HashSet<Key>,
        max_depth: Option<usize>,
    ) {
        let mut copied_history = history.clone();
        copied_history.insert(self.key);
        if self.props.is_children_expanded {
            let Some(mut best) = self.props.children.pop_front() else {
                return;
            };
            best.calc_pndn(store, &copied_history, max_depth);
            self.props.children.push_back(best);
        } else {
            let next_position = self.props.position.reversed();

            match BoardNode::get_child_nodes(store, next_position, self.key) {
                Ok(child_nodes) => {
                    self.props.expand_children(
                        child_nodes,
                        next_position,
                        &copied_history,
                        max_depth,
                    );
                }
                Err(e) => match e {
                    Error::CatchKing(board) => {
                        for history in copied_history.iter() {
                            println!("{}\n===========================", history);
                        }
                        println!("{}", board);
                        panic!("unexpected catch king");
                    }
                },
            }
        }
        self.reload_pndn();
    }

    pub(crate) fn dump_single_best_board(&self, store: &Store) {
        match self.props.position {
            Position::Offense => {
                println!(
                    "{}\n=================================",
                    self.board(store).reversed()
                );
            }
            Position::Defense => {
                println!("{}\n=================================", self.board(store));
            }
        }
        self.props.children.peak_front().map(|node| {
            node.dump_single_best_board(store);
        });
    }

    pub(crate) fn best_boards(mut self, store: &Store) -> Vec<Board> {
        let Some(best_nodes) = self.children().pop_same_key_fronts() else {
            return vec![self.board(store).clone()];
        };
        let mut best_boards_vec: Vec<_> = best_nodes
            .into_iter()
            .map(|n| n.best_boards(store))
            .collect();
        best_boards_vec.sort_unstable_by_key(|h| h.len());
        let mut best_boards = match self.props.position {
            Offense => best_boards_vec.swap_remove(0),
            Defense => best_boards_vec.pop().unwrap(),
        };
        best_boards.push(self.board(store).clone());
        best_boards
    }
}
