#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Kind {
    Fu,
    Kyousha,
    Keima,
    Gin,
    Kin,
    Kaku,
    Hisha,
    King,
}

use Kind::*;

const ALL_KIND: [Kind; 8] = [Fu, Kyousha, Keima, Gin, Kin, Kaku, Hisha, King];

impl Kind {
    pub(crate) fn iter() -> impl Iterator<Item = Kind> {
        ALL_KIND.iter().copied()
    }

    pub(crate) fn is_changable(&self) -> bool {
        match self {
            Kin | King => false,
            _ => true,
        }
    }

    pub(crate) fn is_force_change(&self) -> bool {
        match self {
            Fu | Kyousha | Keima | Hisha | Kaku => true,
            _ => false,
        }
    }

    pub(crate) fn title(&self, is_changed: bool) -> &'static str {
        if is_changed {
            match self {
                Fu => "と",
                Kyousha => "杏",
                Keima => "圭",
                Gin => "全",
                Kin => panic!("成金"),
                Kaku => "馬",
                Hisha => "龍",
                King => panic!("成王"),
            }
        } else {
            match self {
                Fu => "歩",
                Kyousha => "香",
                Keima => "桂",
                Gin => "銀",
                Kin => "金",
                Kaku => "角",
                Hisha => "飛",
                King => "王",
            }
        }
    }
}

impl From<usize> for Kind {
    fn from(index: usize) -> Self {
        ALL_KIND[index]
    }
}
