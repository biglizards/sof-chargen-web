use crate::character::{Character, Stat};
use crate::dice::DiceRoll;
use std::cell::RefCell;
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
}

// the backend contract effectively requires interior mutability
// in theory this could be an UnsafeCell since we never return references to character
#[derive(Debug, Default)]
pub struct BaseBackend {
    pub character: RefCell<Character>,
}

impl Backend for BaseBackend {
    fn get_character_mut(&self) -> impl DerefMut<Target=Character> {
        self.character.borrow_mut()
    }

    fn get_character(&self) -> impl Deref<Target=Character> {
        self.character.borrow()
    }
}
