use enum_map::EnumMap;
use std::fmt;

#[derive(Debug, Enum, Copy, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Stat {
    Arms,
    Block,
    Swing,
    Thrust,

    Face,
    Charm,
    Impose,
    Read,

    Hands,
    Aim,
    Craft,
    Control,

    Legs,
    Balance,
    Power,
    Dodge,

    Mind,
    Language,
    Lore,
    Observe,

    Magic,
    Luck,
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub const CORE_STATS: [Stat; 5] = [Stat::Arms, Stat::Face, Stat::Hands, Stat::Legs, Stat::Mind];

impl Stat {
    pub fn subskills(&self) -> Vec<Stat> {
        match self {
            Stat::Arms => vec![Stat::Block, Stat::Swing, Stat::Thrust],
            Stat::Face => vec![Stat::Charm, Stat::Impose, Stat::Read],
            Stat::Hands => vec![Stat::Aim, Stat::Craft, Stat::Control],
            Stat::Legs => vec![Stat::Balance, Stat::Power, Stat::Dodge],
            Stat::Mind => vec![Stat::Language, Stat::Lore, Stat::Observe],
            _ => vec![],
        }
    }
}

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Character {
    pub stats: EnumMap<Stat, Option<i8>>,
    pub name: String,
    pub traits: Vec<String>,
}
