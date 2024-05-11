use super::{Board, ForceNotCheckmateNode, MultiSet, Node, PnDn, Position};
use crate::{Error, NextBoardKind};
use core::panic;
use std::collections::HashSet;
use Position::*;

pub(crate) struct NormalNode {
    pub(crate) board: Board,
    is_checking: bool,
    is_checked: bool,
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
        next_boards: Vec<(Board, NextBoardKind)>,
        history: &HashSet<&Board>,
        max_depth: Option<usize>,
    ) {
        let next_position = self.position.reversed();

        if let Some(max_depth) = max_depth {
            if history.len() == max_depth {
                self.children
                    .push_back(Node::ForceNotCheckmate(ForceNotCheckmateNode::new(
                        next_position,
                    )));
                return;
            }
        }

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

impl NormalNode {
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

    pub(crate) fn new(
        board: Board,
        position: Position,
        next_board_kind: NextBoardKind,
    ) -> NormalNode {
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

    pub(crate) fn calc_pndn(&mut self, history: &HashSet<&Board>, max_depth: Option<usize>) {
        let mut copied_history = history.clone();
        copied_history.insert(&self.board);
        if self.props.is_children_expanded {
            let Some(mut best) = self.props.children.pop_front() else {
                return;
            };
            best.calc_pndn(&copied_history, max_depth);
            self.props.children.push_back(best);
        } else {
            match self.board.reversed().create_all_next_boards() {
                Ok(next_boards) => {
                    self.props
                        .expand_children(next_boards, &copied_history, max_depth);
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

    pub(crate) fn dump_best_board(&self, is_checkmate: bool) {
        match self.props.position {
            Position::Offense => {
                println!(
                    "{}\n=================================",
                    self.board.reversed()
                );
                if is_checkmate {
                    self.props.children.peak_front().map(|node| {
                        node.dump_best_board(is_checkmate);
                    });
                } else {
                    for node in self.props.children.iter() {
                        node.dump_best_board(is_checkmate);
                    }
                }
            }
            Position::Defense => {
                println!("{}\n=================================", self.board);
                if is_checkmate {
                    for node in self.props.children.iter() {
                        node.dump_best_board(is_checkmate);
                    }
                } else {
                    self.props.children.peak_front().map(|node| {
                        node.dump_best_board(is_checkmate);
                    });
                }
            }
        };
    }

    pub(crate) fn best_boards(mut self) -> Vec<Board> {
        let Some(best_nodes) = self.children().pop_same_key_fronts() else {
            return vec![self.board];
        };
        let mut best_boards_vec: Vec<_> = best_nodes.into_iter().map(|n| n.best_boards()).collect();
        best_boards_vec.sort_unstable_by_key(|h| h.len());
        let mut best_boards = match self.props.position {
            Offense => best_boards_vec.swap_remove(0),
            Defense => best_boards_vec.pop().unwrap(),
        };
        best_boards.push(self.board);
        best_boards
    }
}
