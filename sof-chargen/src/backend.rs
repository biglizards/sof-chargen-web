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
        println!("birth location: {:?}", location);
        self.get_character_mut().birth_location = Some(location)
    }
    fn set_culture(&self, culture: Culture) {
        println!("culture: {:?}", culture);
        self.get_character_mut().culture = Some(culture)
    }
    fn set_faith(&self, faith: Faith) {
        println!("faith: {:?}", faith);
        self.get_character_mut().faith = Some(faith)
    }

    fn set_affiliation(&self, affiliation: Affiliation) {
        println!("affiliation: {:?}", affiliation);
        self.get_character_mut().affiliation = Some(affiliation)
    }
    fn set_career(&self, career: Career) {
        println!("career: {:?}", career);
        self.get_character_mut().careers.push(career);
    }
    fn set_rank(&self, rank: i8) {
        let rank = max(0, min(rank, 9)); // clamp between 0 and 9
        self.get_character_mut().rank = Some(rank);
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
