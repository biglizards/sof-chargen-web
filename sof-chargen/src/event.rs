use crate::backend::Backend;
use crate::character::{BIRTH_OMENS, BirthOmen, Stat};
use crate::data::careers::{
    Career, CareerTableEntry, CareerTableStar, get_affiliation, get_careers, get_rank,
};
use crate::data::locations::{Culture, Faith, associated_faith, further_afield_culture};
use crate::dice::{DiceRoll, MagicDice, d100};
use crate::ipc::{Choice, Question};
use crate::{CORE_STATS, ask, choose, choose_vec, maybe_roll, pick_roll, roll};
use std::cell::Cell;
use std::cmp::{max, min};
use std::rc::Rc;

// maybe we want these to render somehow in the future?
fn d6() -> i8 {
    roll!(1 d 6).result()
}
fn d3() -> i8 {
    roll!(1 d 3).result()
}

macro_rules! run {
    ($call: expr) => {
        for choice in $call {
            yield choice;
        }
    };
}

// todo i hate this i hate this i hate it so much
gen fn roll_affiliation(backend: Backend, mut disadvantage: usize) -> Choice {
    let char = backend.character();
    let loc = char.birth_location.clone().unwrap();
    let culture = char.culture.unwrap();
    let mut faith = char.faith.unwrap();
    let mut rank = char.rank.unwrap();
    let mut affiliation;
    drop(char);

    loop {
        // Roll 1d100 on your birthplace’s affiliation table [...]
        // if your guardians were not members of that location’s dominant culture roll with disadvantage
        // note from lys: if you already have disadvantage from somewhere (eg table says to) it's double
        affiliation = if loc.culture == culture {
            get_affiliation(&loc, roll!(kh((1+disadvantage) d 100)).result())
        } else {
            get_affiliation(&loc, roll!(kh((2+disadvantage) d 100)).result())
        };
        backend.set_affiliation(affiliation);

        match affiliation.star(&loc) {
            CareerTableStar::None => break,
            CareerTableStar::NeedsFaith(f) => {
                if faith == f {
                    // no issues here, they get the affiliation
                    break;
                }
                // ask if they want to convert
                if ask!(format!(
                    "To be a {} you must follow {}. Do you convert? \
                                    If you don't, fall one rank and re-roll affiliation.",
                    affiliation, f
                )) {
                    faith = f;
                    backend.set_faith(faith);
                    break;
                } else {
                    rank = max(rank - 1, 0);
                    backend.set_rank(rank);
                    // we're re-rolling so reset the disadvantage
                    disadvantage = 0;
                }
            }
            _ => unreachable!("Affiliations should never require a culture"),
        };
    }
}

// macro instead of a function because we would need to return a value from a gen fn
macro_rules! handle_star {
    ($star:ident, $career:ident, $backend:ident, $culture:ident, $faith:ident) => {
        match $star {
            CareerTableStar::None => break $career,
            CareerTableStar::NeedsFaith(f) => {
                if ask!(
                    format!("You need to be a {f:?} to that that career. Do you convert?")
                        .to_owned()
                ) {
                    $backend.set_faith(f);
                    break $career;
                }
            }
            CareerTableStar::NeedsFaithAndCulture(f, c) => {
                if c == $culture {
                    if f == $faith {
                        break $career;
                    } else {
                        if ask!(
                            format!("You need to be a {f:?} to that that career. Do you convert?")
                                .to_owned()
                        ) {
                            $backend.set_faith(f);
                            break $career;
                        }
                    }
                } else {
                    // todo make this a popup or something
                    //   or better, don't even offer the option
                    println!("You can't pick that one!")
                }
            }
        }
    };
}

fn is_eligible(culture: Culture, career_table_star: CareerTableStar) -> bool {
    match career_table_star {
        CareerTableStar::None => true,
        CareerTableStar::NeedsFaith(_) => true,
        CareerTableStar::NeedsFaithAndCulture(_, c) => culture == c,
    }
}

