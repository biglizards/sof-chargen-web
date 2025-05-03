use crate::Stat;
use crate::data::careers::Affiliation::*;
use crate::data::locations::{CareerTable, Culture, Demographic, Faith, Location};
use crate::data::perks::Perk;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub enum Affiliation {
    Slumfolk,
    Criminals,
    Vagabonds,
    Peasantry,
    Performers,
    Plebeians,
    Watch,
    Army,
    Scholars,
    Gentry,
}

impl Display for Affiliation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) // all affiliations are one word, so debug and display are the same
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CareerClass {
    Artisan,
    Beggar,
    ConArtist,
    Detective,
    Entertainer,
    Farmer,
    Guard,
    Hunter,
    Infantry,
    Knight,
    Labourer,
    Mariner,
    Noble,
    Official,
    Physician,
    Rogue,
    Scholar,
    Trader,
    Wayfarer,
    Zealot,
}

impl CareerClass {
    fn letter(&self) -> char {
        match self {
            CareerClass::Artisan => 'A',
            CareerClass::Beggar => 'B',
            CareerClass::ConArtist => 'C',
            CareerClass::Detective => 'D',
            CareerClass::Entertainer => 'E',
            CareerClass::Farmer => 'F',
            CareerClass::Guard => 'G',
            CareerClass::Hunter => 'H',
            CareerClass::Infantry => 'I',
            CareerClass::Knight => 'K',
            CareerClass::Labourer => 'L',
            CareerClass::Mariner => 'M',
            CareerClass::Noble => 'N',
            CareerClass::Official => 'O',
            CareerClass::Physician => 'P',
            CareerClass::Rogue => 'R',
            CareerClass::Scholar => 'S',
            CareerClass::Trader => 'T',
            CareerClass::Wayfarer => 'W',
            CareerClass::Zealot => 'Z',
        }
    }
}

#[derive(Copy, Clone, Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct Career {
    pub name: &'static str,
    pub class: CareerClass,
}

impl Display for Career {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.class.letter())
    }
}

#[derive(Copy, Eq, PartialEq, Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum CareerTableStar {
    None,
    NeedsFaith(Faith),
    NeedsFaithAndCulture(Faith, Culture),
}

#[derive(Clone, Eq, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub enum CareerTableEntry {
    Career(Career, CareerTableStar),
    Careers((Career, CareerTableStar), (Career, CareerTableStar)),
    RemainAtRank(i8),
    ChangeAffiliation(Affiliation),
    RerollWithDisadvantage,
    Reroll,
}

macro_rules! compute_affiliation {
    (
        $dice: ident,
        $a: expr,
        $b: expr,
        $c: expr,
        $d: expr,
        $e: expr,
        $f: expr,
        $g: expr,
        $h: expr,
        $i: expr
    ) => {
        match $dice {
            $a..=100 => Slumfolk,
            $b..$a => Criminals,
            $c..$b => Vagabonds,
            $d..$c => Peasantry,
            $e..$d => Performers,
            $f..$e => Plebeians,
            $g..$f => Watch,
            $h..$g => Army,
            $i..$h => Scholars,
            1..$i => Gentry,
            _ => unreachable!(),
        }
    };
}

macro_rules! career_class {
    (A) => {
        crate::data::careers::CareerClass::Artisan
    };
    (B) => {
        crate::data::careers::CareerClass::Beggar
    };
    (C) => {
        crate::data::careers::CareerClass::ConArtist
    };
    (D) => {
        crate::data::careers::CareerClass::Detective
    };
    (E) => {
        crate::data::careers::CareerClass::Entertainer
    };
    (F) => {
        crate::data::careers::CareerClass::Farmer
    };
    (G) => {
        crate::data::careers::CareerClass::Guard
    };
    (H) => {
        crate::data::careers::CareerClass::Hunter
    };
    (I) => {
        crate::data::careers::CareerClass::Infantry
    };
    (K) => {
        crate::data::careers::CareerClass::Knight
    };
    (L) => {
        crate::data::careers::CareerClass::Labourer
    };
    (M) => {
        crate::data::careers::CareerClass::Mariner
    };
    (N) => {
        crate::data::careers::CareerClass::Noble
    };
    (O) => {
        crate::data::careers::CareerClass::Official
    };
    (P) => {
        crate::data::careers::CareerClass::Physician
    };
    (R) => {
        crate::data::careers::CareerClass::Rogue
    };
    (S) => {
        crate::data::careers::CareerClass::Scholar
    };
    (T) => {
        crate::data::careers::CareerClass::Trader
    };
    (W) => {
        crate::data::careers::CareerClass::Wayfarer
    };
    (Z) => {
        crate::data::careers::CareerClass::Zealot
    };
}

