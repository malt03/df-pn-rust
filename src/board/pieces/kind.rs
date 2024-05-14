use bincode::{Decode, Encode};
use std::collections::HashMap;
use Kind::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Encode, Decode)]
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

const ALL_KIND: [Kind; 8] = [Fu, Kyousha, Keima, Gin, Kin, Kaku, Hisha, King];

impl Kind {
    pub(crate) fn safe_parse(c: char) -> Option<(Kind, bool)> {
        match c {
            '歩' => Some((Fu, false)),
            '香' => Some((Kyousha, false)),
            '桂' => Some((Keima, false)),
            '銀' => Some((Gin, false)),
            '金' => Some((Kin, false)),
            '角' => Some((Kaku, false)),
            '飛' => Some((Hisha, false)),
            '王' => Some((King, false)),
            'と' => Some((Fu, true)),
            '杏' => Some((Kyousha, true)),
            '圭' => Some((Keima, true)),
            '全' => Some((Gin, true)),
            '馬' => Some((Kaku, true)),
            '龍' => Some((Hisha, true)),
            _ => None,
        }
    }

    pub(crate) fn parse(c: char) -> (Kind, bool) {
        Kind::safe_parse(c).expect(format!("invalid char: {c}").as_str())
    }

    pub(crate) fn parse_hands<S>(s: S) -> HashMap<Kind, u8>
    where
        S: AsRef<str>,
    {
        let mut map = HashMap::new();
        for hand_str in s.as_ref().split(' ') {
            let mut chars = hand_str.chars();
            let Some(c) = chars.next() else { continue };
            let (kind, is_changed) = Kind::parse(c);
            if is_changed {
                panic!("invalid hand: {}", c);
            }

            let n = {
                if let Some(c) = chars.next() {
                    if c != 'x' {
                        panic!("needs `x` before number for {kind} got {c}");
                    }
                    let n = chars.as_str();
                    n.parse().expect(format!("invalid number {n}").as_str())
                } else {
                    1
                }
            };
            *map.entry(kind).or_insert(0) += n;
        }
        map
    }

    pub(crate) fn iter() -> impl Iterator<Item = Kind> {
        ALL_KIND.iter().copied()
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

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title(false))
    }
}

impl From<usize> for Kind {
    fn from(index: usize) -> Self {
        ALL_KIND[index]
    }
}
