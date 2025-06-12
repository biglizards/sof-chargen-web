use serde::{Deserialize, Serialize};

use crate::character::{BirthOmen, Character, Stat};
use crate::data::careers::{Affiliation, Career};
use crate::data::locations::{Culture, Faith, Location};
use crate::data::perks::Perk;
use crate::dice::DiceRoll;
use std::cell::{Ref, RefCell, RefMut};
use std::cmp::{max, min};
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct BaseBackend {
    pub character: Character,
    pub log_fifo: VecDeque<String>,
}

impl BaseBackend {
    pub fn set_stat(&mut self, stat: Stat, roll: &impl DiceRoll) {
        // during character generation, stats may not go below 1
        self.character.stats[stat] = Some(roll.result().max(1));
    }
    pub fn get_stat(&self, stat: Stat) -> Option<i8> {
        self.character.stats[stat]
    }
    pub fn gain_trait(&mut self, description: String) {
        self.character.traits.push(description)
    }
    pub fn get_omen(&self) -> Option<BirthOmen> {
        self.character.omen
    }
    pub fn set_omen(&mut self, omen: BirthOmen) {
        self.character.omen = Some(omen)
    }

    pub fn gain_perk(&self, _perk: Perk) {
        todo!("generic impl")
    }

    pub fn set_birth_location(&mut self, location: Location) {
        self.log(format!("You were born in {}.", location.name));
        self.character.birth_location = Some(location)
    }
    pub fn set_culture(&mut self, culture: Culture) {
        self.log(format!("You were raised {}.", culture));
        self.character.culture = Some(culture)
    }
    pub fn set_faith(&mut self, faith: Faith) {
        let first_faith = self.character.faith.is_none();
        self.log(match self.character.age {
            0 if first_faith => format!("Your parents worshipped {}.", faith),
            0 => format!("Your parents converted to {}.", faith),
            1..15 => format!(
                "For the sake of your apprenticeship, you were raised to follow {}.",
                faith
            ),
            _ => format!("You converted to {}.", faith),
        });
        self.character.faith = Some(faith)
    }

    pub fn set_affiliation(&mut self, affiliation: Affiliation) {
        match self.character.affiliation {
            None => self.log(format!("Your parents were members of the {}.", affiliation)),
            Some(old) if old != affiliation => self.log(format!("You joined the {}.", affiliation)),
            _ => {}
        }

        self.character.affiliation = Some(affiliation)
    }
    pub fn set_career(&mut self, career: Career) {
        if self.character.parents_career.is_none() {
            self.log(format!("Your parents were {}s.", career.name));
            self.character.parents_career = Some(career);
            return;
        }

        let (already_present, num_careers) = {
            let careers = &self.character.careers;
            (careers.contains(&career), careers.len())
        };
        self.log(match num_careers {
            0 => format!(
                "You were apprenticed as a {}, granting you the skills of a {}.",
                career.name, career.class
            ),
            _ if already_present => format!(
                "You continued working as a {}, granting you the skills of a master {}.",
                career.name, career.class
            ),
            _ => format!(
                "You spent a time working as a {}, granting you the skills of a {}.",
                career.name, career.class
            ),
        });
        // todo in character creation 3.0, gain a perk with that career name instead
        self.character.careers.push(career);
    }
    pub fn set_rank(&mut self, rank: i8) {
        let old_rank = self.character.rank;
        match old_rank {
            None => self.log(format!("Your parents lived life at rank {}.", rank)),
            Some(i) => match rank - i {
                0 => {}
                1 => self.log("You gained a rank.".to_string()),
                -1 => self.log("You fell a rank.".to_string()),
                x if x > 0 => self.log(format!("You gained {} ranks.", x)),
                x if x < 0 => self.log(format!("You fell {} ranks.", -x)),
                _ => unreachable!(),
            },
        }
        let rank = max(0, min(rank, 9)); // clamp between 0 and 9
        self.character.rank = Some(rank);
    }

    pub fn log(&mut self, text: String) {
        println!("{}", text);
        self.log_fifo.push_back(text);
    }
}

// TODO consider making this a pinned pointer instead, which would make this type copy
// pros: don't have to call .clone() all over the place
// cons: i will 100% mess up the drop order by using unsafe code
//   (make sure that this field is always last so it gets dropped last)
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct Backend {
    base: Rc<RefCell<BaseBackend>>,
}

impl Backend {
    pub fn character(&self) -> impl Deref<Target = Character> {
        Ref::map(self.base.borrow(), |b| &b.character)
    }
    pub fn character_mut(&self) -> impl DerefMut<Target = Character> {
        RefMut::map(self.base.borrow_mut(), |b| &mut b.character)
    }

    pub fn set_stat(&self, stat: Stat, roll: &impl DiceRoll) {
        self.base.borrow_mut().set_stat(stat, roll);
    }
    pub fn get_stat(&self, stat: Stat) -> Option<i8> {
        self.base.borrow().get_stat(stat)
    }
    pub fn gain_trait(&self, description: String) {
        self.base.borrow_mut().gain_trait(description);
    }
    pub fn get_omen(&self) -> Option<BirthOmen> {
        self.base.borrow().get_omen()
    }
    pub fn set_omen(&self, omen: BirthOmen) {
        self.base.borrow_mut().set_omen(omen);
    }

    pub fn gain_perk(&self, _perk: Perk) {
        todo!("generic impl")
    }

    pub fn set_birth_location(&self, location: Location) {
        self.base.borrow_mut().set_birth_location(location);
    }
    pub fn set_culture(&self, culture: Culture) {
        self.base.borrow_mut().set_culture(culture);
    }
    pub fn set_faith(&self, faith: Faith) {
        self.base.borrow_mut().set_faith(faith);
    }

    pub fn set_affiliation(&self, affiliation: Affiliation) {
        self.base.borrow_mut().set_affiliation(affiliation);
    }
    pub fn set_career(&self, career: Career) {
        self.base.borrow_mut().set_career(career);
    }
    pub fn set_rank(&self, rank: i8) {
        self.base.borrow_mut().set_rank(rank);
    }

    pub fn log(&self, text: String) {
        self.base.borrow_mut().log(text);
    }
    pub fn get_log(&self) -> impl DerefMut<Target = VecDeque<String>> {
        RefMut::map(self.base.borrow_mut(), |b| &mut b.log_fifo)
    }
}