macro_rules! career_inner {
    ($($name:ident) * ($letter:ident)) => {
        crate::data::careers::Career{name: stringify!($($name) *), class: career_class!($letter)}
    };
}

macro_rules! career {
    ($($name:ident) * ($letter:ident)) => {
        crate::data::careers::CareerTableEntry::Career(
            career_inner!($($name) * ($letter)),
            CareerTableStar::None,
        )
    };
    ($($name1:ident) * ($letter1:ident), $($name2:ident) * ($letter2:ident)) => {
        crate::data::careers::CareerTableEntry::Careers(
            (career_inner!($($name1) * ($letter1)), CareerTableStar::None,),
            (career_inner!($($name2) * ($letter2)), CareerTableStar::None,)
        )
    };
    ($($name1:ident) * ($letter1:ident), *$($name2:ident) * ($letter2:ident) $star:expr) => {
        crate::data::careers::CareerTableEntry::Careers(
            (career_inner!($($name1) * ($letter1)), CareerTableStar::None,),
            (career_inner!($($name2) * ($letter2)), $star,)
        )
    };
    (Join $affiliation:ident) => {
        crate::data::careers::CareerTableEntry::ChangeAffiliation(
            crate::data::careers::Affiliation::$affiliation
        )
    };
    (Remain at rank $rank:literal) => {
        crate::data::careers::CareerTableEntry::RemainAtRank($rank)
    };
    (Reroll affiliation with disadvantage) => {
        crate::data::careers::CareerTableEntry::RerollWithDisadvantage
    };
    (Reroll affiliation) => {
        crate::data::careers::CareerTableEntry::Reroll
    };
}

pub const fn get_affiliation(location: &Location, d100: i8) -> Affiliation {
    match location.career_table {
        CareerTable::ValiantEmpire => match location.demographic {
            Demographic::Urban => compute_affiliation!(d100, 91, 76, 71, 61, 51, 31, 27, 17, 11),
            Demographic::Rural => compute_affiliation!(d100, 91, 86, 76, 46, 41, 31, 27, 17, 11),
            Demographic::Border => compute_affiliation!(d100, 86, 81, 61, 51, 46, 36, 31, 16, 6),
        },
        CareerTable::Nivena => match location.demographic {
            Demographic::Urban => compute_affiliation!(d100, 93, 84, 81, 76, 66, 46, 31, 21, 11),
            Demographic::Rural => compute_affiliation!(d100, 93, 86, 76, 61, 51, 36, 31, 11, 6),
            Demographic::Border => compute_affiliation!(d100, 91, 81, 56, 46, 36, 31, 26, 11, 4),
        },
        CareerTable::Marolaw => match location.demographic {
            Demographic::Urban => compute_affiliation!(d100, 81, 71, 66, 61, 51, 36, 31, 21, 11),
            Demographic::Rural => compute_affiliation!(d100, 81, 76, 71, 61, 46, 36, 26, 11, 6),
            Demographic::Border => compute_affiliation!(d100, 86, 81, 61, 56, 46, 39, 30, 13, 4),
        },
    }
}

