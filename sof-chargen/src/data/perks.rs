// this file is under construction
#![allow(warnings)]

use subenum::subenum;

// ok so, the nature of perks
// some are specific to a primary skill, some are general
// some have pre-requisites
// some may be taken multiple times

// notions:
// you may take a perk if ...
//   - you have enough perk slots in the corresponding category
//   - you meet all the pre-requisites
// the effect of taking a perk is ...
//   - run some arbitrary function

use crate::data::careers::CareerClass;
use crate::data::locations::Language;
use crate::{Backend, Stat};

#[derive(Clone, Default, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Perks {
    arms: [PerkSlot; 10],
    face: [PerkSlot; 10],
    hands: [PerkSlot; 10],
    legs: [PerkSlot; 10],
    mind: [PerkSlot; 10],
}

impl Perks {
    fn get_table(&self, stat: Stat) -> &[PerkSlot; 10] {
        match stat {
            Stat::Arms => &self.arms,
            Stat::Face => &self.face,
            Stat::Hands => &self.hands,
            Stat::Legs => &self.legs,
            Stat::Mind => &self.mind,
            Stat::Magic => unimplemented!(), // maybe?
            _ => panic!("tried to gain perk in non-core stat!"), // todo consider sub-enums here
        }
    }
    fn get_table_mut(&mut self, stat: Stat) -> &mut [PerkSlot; 10] {
        match stat {
            Stat::Arms => &mut self.arms,
            Stat::Face => &mut self.face,
            Stat::Hands => &mut self.hands,
            Stat::Legs => &mut self.legs,
            Stat::Mind => &mut self.mind,
            Stat::Magic => unimplemented!(), // maybe?
            _ => panic!("tried to gain perk in non-core stat!"), // todo consider sub-enums here
        }
    }
    fn insert_perk(
        &mut self,
        stat: Stat,
        perk: Perk,
        provenance: Option<CareerClass>,
    ) -> Result<(), ()> {
        for perkslot in self.get_table_mut(stat).iter_mut() {
            if let PerkSlot::None = perkslot {
                *perkslot = PerkSlot::Perk(perk, provenance);
                return Ok(());
            }
        }

        Err(()) // no space left!
    }