gen fn change_rank(backend: Backend, rank: i8) -> Choice {
    let char = backend.character();
    let loc = char.birth_location.clone().unwrap();
    let mut affiliation = char.affiliation.unwrap();
    let culture = char.culture.unwrap();
    let mut faith = char.faith.unwrap();
    // drop char so we can borrow it as mut later (within handle_star)
    drop(char);

    let mut rank = max(0, min(rank, 9)); // clamp between 0 and 9

    let career = loop {
        let entry = get_careers(&loc, affiliation, rank);
        match entry {
            CareerTableEntry::Career(career, star) => {
                handle_star!(star, career, backend, culture, faith)
            }
            CareerTableEntry::Careers((c1, s1), (c2, s2)) => {
                let careers: Vec<Career> = [(c1, s1), (c2, s2)]
                    .iter()
                    .filter(|(_, s)| is_eligible(culture, *s))
                    .map(|&(c, _)| c)
                    .collect();
                let career = if careers.len() == 1 {
                    *careers.first().unwrap()
                } else {
                    choose!("Pick your guardians' career:", c1, c2)
                };
                let star = if career == c1 { s1 } else { s2 };
                handle_star!(star, career, backend, culture, faith)
            }
            CareerTableEntry::RemainAtRank(r) => {
                rank = r;
                backend.set_rank(rank);
                // rerun the loop to pick a career for that rank
            }
            CareerTableEntry::ChangeAffiliation(a) => {
                affiliation = a;
                backend.set_affiliation(affiliation)
                // rerun the loop to pick a new career for that affiliation
            }
            CareerTableEntry::RerollWithDisadvantage | CareerTableEntry::Reroll => {
                let disadvantage = if entry == CareerTableEntry::RerollWithDisadvantage {
                    1
                } else {
                    0
                };

                // awkward limitation of gen fns - they can't return anything
                // (and you can't get around that by passing a &mut arg)
                // instead allow it to arbitrarily mutate the character and update our state after
                run!(roll_affiliation(backend.clone(), disadvantage));

                let char = backend.character();
                faith = char.faith.unwrap();
                affiliation = char.affiliation.unwrap();
                rank = char.rank.unwrap();
            }
        }
    };
    backend.set_career(career);
}

pub gen fn pick_stat(backend: Backend) -> Choice {
    let core_stat = choose_vec!(
        "Pick a core stat to roll next",
        CORE_STATS
            .into_iter()
            .filter(|&x| backend.get_stat(x).is_none())
    );

    // in character creation we pick the lowest of all rolled dice,
    // which is the same as rolling with advantage
    let num_dice = 2 + CORE_STATS
        .into_iter()
        .filter(|&x| backend.get_stat(x).is_some_and(|x| x >= 50))
        .count();
    let roll = roll!(kl(num_dice d 100));
    backend.set_stat(core_stat, &roll);
    let stat = roll.result();

    let mut remaining_stats: Vec<Stat> = core_stat
        .subskills()
        .into_iter()
        .filter(|&x| backend.get_stat(x).is_none())
        .collect();

    for i in 0..3 {
        let choice = choose_vec!("pick a sub-skill", remaining_stats);
        let mallus_roll = roll!(stat - i d 10);
        backend.set_stat(choice, &mallus_roll);
    }
}

pub fn roll_magic(backend: Backend) {
    let roll = MagicDice::roll();
    if roll.result() >= 100 {
        println!("You died during character creation!");
    }

    backend.set_stat(Stat::Magic, &roll);
}
pub fn roll_luck(backend: Backend) {
    backend.set_stat(Stat::Luck, &d100());
}

pub fn roll_stamina(backend: Backend) {
    backend.set_stat(Stat::Stamina, &roll!(2 d 6));
}

pub fn roll_core_stats(backend: Backend) -> impl Iterator<Item = Choice> {
    pick_stat(backend.clone())
        .chain(pick_stat(backend.clone()))
        .chain(pick_stat(backend.clone()))
        .chain(pick_stat(backend.clone()))
        .chain(pick_stat(backend))
}