pub const fn get_rank(location: &Location, affiliation: Affiliation, d6: i8) -> i8 {
    let d3 = (d6 + 1) / 2; // rounds up so 1,2 -> 1, 3,4 -> 2, etc
    match location.career_table {
        CareerTable::ValiantEmpire => match affiliation {
            Slumfolk => d3,
            Criminals => d3,
            Vagabonds => d3 + 1,
            Peasantry => d3 + 1,
            Performers => d3 + 1,
            Plebeians => d6,
            Watch => d3 + 2,
            Army => d3 + 2,
            Scholars => d6 + 1,
            Gentry => d3 + 3,
        },
        CareerTable::Nivena => match affiliation {
            Slumfolk => d3,
            Criminals => d3,
            Vagabonds => d3 + 1,
            Peasantry => d3 + 1,
            Performers => d3 + 1,
            Plebeians => d3 + 2,
            Watch => d3 + 2,
            Army => d6 + 1,
            Scholars => d3 + 3,
            Gentry => d6 + 2,
        },
        CareerTable::Marolaw => match affiliation {
            Slumfolk => d3 - 1,
            Criminals => d3,
            Vagabonds => d3,
            Peasantry => d3 + 1,
            Performers => d3 + 1,
            Plebeians => d3 + 2,
            Watch => d3 + 2,
            Army => d3 + 3,
            Scholars => d3 + 3,
            Gentry => d3 + 5,
        },
    }
}

