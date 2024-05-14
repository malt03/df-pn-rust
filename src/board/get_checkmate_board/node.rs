mod force_not_checkmate_node;
mod multi_set;
mod normal_node;
mod pndn;

use crate::Board;
use force_not_checkmate_node::ForceNotCheckmateNode;
use multi_set::*;
pub(super) use normal_node::{Key, NormalNode, Store};
use pndn::*;
use std::collections::HashSet;

pub(super) enum Node {
    ForceNotCheckmate(ForceNotCheckmateNode),
    Normal(NormalNode),
}

impl Node {
    pub(super) fn calc_pndn(
        &mut self,
        store: &mut Store,
        history: &HashSet<Key>,
        max_depth: Option<usize>,
    ) {
        match self {
            Node::ForceNotCheckmate(_) => {}
            Node::Normal(node) => node.calc_pndn(store, history, max_depth),
        }
    }

    pub(super) fn pndn(&self) -> &PnDn {
        match self {
            Node::ForceNotCheckmate(node) => &node.pndn,
            Node::Normal(node) => &node.pndn,
        }
    }

    pub(crate) fn best_boards(self, store: &Store) -> Vec<Board> {
        match self {
            Node::ForceNotCheckmate(_) => Vec::new(),
            Node::Normal(node) => node.best_boards(store),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn board<'store>(&self, store: &'store Store) -> &'store Board {
        match self {
            Node::ForceNotCheckmate(_) => panic!("ForceNotCheckmateNode has no board"),
            Node::Normal(node) => &node.board(store),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn children(&mut self) -> &mut MultiSet<Node> {
        match self {
            Node::ForceNotCheckmate(_) => panic!("ForceNotCheckmateNode has no children"),
            Node::Normal(node) => node.children(),
        }
    }

    pub(crate) fn dump_single_best_board(&self, store: &Store) {
        match self {
            Node::ForceNotCheckmate(_) => {}
            Node::Normal(node) => node.dump_single_best_board(store),
        }
    }
}

impl MultiSetValue for Node {
    type MultiSetOrderValue = u32;

    fn multi_set_order_value(&self) -> Self::MultiSetOrderValue {
        self.pndn().dn
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Position {
    Offense,
    Defense,
}

impl Position {
    fn reversed(&self) -> Position {
        match self {
            Position::Offense => Position::Defense,
            Position::Defense => Position::Offense,
        }
    }
}