    fn count_perk(&self, stat: Stat, perk: &Perk) -> usize {
        self.get_table(stat)
            .iter()
            .filter(|p| match p {
                PerkSlot::Perk(p, _) => p == perk,
                _ => false,
            })
            .count()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
enum PerkSlot {
    None,
    Perk(Perk, Option<CareerClass>),
    Wound(String),
    // todo is a mind wound which used to be a madness quirk notably different?
    //   i think so - they still count as madness quirks, right?
}

impl Default for PerkSlot {
    fn default() -> Self {
        Self::None
    }
}

#[subenum(ArmsPerk, FacePerk, HandsPerk, LegsPerk, MindPerk)]
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Perk {
    Addict,
    OldWound,

    #[subenum(ArmsPerk)]
    ArmourTraining,
    #[subenum(ArmsPerk)]
    Swimmer,
    #[subenum(ArmsPerk)]
    Knockout,
    #[subenum(ArmsPerk)]
    Mountaineer,
    #[subenum(ArmsPerk)]
    FightingStyle(FightingStyle),
    #[subenum(ArmsPerk)]
    Grappler,
    #[subenum(ArmsPerk)]
    Riposte,
    #[subenum(ArmsPerk)]
    Sentinel,
    #[subenum(ArmsPerk)]
    Adrenaline,
    #[subenum(ArmsPerk)]
    Feint,
    #[subenum(ArmsPerk)]
    Momentum,
    #[subenum(ArmsPerk)]
    PerfectParry,

    #[subenum(FacePerk)]
    BulkBuyer,
    #[subenum(FacePerk)]
    Manager,
    #[subenum(FacePerk)]
    Passionate,
    #[subenum(FacePerk)]
    Ritualist,
    #[subenum(FacePerk)]
    Bossy,
    #[subenum(FacePerk)]
    ColdRead,
    #[subenum(FacePerk)]
    Indulgent,
    #[subenum(FacePerk)]
    Retainer,
    #[subenum(FacePerk)]
    Torturer,
    #[subenum(FacePerk)]
    StoneHearted,
    #[subenum(FacePerk)]
    SturdyStuff,
    #[subenum(FacePerk)]
    Bard,
    #[subenum(FacePerk)]
    BraveFace,
    #[subenum(FacePerk)]
    Impeccable,
    #[subenum(FacePerk)]
    PerfectMask,
    #[subenum(FacePerk)]
    PiercingGaze,
    #[subenum(FacePerk)]
    Tactician,

    #[subenum(HandsPerk)]
    Horseriding,
    #[subenum(HandsPerk)]
    Marine,
    #[subenum(HandsPerk)]
    Instrumentalist,
    #[subenum(HandsPerk)]
    Craft(String),
    #[subenum(HandsPerk)]
    Ambidextrous,
    #[subenum(HandsPerk)]
    Disassembler,
    #[subenum(HandsPerk)]
    Finesse,
    #[subenum(HandsPerk)]
    RangedFightingStyle(RangedFightingStyle),
    #[subenum(HandsPerk)]
    Trainer,
    #[subenum(HandsPerk)]
    Captain,
    #[subenum(HandsPerk)]
    Locksmith,
    #[subenum(HandsPerk)]
    MountMastery,
    #[subenum(HandsPerk)]
    Quickdraw,

    #[subenum(LegsPerk)]
    StrongBack,
    #[subenum(LegsPerk)]
    MagicalAwakening,
    #[subenum(LegsPerk)]
    Dancer,
    #[subenum(LegsPerk)]
    Athletic,
    #[subenum(LegsPerk)]
    Diver,
    #[subenum(LegsPerk)]
    Irongut,
    #[subenum(LegsPerk)]
    Kicker,
    #[subenum(LegsPerk)]
    Vitality,
    #[subenum(LegsPerk)]
    Evasive,
    #[subenum(LegsPerk)]
    HeavySleeper,
    #[subenum(LegsPerk)]
    LightFooted,
    #[subenum(LegsPerk)]
    Reckless,
    #[subenum(LegsPerk)]
    DanceOfDeath,
    #[subenum(LegsPerk)]
    Tireless,

    #[subenum(MindPerk)]
    NightEyes,
    #[subenum(MindPerk)]
    Wargamer,
    #[subenum(MindPerk)]
    Fluency(Language),
    #[subenum(MindPerk)]
    Gamer,
    #[subenum(MindPerk)]
    Lore(String),
    #[subenum(MindPerk)]
    Numeracy,
    #[subenum(MindPerk)]
    Retraining,
    #[subenum(MindPerk)]
    Alchemist,
    #[subenum(MindPerk)]
    Chef,
    #[subenum(MindPerk)]
    Pathfinder,
    #[subenum(MindPerk)]
    Literacy,
    #[subenum(MindPerk)]
    QuickStudy,
    #[subenum(MindPerk)]
    Reactionary,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum RangedFightingStyle {
    CouchedLances,
    CutthroatScoundrel,
    HitAndRun,
    LaicSkirmishing,
    NivishLongbows,
    RedWallsOfTekitara,
    TorienneHail,
    VitalPoints,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum FightingStyle {
    ArdakAlleyfighting,
    ChargeLeader,
    FrayFighter,
    HumanShield,
    KnightBreaker,
    LebrissanCitizenSelfDefence,
    NobleDuelling,
    PhalanxFighter,
    PoorFingInfantry,
    SeaLegs,
    ThreeBladesOfSura,
}

enum CanTakePerk {
    True,
    False,
    DiscussWithDM,
    YesIfTrainedIn(&'static str),
}

fn can_take_perk(perk: Perk, backend: &impl Backend) -> CanTakePerk {
    unimplemented!("Function under construction - not really needed during char gen anyawy");

    use crate::character::Stat::*;
    use CanTakePerk::*;

    let perks = &backend.get_character().perks;

    // need to know:
    // all the character's stats
    // how many instances of this perk they have
    // do they have a free perk slot?
    // do they have the pre-requisite perks?
    // do you lack mutually exclusive perks?
    // have you magically awakened?
    // do they meet the fluff requirements (eg training)

    // ok lets try some decomposition
    macro_rules! requires_clause {
        ($perk:ident $base:literal+$addend:literal $stat:ident) => {
            (backend.get_stat($stat).unwrap_or_default()
                >= $base + ($addend*perks.count_perk(Arms, &Perk::$perk) as i8))
        };
        ($perk:ident $base:literal+$addend:literal $stat:ident and $($tail:tt)*) => {
            (backend.get_stat($stat).unwrap_or_default()
                >= $base + ($addend*perks.count_perk(Arms, &Perk::$perk) as i8))
            && requires_clause!($perk $($tail)*)
        };
        ($perk:ident $base:literal+$addend:literal $stat:ident or $($tail:tt)*) => {
            (backend.get_stat($stat).unwrap_or_default()
                >= $base + ($addend*perks.count_perk(Arms, &Perk::$perk) as i8))
            || requires_clause!($perk $($tail)*)
        };
        ($perk:ident $base:literal $stat:ident) => {
            (backend.get_stat($stat).unwrap_or_default() >= $base)
        };
        ($perk:ident $base:literal $stat:ident and $($tail:tt)*) => {
            (backend.get_stat($stat).unwrap_or_default() >= $base) && requires_clause!($perk $($tail)*)
        };
        ($perk:ident $base:literal $stat:ident or $($tail:tt)*) => {
            (backend.get_stat($stat).unwrap_or_default() >= $base) || requires_clause!($perk $($tail)*)
        };
    }
    macro_rules! requires {
        ($perk:ident, squeeble: $training:expr, $($tail:tt)*) => {
            if (requires_clause!($perk $($tail)*)) {
                $training
            } else {
                False
            }
        };
        ($perk:ident, training: $training:expr, $($tail:tt)*) => {
            requires!($perk, squeeble: YesIfTrainedIn($training), $($tail)*)
        };
        ($perk:ident, $($tail:tt)*) => {
            requires!($perk, squeeble: True, $($tail)*)
        };
    }

    match perk {
        Perk::Addict => DiscussWithDM,
        Perk::OldWound => False,
        Perk::ArmourTraining => {
            requires!(ArmourTraining, training: "a combat trainer specialised in armoured combat", 0+40 Block)
        }
        Perk::Swimmer => True,
        Perk::Knockout => requires!(Knockout, 30 Swing and 30 Thrust),
        Perk::Mountaineer => requires!(Mountaineer, 30 Arms and 30 Balance),
        Perk::FightingStyle(_) => todo!(), // can't right now // requires!(FightingStyle, 40+30 Arms subskill),
        Perk::Grappler => requires!(Grappler, 40 Arms),
        Perk::Riposte => requires!(Riposte, 40 Block),
        Perk::Sentinel => requires!(Sentinel, 40 Thrust or 40 Aim),
        Perk::Adrenaline => requires!(Adrenaline, 50 Arms),
        Perk::Feint => requires!(Feint, 50 Swing or 50 Thrust),
        Perk::Momentum => requires!(Momentum, 50 Swing),
        Perk::PerfectParry => {
            requires!(PerfectParry, training: "training from a parrying master", 60 Block)
        }
        _ => todo!(),
    };
    False
}

impl Perk {
    fn take_effect(&self, backend: &impl Backend) {
        match self {
            _ => todo!(),
        }
    }
}
