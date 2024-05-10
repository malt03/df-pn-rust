use crate::Board;

#[derive(Debug)]
pub(crate) enum Error {
    CatchKing(Board),
}
