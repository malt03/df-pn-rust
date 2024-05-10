mod force_not_checkmate_node;
mod multi_set;
mod normal_node;
mod pndn;

use crate::{
    common::{shared::board::Board, Rule},
    Result,
};
use force_not_checkmate_node::ForceNotCheckmateNode;
use multi_set::*;
pub(super) use normal_node::NormalNode;
use pndn::*;
use std::collections::HashSet;

pub(super) enum Node<R: Rule> {
    ForceNotCheckmate(ForceNotCheckmateNode),
    Normal(NormalNode<R>),
}

impl<R: Rule> Node<R> {
    pub(super) fn calc_pndn(&mut self, history: &HashSet<&Board<R>>) -> Result<R, ()> {
        match self {
            Node::ForceNotCheckmate(_) => Ok(()),
            Node::Normal(node) => node.calc_pndn(history),
        }
    }

    pub(super) fn pndn(&self) -> &PnDn {
        match self {
            Node::ForceNotCheckmate(node) => &node.pndn,
            Node::Normal(node) => &node.pndn,
        }
    }

    pub(crate) fn board(self) -> Option<Board<R>> {
        match self {
            Node::ForceNotCheckmate(_) => None,
            Node::Normal(node) => Some(node.board),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn dump_best_board(&self) {
        match self {
            Node::ForceNotCheckmate(_) => {}
            Node::Normal(node) => node.dump_best_board(),
        }
    }
}

impl<R: Rule> MultiSetValue for Node<R> {
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