pub fn get_careers(location: &Location, affiliation: Affiliation, rank: i8) -> CareerTableEntry {
    match location.career_table {
        CareerTable::ValiantEmpire => match affiliation {
            Slumfolk => match rank {
                0 => career!(Outcast(B)),
                1 => career!(Militia (G), Odd Jobber (L)),
                2 => career!(Militia (G), Odd Jobber (L)),
                3 => career!(Peddler(C), *Dewisetic(Z)
                    CareerTableStar::NeedsFaithAndCulture(Faith::Gytungrug, Culture::Kremish)),
                4..=9 => career!(Join Plebeians),
                _ => unreachable!(),
            },
            Criminals => match rank {
                0 => career!(Thief(R)),
                1 => career!(Thief(R), Bodyguard(G)),
                2 => career!(Sawbones(P), Smuggler(M)),
                3 => career!(Counterfeiter(C), Headhunter(D)),
                4 => career!(Fence(T), Gang Boss(K)),
                5 => career!(Gang Boss(K)),
                6..=9 => career!(Remain at rank 5),
                _ => unreachable!(),
            },
            Vagabonds => match rank {
                0 => career!(Deserter(R), Poacher(H)),
                1 => career!(Deserter(R), Poacher(H)),
                2 => career!(Smuggler(M), Highwayman(W)),
                3 => career!(Sellsword(I), Highwayman(W)),
                4 => career!(Sellsword(I), Sawbones(P)),
                5 => career!(Field Captain(K), Baneman(D)),
                6 => career!(Field Captain(K), Baneman(D)),
                7..=9 => career!(Remain at rank 6),
                _ => unreachable!(),
            },
            Peasantry => match rank {
                0 => career!(Woodsman(H), Serf(L)),
                1 => career!(Woodsman(H), Serf(L)),
                2 => career!(Fisher(M), Mootman(F)),
                3 => career!(Fisher(M), Mootman(F)),
                4 => career!(Herder(T), Smallholder(F)),
                5 => career!(Herder(T), Smallholder(F)),
                6..=9 => career!(Remain at rank 5),
                _ => unreachable!(),
            },
            Performers => match rank {
                0 => career!(Penny Mummer(B)),
                1 => career!(Penny Mummer(B)),
                2 => career!(Mummer(E)),
                3 => career!(Mummer(E)),
                4 => career!(Herald(E), Proprietor(T)),
                5 => career!(Proprietor(T)),
                6..=9 => career!(Remain at rank 5),
                _ => unreachable!(),
            },
            Plebeians => match rank {
                0 => career!(Reroll affiliation with disadvantage),
                1 => career!(Outcast(B)),
                2 => career!(Peddler(C), Odd Jobber(L)),
                3..=4 => career!(Courtesan(E), *Lodge Artisan(A)
                    CareerTableStar::NeedsFaith(Faith::Accorder)),
                5 => career!(Proprietor(T), Master Artisan(A)),
                6 => career!(Proprietor(T), Master Artisan(A)),
                7..=9 => career!(Remain at rank 6),
                _ => unreachable!(),
            },
            Watch => match rank {
                0..=1 => career!(Join Scholars),
                2 => career!(Inquisitor(I)),
                3 => career!(Inquisitor(I)),
                4 => career!(Inquisition Seer(D), Bluecoat(H)),
                5 => career!(Inquisition Seer(D), Bluecoat Seer(S)),
                6 => career!(Sceptre Scribe(O), Temple Guardian(K)),
                7 => career!(Sceptre Scribe(O), Temple Guardian(K)),
                8..=9 => career!(Remain at rank 7),
                _ => unreachable!(),
            },
            Army => match rank {
                0..=1 => career!(Join Vagabonds),
                2 => career!(Watch(G), Auxiliary(H)),
                3 => career!(Sailor(M), Legionnaire(I)),
                4 => career!(Medicus(P), Legionnaire(I)),
                5 => career!(Medicus(P), Courier(W)),
                6 => career!(Field Captain(K)),
                7..=9 => career!(Remain at rank 6),
                _ => unreachable!(),
            },
            Scholars => match rank {
                0 => career!(Reroll affiliation with disadvantage),
                1 => career!(Mendicant(B)),
                2 => career!(Mendicant(B), Apothecary(P)),
                3 => career!(Cleric(Z), Apothecary(P)),
                4 => career!(Cleric(Z), Temple Seer(S)),
                5 => career!(Cleric(Z), Temple Seer(S)),
                6 => career!(Sceptre Scribe(O)),
                7 => career!(Sceptre Scribe(O)),
                8..=9 => career!(Remain at rank 7),
                _ => unreachable!(),
            },
            Gentry => match rank {
                0..=3 => career!(Reroll affiliation),
                4 => career!(Prefect(O)),
                5 => career!(Prefect(O), *Lodge Officiary(Z)
                    CareerTableStar::NeedsFaith(Faith::Accorder)),
                6 => career!(Courtier(O), *Lodge Officiary(Z)
                    CareerTableStar::NeedsFaith(Faith::Accorder)),
                7 => career!(Grandee(N), Centurion(K)),
                8 => career!(Grandee(N), Centurion(K)),
                9 => career!(Liege Lord(N)),
                _ => unreachable!(),
            },
        },
        CareerTable::Nivena => match affiliation {
            Slumfolk => match rank {
                0 => career!(Outcast(B)),
                1 => career!(Militia(G), OddJobber(L)),
                2 => career!(Militia(G), OddJobber(L)),
                3 => career!(Peddler(C), *Dewisetic(Z)
                    CareerTableStar::NeedsFaithAndCulture(Faith::Gytungrug, Culture::Kremish)),
                4..=9 => career!(Join Plebeians),
                _ => unreachable!(),
            },
            Criminals => match rank {
                0 => career!(Thief(R)),
                1 => career!(Thief(R), Bodyguard(G)),
                2 => career!(Sawbones(P), Smuggler(M)),
                3 => career!(Counterfeiter(C), Headhunter(D)),
                4 => career!(Fence(T), Gang Boss(K)),
                5 => career!(Gang Boss(K)),
                6..=9 => career!(Remain at rank 6),
                _ => unreachable!(),
            },
            Vagabonds => match rank {
                0 => career!(Deserter(R), Woodsman(H)),
                1 => career!(Deserter(R), Woodsman(H)),
                2 => career!(Smuggler(M), Highwayman(W)),
                3 => career!(Sellsword(I), Highwayman(W)),
                4 => career!(Sellsword(I), Sawbones(P)),
                5 => career!(Field Captain(K), Baneman(D)),
                6 => career!(Field Captain(K), Baneman(D)),
                7..=9 => career!(Remain at rank 6),
                _ => unreachable!(),
            },
            Peasantry => match rank {
                0 => career!(Woodsman(H), Serf(L)),
                1 => career!(Woodsman(H), Serf(L)),
                2 => career!(Fisher(M), Mootman(F)),
                3 => career!(Fisher(M), Mootman(F)),
                4 => career!(Herder(T), Smallholder(F)),
                5 => career!(Herder(T), Smallholder(F)),
                6..=9 => career!(Remain at rank 6),
                _ => unreachable!(),
            },
            Performers => match rank {
                0 => career!(Penny Mummer(B)),
                1 => career!(Lodge Apprentice(L), Bodyguard(G)),
                2 => career!(Mummer(E), Bodyguard(G)),
                3 => career!(Mummer(E), Lodge Merchant(T)),
                4 => career!(Herald(E), Lodge Merchant(T)),
                5 => career!(Lodge Officiary(Z)),
                6..=9 => career!(Remain at rank 6),
                _ => unreachable!(),
            },
            Plebeians => match rank {
                0..=1 => career!(Reroll affiliation with disadvantage),
                2 => career!(Bodyguard(G), Lodge Apprentice(L)),
                3 => career!(Bodyguard(G), Lodge Apprentice(L)),
                4 => career!(Lodge Merchant(T), Lodge Artisan(A)),
                5 => career!(Lodge Artisan(A), Lodge Officiary(Z)),
                6 => career!(Lodge Officiary(Z)),
                7..=9 => career!(Remain at rank 6),
                _ => unreachable!(),
            },
            Watch => match rank {
                0..=1 => career!(Join Criminals),
                2 => career!(Watch(G)),
                3 => career!(Watch(G), Courier(W)),
                4 => career!(Watch Sergeant(D), Pale Agent(R)),
                5 => career!(Courtier(O), Pale Agent(R)),
                6 => career!(Courtier(O), Pale Sergeant(C)),
                7..=9 => career!(Remain at rank 6),
                _ => unreachable!(),
            },
            Army => match rank {
                0..=1 => career!(Join Vagabonds),
                2 => career!(Militia(G), Spotter(R)),
                3 => career!(Legionary(I), Archer(W)),
                4 => career!(Legionary(I), Archer(W)),
                5 => career!(Landed Veteran(F), Prefect(O)),
                6 => career!(Centurion(K), Prefect(O)),
                7 => career!(Centurion(K), Senator(N)),
                8..=9 => career!(Remain at rank 7),
                _ => unreachable!(),
            },
            Scholars => match rank {
                0..=2 => career!(Reroll affiliation),
                3 => career!(Medicus(P)),
                4 => career!(Medicus(P), Lodge Scribe(O)),
                5 => career!(Lodge Scribe(O), Lodge Scholar(S)),
                6 => career!(Lodge Officiary(Z), Lodge Scholar(S)),
                7 => career!(Lodge Officiary(Z)),
                8..=9 => career!(Remain at rank 7),
                _ => unreachable!(),
            },
            Gentry => match rank {
                0..=1 => career!(Reroll affiliation with disadvantage),
                2 => career!(Bodyguard(G)),
                3 => career!(Bodyguard(G), SaIlor(M)),
                4 => career!(Caravan Guard(W), Sailor(M)),
                5 => career!(Caravan Guard(W), Lodge Merchant(T)),
                6 => career!(Lodge Merchant(T)),
                7 => career!(Lodge Merchant(T), Lodge Officiary(Z)),
                8 => career!(Lodge Officiary(Z)),
                9 => career!(Remain at rank 8),
                _ => unreachable!(),
            },
        },

        CareerTable::Marolaw => match affiliation {
            Slumfolk => match rank {
                0 => career!(Üslings(B), Clan Thrall(L)),
                1 => career!(Galley Thrall(M), Clan Thrall(L)),
                2 => career!(Galley Thrall(M), Peddler(C)),
                3 => career!(March Thrall(F), Courtesan(E)),
                4..=9 => career!(Join Criminals),
                _ => unreachable!(),
            },
            Criminals => match rank {
                0 => career!(Thief(R)),
                1 => career!(Thief(R), Bodyguard(G)),
                2 => career!(Sawbones(P), Smuggler(M)),
                3 => career!(Counterfeiter(C), Headhunter(D)),
                4 => career!(Fence(T), Gang Boss(K)),
                5 => career!(Gang Boss(K)),
                6..=9 => career!(Remain at rank 5),
                _ => unreachable!(),
            },
            Vagabonds => match rank {
                0 => career!(Deserter(R), Poacher(H)),
                1 => career!(Deserter(R), Poacher(H)),
                2 => career!(Smuggler(M), Highwayman(W)),
                3 => career!(Sellsword(I), Highwayman(W)),
                4 => career!(Sellsword(I), Sawbones(P)),
                5 => career!(Field Captain(K), Baneman(D)),
                6 => career!(Field Captain(K), Baneman(D)),
                7..=9 => career!(Join Gentry),
                _ => unreachable!(),
            },
            Peasantry => match rank {
                0 => career!(Woodsman(H), Serf(L)),
                1 => career!(Woodsman(H), Serf(L)),
                2 => career!(Fisher(M), Mootman(F)),
                3 => career!(Fisher(M), Mootman(F)),
                4 => career!(Herder(T), Smallholder(F)),
                5 => career!(Herder(T), Smallholder(F)),
                6..=9 => career!(Join Gentry),
                _ => unreachable!(),
            },
            Performers => match rank {
                0 => career!(Üstaler(C)),
                1 => career!(Üstaler(C), Skald(E)),
                2 => career!(Lundstaler(C), Skald(E)),
                3 => career!(Lundstaler(P), Runescribe(A)),
                4 => career!(Speaker(Z), Runescribe(A)),
                5 => career!(Speaker(Z), Chronicler(S)),
                6..=9 => career!(Remain at rank 5),
                _ => unreachable!(),
            },
            Plebeians | Army | Gentry => match rank {
                0..=1 => career!(Join Vagabonds),
                2 => career!(Drabling(G), Sailor(M)),
                3 => career!(Clan Crafter(A), Sailor(M)),
                4 => career!(Clan Crafter(A), Shield Bearer(I)),
                5 => career!(Herder(T), Shield Bearer(I)),
                6 => career!(Herder(T), Thane(K)),
                7 => career!(Courtier(O), Thane(K)),
                8 => career!(Jarl(N)),
                9 => career!(Remain at rank 8),
                _ => unreachable!(),
            },
            Watch => match rank {
                0..=1 => career!(Join Scholars),
                2 => career!(Inquisitor(I)),
                3 => career!(Inquisitor(I)),
                4 => career!(Inquisition Seer(D), Bluecoat(H)),
                5 => career!(Inquisition Seer(D), Bluecoat Seer(S)),
                6 => career!(Sceptre Scribe(O), Temple Guardian(K)),
                7 => career!(Sceptre Scribe(O), Temple Guardian(K)),
                8..=9 => career!(Remain at rank 7),
                _ => unreachable!(),
            },
            Scholars => match rank {
                0 => career!(Reroll affiliation with disadvantage),
                1 => career!(Mendicant(B)),
                2 => career!(Mendicant(B), Apothecary(P)),
                3 => career!(Cleric(Z), Apothecary(P)),
                4 => career!(Cleric(Z), Temple Seer(S)),
                5 => career!(Cleric(Z), Temple Seer(S)),
                6 => career!(Sceptre Scribe(O)),
                7 => career!(Sceptre Scribe(O)),
                8..=9 => career!(Remain at rank 7),
                _ => unreachable!(),
            },
        },
    }
}

