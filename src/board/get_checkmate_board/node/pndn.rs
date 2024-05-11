#[derive(Clone, Copy, Debug)]
pub(crate) struct PnDn {
    pub(crate) pn: u32,
    pub(crate) dn: u32,
}

impl PnDn {
    pub(crate) fn new(pn: u32, dn: u32) -> Self {
        PnDn { pn, dn }
    }

    pub(crate) fn update_reversed(&mut self, other: &PnDn) {
        if other.dn < self.pn {
            self.pn = other.dn;
        }
        self.dn = self.dn.saturating_add(other.pn);
    }
}
