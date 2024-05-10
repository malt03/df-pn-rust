use super::{PnDn, Position};

pub(crate) struct ForceNotCheckmateNode {
    pub(super) pndn: PnDn,
}

impl ForceNotCheckmateNode {
    pub(super) fn new(position: Position) -> Self {
        Self {
            pndn: match position {
                Position::Offense => PnDn {
                    pn: u32::MAX,
                    dn: 0,
                },
                Position::Defense => PnDn {
                    pn: 0,
                    dn: u32::MAX,
                },
            },
        }
    }
}
