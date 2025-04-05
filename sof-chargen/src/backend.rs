use crate::character::{Character, Stat};
use crate::dice::DiceRoll;
use std::cell::RefCell;

pub trait Backend {
    fn set_stat<T: DiceRoll>(&self, stat: Stat, roll: &T);
    fn get_stat(&self, stat: Stat) -> Option<i8>;
    fn gain_trait(&self, description: String);
}

// the backend contract effectively requires interior mutability
// in theory this could be an UnsafeCell since we never return references to character
#[derive(Debug, Default)]
pub struct BaseBackend {
    pub character: RefCell<Character>,
}

impl Backend for BaseBackend {
    fn set_stat<T: DiceRoll>(&self, stat: Stat, new_val: &T) {
        // during character generation, stats may not go below 1
        self.character.borrow_mut().stats[stat] = Some(new_val.result().max(1));
    }
    fn get_stat(&self, stat: Stat) -> Option<i8> {
        self.character.borrow().stats[stat]
    }

    fn gain_trait(&self, description: String) {
        // just don't
        // normally you'd prompt the user for input and store it somewhere
        println!("{}", description)
    }
}
