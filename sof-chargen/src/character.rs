use crate::character::CheckResult::{
    CriticalFailure, CriticalSuccess, ExtremeSuccess, Failure, HardSuccess, Success,
};
use crate::data::careers::{Affiliation, Career};
use crate::data::locations::{Culture, Faith, Location};
use crate::data::perks::Perks;
use crate::event::stages::LifeStage;
use enum_map::EnumMap;
use std::fmt;
use std::fmt::{Display, Formatter};

// spans the numbers 0..=100
// in some cases only 1..=100 are valid, though
type DiceT = i8;

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

    // non-skill stats
    Magic,
    Luck,
    Stamina,
    Speed,
}

impl Display for Stat {
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

#[derive(Debug, Copy, Clone, serde::Deserialize, serde::Serialize)]
pub enum BirthOmen {
    ProsperousConstellations,
    PropheticSigns(usize),
    PracticallyMinded,
    ShootingStar,
    PortentsOfDoom,
}

pub const BIRTH_OMENS: [BirthOmen; 5] = [
    BirthOmen::ProsperousConstellations,
    BirthOmen::PropheticSigns(2), // stars with two charges
    BirthOmen::PracticallyMinded,
    BirthOmen::ShootingStar,
    BirthOmen::PortentsOfDoom,
];

impl Display for BirthOmen {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BirthOmen::ProsperousConstellations => write!(f, "Prosperous Constellations"),
            BirthOmen::PropheticSigns(0) => write!(f, "Prophetic Signs"),
            BirthOmen::PropheticSigns(uses) => write!(f, "Prophetic Signs (uses: {})", uses),
            BirthOmen::PracticallyMinded => write!(f, "Practically-Minded"),
            BirthOmen::ShootingStar => write!(f, "Shooting Star"),
            BirthOmen::PortentsOfDoom => write!(f, "Portents of Doom"),
        }
    }
}

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct Character {
    pub stats: EnumMap<Stat, Option<DiceT>>,
    pub name: String,
    pub traits: Vec<String>,
    pub omen: Option<BirthOmen>,
    pub perks: Perks,
    pub birth_location: Option<Location>,

    pub culture: Option<Culture>,
    pub faith: Option<Faith>,

    pub affiliation: Option<Affiliation>,
    pub parents_career: Option<Career>,
    pub careers: Vec<Career>, // you can have up to 4 careers i think
    pub rank: Option<DiceT>,
    pub life_stage: LifeStage,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum CheckResult {
    CriticalFailure = 0,
    Failure = 1,
    Success = 2,
    HardSuccess = 3,
    ExtremeSuccess = 4,
    CriticalSuccess = 5,
}

fn check(to_beat: u64, roll: u64) -> CheckResult {
    match roll {
        1..=5 => CriticalSuccess,
        96..=100 => CriticalFailure,
        _ if roll * 4 <= to_beat => ExtremeSuccess,
        _ if roll * 2 <= to_beat => HardSuccess,
        _ if roll <= to_beat => Success,
        _ => Failure,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::dice::DiceRoll;
    #[test]
    fn test_check() {
        assert_eq!(check(50, 2), CriticalSuccess);
        assert_eq!(check(50, 5), CriticalSuccess);
        assert_eq!(check(50, 96), CriticalFailure);
        assert_eq!(check(50, 100), CriticalFailure);
        assert_eq!(check(50, 50), Success);
        assert_eq!(check(50, 51), Failure);
        assert_eq!(check(50, 26), Success);
        assert_eq!(check(50, 25), HardSuccess);
        assert_eq!(check(50, 13), HardSuccess);
        assert_eq!(check(50, 12), ExtremeSuccess);
        assert_ne!(check(100, crate::roll!(1 d 100).result() as u64), Failure);
    }
}
