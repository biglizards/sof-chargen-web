use crate::character::{BirthOmen, Character, Stat};
use crate::data::careers::{Affiliation, Career};
use crate::data::locations::{Culture, Faith, Location};
use crate::data::perks::Perk;
use crate::dice::DiceRoll;
use std::cell::RefCell;
use std::cmp::{max, min};
use std::ops::{Deref, DerefMut};

pub trait Backend {
    fn get_character_mut(&self) -> impl DerefMut<Target = Character>;
    fn get_character(&self) -> impl Deref<Target = Character>;

    fn set_stat(&self, stat: Stat, roll: &impl DiceRoll) {
        // during character generation, stats may not go below 1
        self.get_character_mut().stats[stat] = Some(roll.result().max(1));
    }
    fn get_stat(&self, stat: Stat) -> Option<i8> {
        self.get_character().stats[stat]
    }
    fn gain_trait(&self, description: String) {
        self.get_character_mut().traits.push(description)
    }
    fn get_omen(&self) -> Option<BirthOmen> {
        self.get_character().omen
    }
    fn set_omen(&self, omen: BirthOmen) {
        self.get_character_mut().omen = Some(omen)
    }

    fn gain_perk(&self, perk: Perk) {
        todo!("generic impl")
    }

    fn set_birth_location(&self, location: Location) {
        self.log(format!("You were born in {}.", location.name));
        self.get_character_mut().birth_location = Some(location)
    }
    fn set_culture(&self, culture: Culture) {
        self.log(format!("You were raised {}.", culture));
        self.get_character_mut().culture = Some(culture)
    }
    fn set_faith(&self, faith: Faith) {
        let first_faith = self.get_character().faith.is_none();
        self.log(match self.get_character().age {
            0 if first_faith => format!("Your parents worshipped {}.", faith),
            0 => format!("Your parents converted to {}.", faith),
            1..15 => format!("For the sake of your apprenticeship, you were raised to follow {}.", faith),
            _ => format!("You converted to {}.", faith)
        });
        self.get_character_mut().faith = Some(faith)
    }

    fn set_affiliation(&self, affiliation: Affiliation) {
        match self.get_character().affiliation {
            None => self.log(format!("Your parents were members of the {}.", affiliation)),
            Some(old) if old != affiliation => self.log(format!("You joined the {}.", affiliation)),
            _ => {}
        }

        self.get_character_mut().affiliation = Some(affiliation)
    }
    fn set_career(&self, career: Career) {
        if self.get_character().parents_career.is_none() {
            self.log(format!("Your parents were {}s.", career.name));
            self.get_character_mut().parents_career = Some(career);
            return;
        }
        
        let (already_present, num_careers) = {
            let careers = &self.get_character().careers;
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
        self.get_character_mut().careers.push(career);
    }
    fn set_rank(&self, rank: i8) {
        let old_rank = self.get_character().rank;
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
        self.get_character_mut().rank = Some(rank);
    }

    fn log(&self, text: String) {
        println!("{}", text);
    }
}

// the backend contract effectively requires interior mutability
// in theory this could be an UnsafeCell since we never return references to character
#[derive(Debug, Default)]
pub struct BaseBackend {
    pub character: RefCell<Character>,
}

impl Backend for BaseBackend {
    fn get_character_mut(&self) -> impl DerefMut<Target = Character> {
        self.character.borrow_mut()
    }

    fn get_character(&self) -> impl Deref<Target = Character> {
        self.character.borrow()
    }
}
