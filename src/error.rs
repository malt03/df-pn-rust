use crate::Board;

#[derive(Debug)]
pub enum Error {
    CatchKing(Board),
}