pub fn get_career_requirements() {}

enum CareerClassPerkOption {
    Perk(Perk),
    AnyFightingStyle,
    AnyArmsFightingStyle,
    AnyHandsFightingStyle,
    AnyLore,
    AnyCraft,
    MutuallyExclusive(Perk, Perk),
}

impl CareerClass {
    // the primary skill is the first one
    const fn skills(&self) -> (Stat, Stat, Stat, Stat) {
        use Stat::*;

        match self {
            CareerClass::Artisan => (Craft, Arms, Power, Swing),
            CareerClass::Beggar => (Legs, Charm, Mind, Read),
            CareerClass::ConArtist => (Face, Hands, Power, Read),
            CareerClass::Detective => (Read, Dodge, Lore, Observe),
            CareerClass::Entertainer => (Dodge, Charm, Face, Hands),
            CareerClass::Farmer => (Swing, Arms, Control, Hands),
            CareerClass::Guard => (Observe, Aim, Block, Impose),
            CareerClass::Hunter => (Aim, Balance, Observe, Swing),
            CareerClass::Infantry => (Thrust, Block, Dodge, Legs),
            CareerClass::Knight => (Block, Control, Impose, Thrust),
            CareerClass::Labourer => (Arms, Balance, Craft, Legs),
            CareerClass::Mariner => (Power, Arms, Balance, Control),
            CareerClass::Noble => (Impose, Aim, Block, Language),
            CareerClass::Official => (Language, Face, Impose, Mind),
            CareerClass::Physician => (Hands, Craft, Impose, Thrust),
            CareerClass::Rogue => (Balance, Hands, Power, Thrust),
            CareerClass::Scholar => (Lore, Language, Legs, Mind),
            CareerClass::Trader => (Charm, Face, Language, Observe),
            CareerClass::Wayfarer => (Control, Aim, Dodge, Observe),
            CareerClass::Zealot => (Mind, Charm, Lore, Read),
        }
    }