// Note: we move onto a new doc, which starts numbering from 1
// https://docs.google.com/document/d/13-d2KpDkzUQod8Uby-l3rSK8wRsfeMtZbFOxDvsswhY
// Step 1: Location of Birth
pub gen fn pick_omens(backend: Backend) -> Choice {
    println!("ASDF picking omens!");
    let rank = backend.character().rank.unwrap_or_default();

    let omen = choose_vec!(consume "Pick your birth omen", BIRTH_OMENS);
    backend.set_omen(omen);
    match omen {
        BirthOmen::ProsperousConstellations => {
            // You grew up doted on by guardians who saw you as their ticket to wealth and success.
            // Reroll your stamina and keep the highest, and inherit your guardians’ affiliation and rank.
            backend.set_stat(
                Stat::Stamina,
                &max(
                    roll!(2 d 6).result(),
                    backend.get_stat(Stat::Stamina).unwrap_or_default(),
                ),
            );
        }
        BirthOmen::PropheticSigns(_) => {
            // Someone trusted by your guardians foresaw a striking destiny for you,
            // though whether great or terrible they could not say.
            // When rolling a die during character creation, you may choose the result up to twice.
            // Inherit your guardians’ rank, then reroll your affiliation.
            run!(roll_affiliation(backend.clone(), 0));
        }
        BirthOmen::PracticallyMinded => {
            // Whatever omens were present at your birth, your guardians were practical folk who
            // gave them little notice.  Pick any two primary skills, and swap their values
            // and those of each of their secondaries.
            let mut skills = CORE_STATS.to_vec();
            let skill1 = choose_vec!(
                "Pick any two primary skills, and swap their values and those of each of their secondaries",
                skills
            );
            let skill2 = choose_vec!(
                "Pick any two primary skills, and swap their values and those of each of their secondaries",
                skills
            );
            println!("swapping {skill1} and {skill2}");

            let v1 = backend.get_stat(skill1).unwrap_or_default();
            let v2 = backend.get_stat(skill2).unwrap_or_default();
            backend.set_stat(skill1, &v2);
            backend.set_stat(skill2, &v1);

            skill1
                .subskills()
                .into_iter()
                .zip(skill2.subskills().into_iter())
                .for_each(|(s1, s2)| {
                    let v1 = backend.get_stat(s1).unwrap_or_default();
                    let v2 = backend.get_stat(s2).unwrap_or_default();
                    backend.set_stat(s1, &v2);
                    backend.set_stat(s2, &v1);
                });

            // Inherit your guardians’ affiliation, but start one rank lower.
            backend.set_rank(max(rank - 1, 0))
        }
        BirthOmen::ShootingStar => {
            // A symbol of change accompanied your birth;
            // your guardians could only pray that when it came it would be for the better.
            // Reroll Luck, but start one rank below your guardians and reroll your affiliation.
            backend.set_stat(Stat::Luck, &roll!(1 d 100));
            backend.set_rank(max(rank - 1, 0));
            run!(roll_affiliation(backend.clone(), 0));
        }
        BirthOmen::PortentsOfDoom => {
            // Without explanation, your guardians shunned you from birth,
            // for the omens accompanying it were unambiguous in their dark promises.
            // Reroll Magic and keep the highest roll, but start 1d3 ranks below your guardians
            // and reroll your affiliation with disadvantage.
            backend.set_stat(
                Stat::Magic,
                &max(
                    backend.get_stat(Stat::Magic).unwrap_or_default(),
                    MagicDice::roll().result(),
                ),
            );
            backend.set_rank(max(rank - d3(), 0));
            run!(roll_affiliation(backend.clone(), 1));
        }
    };

    // then we gain a career
    let rank = backend.character().rank.unwrap_or_default();
    run!(change_rank(backend, rank));
}

pub gen fn test_pick_dice(backend: Backend) -> Choice {
    let roll = maybe_roll!("test roll please ignore", backend, 1 d 10);
    println!("got {} in the gen fn", roll.result());
}

