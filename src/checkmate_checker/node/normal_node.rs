use super::{Board, ForceNotCheckmateNode, MultiSet, Node, PnDn, Position};
use crate::{
    common::{shared::board::NextBoardKind, Rule},
    Result,
};
use std::collections::HashSet;

pub(crate) struct NormalNode<R: Rule> {
    pub(crate) board: Board<R>,
    is_checking: bool,
    is_checked: bool,
    pub(crate) pndn: PnDn,
    props: Props<R>,
    next_board_kind: NextBoardKind,
}

struct Props<R: Rule> {
    position: Position,
    is_children_expanded: bool,
    children: MultiSet<Node<R>>,
}

impl<R: Rule> Props<R> {
    fn expand_children(
        &mut self,
        next_boards: Vec<(Board<R>, NextBoardKind)>,
        history: &HashSet<&Board<R>>,
    ) {
        let next_position = self.position.reversed();

        for (next_board, next_board_kind) in next_boards {
            if history.contains(&next_board) {
                self.children
                    .push_back(Node::ForceNotCheckmate(ForceNotCheckmateNode::new(
                        next_position,
                    )));
                continue;
            }

            let node = NormalNode::new(next_board, next_position, next_board_kind);
            if !node.is_valid() {
                continue;
            }
            self.children.push_back(Node::Normal(node));
        }

        self.is_children_expanded = true;
    }
}

impl<R: Rule> NormalNode<R> {
    pub(crate) fn children(&mut self) -> &mut MultiSet<Node<R>> {
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

    pub(crate) fn new(
        board: Board<R>,
        position: Position,
        next_board_kind: NextBoardKind,
    ) -> NormalNode<R> {
        NormalNode {
            is_checking: board.is_checking(),
            is_checked: board.is_checked(),
            board,
            pndn: PnDn::new(1, 1),
            next_board_kind,
            props: Props {
                position,
                children: MultiSet::new(),
                is_children_expanded: false,
            },
        }
    }

    fn is_valid(&self) -> bool {
        match self.props.position {
            Position::Offense => !self.is_checked,
            Position::Defense => !self.is_checked && self.is_checking,
        }
    }

    pub(crate) fn calc_pndn(&mut self, history: &HashSet<&Board<R>>) -> Result<R, ()> {
        let mut copied_history = history.clone();
        copied_history.insert(&self.board);
        if self.props.is_children_expanded {
            let Some(mut best) = self.props.children.pop_front() else {
                return Ok(());
            };
            best.calc_pndn(&copied_history)?;
            self.props.children.push_back(best);
        } else {
            self.props.expand_children(
                self.board.reversed().create_all_next_boards()?,
                &copied_history,
            );
        }
        self.reload_pndn();

        Ok(())
    }

    pub(crate) fn dump_best_board(&self) {
        match self.props.position {
            Position::Offense => {
                println!(
                    "{}\n=================================",
                    self.board.reversed()
                );
                self.props.children.peak_front().map(|node| {
                    node.dump_best_board();
                });
            }
            Position::Defense => {
                println!("{}\n=================================", self.board);
                for node in self.props.children.iter() {
                    node.dump_best_board();
                }
            }
        };
    }
}