    fn perks(
        &self,
    ) -> (
        CareerClassPerkOption,
        CareerClassPerkOption,
        CareerClassPerkOption,
    ) {
        use crate::data::perks::Perk::*;

        macro_rules! single {
            ((FightingStyle(A / H))) => {
                CareerClassPerkOption::AnyFightingStyle
            };
            ((FightingStyle(H))) => {
                CareerClassPerkOption::AnyHandsFightingStyle
            };
            ((FightingStyle(A))) => {
                CareerClassPerkOption::AnyArmsFightingStyle
            };
            (AnyLore) => {
                CareerClassPerkOption::AnyLore
            };
            (AnyCraft) => {
                CareerClassPerkOption::AnyCraft
            };
            (($id:tt || $id2:tt)) => {
                CareerClassPerkOption::MutuallyExclusive($id, $id2)
            };
            ($id:tt || $id2:tt) => {
                CareerClassPerkOption::MutuallyExclusive($id, $id2)
            };
            ($id:expr) => {
                CareerClassPerkOption::Perk($id)
            };
        }

        macro_rules! sof_option {
            ($($es:tt),+) => {(
                $(single![$es]),+
            )};
        }

        match self {
            CareerClass::Artisan => sof_option!(BulkBuyer, AnyCraft, Numeracy),
            CareerClass::Beggar => sof_option!(HeavySleeper, Instrumentalist, SturdyStuff),
            CareerClass::ConArtist => sof_option!(ColdRead, Disassembler, Finesse),
            CareerClass::Detective => sof_option!((ColdRead || Torturer), Locksmith, AnyLore),
            CareerClass::Entertainer => sof_option!(Dancer, Instrumentalist, Trainer),
            CareerClass::Farmer => sof_option!(AnyCraft, (Lore("Nature".to_owned())), Trainer),
            CareerClass::Guard => sof_option!(Grappler, Sentinel, (FightingStyle(A / H))),
            CareerClass::Hunter => sof_option!(LightFooted, Trainer, (FightingStyle(A))),
            CareerClass::Infantry => sof_option!(ArmourTraining, (FightingStyle(H)), StoneHearted),
            CareerClass::Knight => sof_option!(ArmourTraining, (FightingStyle(A / H)), Horseriding),
            CareerClass::Labourer => {
                sof_option!(Knockout, (Craft("Carpentry".to_owned())), StrongBack)
            }
            CareerClass::Mariner => sof_option!(Marine, Swimmer, (FightingStyle(A))),
            CareerClass::Noble => sof_option!(Bossy, Horseriding, Numeracy),
            CareerClass::Official => sof_option!(Dancer, Indulgent, Literacy),
            CareerClass::Physician => sof_option!(
                (Craft("Surgery".to_owned())),
                Literacy,
                (Lore("Medicine".to_owned()))
            ),
            CareerClass::Rogue => sof_option!(Finesse, LightFooted, (FightingStyle(H))),
            CareerClass::Scholar => sof_option!(Literacy, AnyLore, Numeracy),
            CareerClass::Trader => sof_option!(BulkBuyer, (Horseriding || Marine), Numeracy),
            CareerClass::Wayfarer => sof_option!(Horseriding, NightEyes, Pathfinder),
            CareerClass::Zealot => sof_option!(Literacy, (Lore("Religion".to_owned())), Ritualist),
        }
    }
}

impl Affiliation {
    pub fn star(&self, location: &Location) -> CareerTableStar {
        match location.career_table {
            CareerTable::ValiantEmpire => match self {
                Watch | Scholars => CareerTableStar::NeedsFaith(Faith::Accorder),
                _ => CareerTableStar::None,
            },
            CareerTable::Nivena => match self {
                Performers | Plebeians | Scholars | Gentry => {
                    CareerTableStar::NeedsFaith(Faith::IdealLodges)
                }
                _ => CareerTableStar::None,
            },
            CareerTable::Marolaw => match self {
                Performers => CareerTableStar::NeedsFaith(Faith::OrodTast),
                Watch | Scholars => CareerTableStar::NeedsFaith(Faith::Accorder),
                _ => CareerTableStar::None,
            },
        }
    }
}
