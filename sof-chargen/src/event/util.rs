use crate::data::careers::{
    Career, CareerTableEntry, CareerTableStar, get_affiliation, get_careers,
};
use crate::data::locations::Culture;
use crate::dice::DiceRoll;
use crate::ipc::Choice;
use crate::{Backend, ask, choose, roll};
use std::cmp::{max, min};

// maybe we want these to render somehow in the future?
pub(crate) fn d6() -> i8 {
    roll!(1 d 6).result()
}
pub(crate) fn d3() -> i8 {
    roll!(1 d 3).result()
}

#[macro_export]
macro_rules! run {
    ($call: expr) => {
        for choice in $call {
            yield choice;
        }
    };
}

// todo i hate this i hate this i hate it so much
pub(crate) gen fn roll_affiliation(backend: &impl Backend, mut disadvantage: usize) -> Choice {
    let char = backend.get_character();
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

pub(crate) gen fn change_rank(backend: &impl Backend, rank: i8) -> Choice {
    let char = backend.get_character();
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
                run!(roll_affiliation(backend, disadvantage));

                let char = backend.get_character();
                faith = char.faith.unwrap();
                affiliation = char.affiliation.unwrap();
                rank = char.rank.unwrap();
            }
        }
    };
    backend.set_career(career);
}
