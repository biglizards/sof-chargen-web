use std::cmp::max;
// we use roll instead of maybe_roll here because birth events happen pre-omen
use crate::character::BIRTH_OMENS;
use crate::data::careers::{CareerTableStar, get_affiliation, get_rank};
use crate::data::locations::{Culture, Faith, associated_faith, further_afield_culture};
use crate::dice::{DiceRoll, MagicDice, d100};
use crate::event::util::{d3, d6};
use crate::event::{Event, util};
use crate::ipc::Choice;
use crate::{Backend, BirthOmen, CORE_STATS, Stat, choose_vec, roll, run};

pub gen fn pick_stat(backend: &impl Backend) -> Choice {
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

pub fn roll_magic(backend: &impl Backend) {
    let roll = MagicDice::roll();
    if roll.result() >= 100 {
        println!("You died during character creation!");
    }

    backend.set_stat(Stat::Magic, &roll);
}

pub fn roll_luck(backend: &impl Backend) {
    backend.set_stat(Stat::Luck, &d100());
}

pub fn roll_stamina(backend: &impl Backend) {
    backend.set_stat(Stat::Stamina, &roll!(2 d 6));
}

pub fn roll_core_stats(backend: &impl Backend) -> impl Event {
    pick_stat(backend)
        .chain(pick_stat(backend))
        .chain(pick_stat(backend))
        .chain(pick_stat(backend))
        .chain(pick_stat(backend))
}

pub fn roll_location_of_birth(backend: &impl Backend) {
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

pub gen fn affiliation_rank_careers(backend: &impl Backend) -> Choice {
    let loc;
    let culture;
    {
        // we're mutating char later on, so don't borrow it for very long
        let char = backend.get_character();
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
    run!(util::change_rank(backend, rank));

    // todo i think this should actually set your PARENT's career
    // you do want to keep the affiliation and rank though those are functionally yours
}

// Note: we move onto a new doc, which starts numbering from 1
// https://docs.google.com/document/d/13-d2KpDkzUQod8Uby-l3rSK8wRsfeMtZbFOxDvsswhY
// Step 1: Location of Birth
pub gen fn pick_omens<T: Backend>(backend: &T) -> Choice {
    let rank = backend.get_character().rank.unwrap_or_default();

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
            let parent_career = backend.get_character().parents_career.unwrap();
            backend.set_career(parent_career);
            return; // don't prompt the user to pick a career, they explicitly get their parents
        }
        BirthOmen::PropheticSigns(_) => {
            // Someone trusted by your guardians foresaw a striking destiny for you,
            // though whether great or terrible they could not say.
            // When rolling a die during character creation, you may choose the result up to twice.
            // Inherit your guardians’ rank, then reroll your affiliation.
            run!(util::roll_affiliation(backend, 0));
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
            run!(util::roll_affiliation(backend, 0));
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
            run!(util::roll_affiliation(backend, 1));
        }
    };

    // then we gain a career
    let rank = backend.get_character().rank.unwrap_or_default();
    run!(util::change_rank(backend, rank));
}
