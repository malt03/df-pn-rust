mod force_not_checkmate_node;
mod multi_set;
mod normal_node;
mod pndn;

use crate::{
    db::{Key, DB},
    Board,
};
use force_not_checkmate_node::ForceNotCheckmateNode;
use multi_set::*;
pub(super) use normal_node::NormalNode;
use pndn::*;
use std::collections::HashSet;

pub(super) enum Node {
    ForceNotCheckmate(ForceNotCheckmateNode),
    Normal(NormalNode),
}

impl Node {
    pub(super) fn calc_pndn(&mut self, db: &DB, history: &HashSet<Key>, max_depth: Option<usize>) {
        match self {
            Node::ForceNotCheckmate(_) => {}
            Node::Normal(node) => node.calc_pndn(db, history, max_depth),
        }
    }

    pub(super) fn pndn(&self) -> &PnDn {
        match self {
            Node::ForceNotCheckmate(node) => &node.pndn,
            Node::Normal(node) => &node.pndn,
        }
    }

    pub(crate) fn best_boards(self, db: &DB) -> Vec<Board> {
        match self {
            Node::ForceNotCheckmate(_) => Vec::new(),
            Node::Normal(node) => node.best_boards(db),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn board(&self, db: &DB) -> Board {
        match self {
            Node::ForceNotCheckmate(_) => panic!("ForceNotCheckmateNode has no board"),
            Node::Normal(node) => node.board(db),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn children(&mut self) -> &mut MultiSet<Node> {
        match self {
            Node::ForceNotCheckmate(_) => panic!("ForceNotCheckmateNode has no children"),
            Node::Normal(node) => node.children(),
        }
    }

    pub(crate) fn dump_single_best_board(&self, db: &DB) {
        match self {
            Node::ForceNotCheckmate(_) => {}
            Node::Normal(node) => node.dump_single_best_board(db),
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