pub fn roll_location_of_birth(backend: Backend) {
    let loc = crate::data::locations::location_table((d6(), d6(), d6()), d3());

    // ok just to speed things up a bit we're doing step 2 here too
    if loc.far_afield {
        backend.set_culture(loc.culture);
        backend.set_faith(loc.faith);
    } else {
        let culture = match d6() {
            1..=3 => loc.culture,
            4..=5 => loc.secondary_culture,
            6 => match d6() {
                1..=3 => Culture::Varlish,
                4 => Culture::Revic,
                5 => Culture::Kremish,
                6 => further_afield_culture((d6(), d6())),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        let faith = match d6() {
            1..=2 => {
                if culture == Culture::Kremish {
                    Faith::Gytungrug
                } else {
                    loc.faith
                }
            }
            3 => loc.secondary_faith,
            4..=5 => associated_faith(culture),
            6 => match d6() {
                1..=3 => Faith::Accorder,
                4..=5 => Faith::Irreligious,
                6 => Faith::TempleOfSeraf,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        backend.set_culture(culture);
        backend.set_faith(faith);
    }

    backend.set_birth_location(loc);
}

pub gen fn affiliation_rank_careers(backend: Backend) -> Choice {
    let loc;
    let culture;
    {
        // we're mutating char later on, so don't borrow it for very long
        let char = backend.character();
        loc = char.birth_location.as_ref().unwrap().clone();
        culture = char.culture.unwrap();
    }

    // Once you have found the culture and faith you were brought up in,
    // use these to determine the affiliation, rank and career of your guardians
    let affiliation = if loc.culture == culture {
        get_affiliation(&loc, roll!(1 d 100).result())
    } else {
        get_affiliation(&loc, roll!(kh(2 d 100)).result())
    };

    match affiliation.star(&loc) {
        CareerTableStar::None => affiliation,
        CareerTableStar::NeedsFaith(f) => {
            // normally you'd reroll and drop a rank if you don't match it,
            // but you haven't rolled rank yet.
            // ruling from lys: your parents are forced to convert
            backend.set_faith(f);
            affiliation
        }
        _ => unreachable!(),
    };

    // Then, roll the rank die listed under that affiliation,
    let rank = get_rank(&loc, affiliation, d6());
    backend.set_rank(rank);
    backend.set_affiliation(affiliation);

    // and select a career listed at that rank for that affiliation for them to have practised
    run!(change_rank(backend, rank));

    // todo i think this should actually set your PARENT's career
    // you do want to keep the affiliation and rank though those are functionally yours
}

// test scenarios
pub mod scenarios {
    use super::*;
    use crate::Character;
    use crate::data::careers::Affiliation;
    use crate::data::locations::{CareerTable, Demographic, Location};
    fn test_location() -> Option<Location> {
        Some(Location {
            name: "test location".to_string(),
            culture: Culture::Varlish,
            secondary_culture: Culture::Varlish,
            faith: Faith::Accorder,
            secondary_faith: Faith::Accorder,
            demographic: Demographic::Urban,
            career_table: CareerTable::ValiantEmpire,
            far_afield: false,
        })
    }
    pub fn kremish_accorder(backend: Backend) -> impl Iterator<Item = Choice> {
        // scenario 1. You rolled a rank 3 slum folk kremish accorder
        // you should be offered the option to convert to Gytungrug
        *backend.character_mut() = Character {
            birth_location: test_location(),
            culture: Some(Culture::Kremish),
            faith: Some(Faith::Accorder),
            affiliation: Some(Affiliation::Slumfolk),
            rank: Some(3),
            ..Default::default()
        };

        change_rank(backend, 3)
    }

    pub fn non_kremish_accorder(backend: Backend) -> impl Iterator<Item = Choice> {
        // scenario 2. You rolled a rank 3 slum folk valish accorder
        // you should NOT be offered the option to convert to Gytungrug
        *backend.character_mut() = Character {
            birth_location: test_location(),
            culture: Some(Culture::Varlish),
            faith: Some(Faith::Accorder),
            affiliation: Some(Affiliation::Slumfolk),
            rank: Some(3),
            ..Default::default()
        };

        change_rank(backend, 3)
    }
}
