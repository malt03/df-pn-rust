use std::ops::Index;

use crate::{Coord, PieceKind};
use once_cell::sync::Lazy;

pub(crate) struct Specific {
    normal: Vec<Coord>,
    changed: Vec<Coord>,
}
impl Specific {
    fn new(normal: Vec<Coord>, changed: Vec<Coord>) -> Specific {
        Specific { normal, changed }
    }
}
impl Index<bool> for Specific {
    type Output = Vec<Coord>;

    fn index(&self, index: bool) -> &Self::Output {
        if index {
            &self.changed
        } else {
            &self.normal
        }
    }
}

pub(crate) struct ControlMap([Specific; 8]);

pub(crate) static CONTROL_MAP: Lazy<ControlMap> = Lazy::new(|| {
    let kin = vec![
        Coord::new(-1, -1),
        Coord::new(0, -1),
        Coord::new(1, -1),
        Coord::new(-1, 0),
        Coord::new(1, 0),
        Coord::new(0, 1),
    ];
    let fu = Specific::new(vec![Coord::new(0, -1)], kin.clone());
    let kyousha = Specific::new(vec![], kin.clone());
    let keima = Specific::new(vec![Coord::new(-1, -2), Coord::new(1, -2)], kin.clone());
    let gin = Specific::new(
        vec![
            Coord::new(-1, -1),
            Coord::new(0, -1),
            Coord::new(1, -1),
            Coord::new(-1, 1),
            Coord::new(1, 1),
        ],
        kin.clone(),
    );
    let kin = Specific::new(kin, vec![]);
    let kaku = Specific::new(
        vec![],
        vec![
            Coord::new(-1, 0),
            Coord::new(1, 0),
            Coord::new(0, -1),
            Coord::new(0, 1),
        ],
    );
    let hisha = Specific::new(
        vec![],
        vec![
            Coord::new(-1, -1),
            Coord::new(1, -1),
            Coord::new(-1, 1),
            Coord::new(1, 1),
        ],
    );
    let king = Specific::new(
        vec![
            Coord::new(-1, -1),
            Coord::new(0, -1),
            Coord::new(1, -1),
            Coord::new(-1, 0),
            Coord::new(1, 0),
            Coord::new(-1, 1),
            Coord::new(0, 1),
            Coord::new(1, 1),
        ],
        vec![],
    );
    ControlMap([fu, kyousha, keima, gin, kin, kaku, hisha, king])
});

impl Index<PieceKind> for ControlMap {
    type Output = Specific;

    fn index(&self, index: PieceKind) -> &Self::Output {
        &self.0[index as usize]
    }
}

static KAKU_VECTORS: Lazy<[Coord; 4]> = Lazy::new(|| {
    [
        Coord::new(1, 1),
        Coord::new(1, -1),
        Coord::new(-1, 1),
        Coord::new(-1, -1),
    ]
});
static HISHA_VECTORS: Lazy<[Coord; 4]> = Lazy::new(|| {
    [
        Coord::new(1, 0),
        Coord::new(-1, 0),
        Coord::new(0, 1),
        Coord::new(0, -1),
    ]
});
static KYOUSHA_VECTORS: Lazy<[Coord; 1]> = Lazy::new(|| [Coord::new(0, -1)]);
pub(crate) fn get_vectors(kind: PieceKind) -> &'static [Coord] {
    match kind {
        PieceKind::Kaku => &KAKU_VECTORS.as_ref(),
        PieceKind::Hisha => &HISHA_VECTORS.as_ref(),
        PieceKind::Kyousha => &KYOUSHA_VECTORS.as_ref(),
        _ => panic!("Invalid kind: {:?}", kind),
    }
}
