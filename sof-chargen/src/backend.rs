use crate::character::{Character, Stat};
use crate::dice::Roll;

pub trait Backend {
    fn set_stat(&mut self, stat: Stat, new_val: i8);
    fn set_stat_by_roll(&mut self, stat: Stat, roll: &Roll) {
        self.set_stat(stat, roll.result())
    }
    fn get_stat(&self, stat: Stat) -> Option<i8>;
    fn gain_trait(&mut self, description: String);
}
#[derive(Debug, Default)]
pub struct BaseBackend {
    pub character: Character,
}

impl Backend for BaseBackend {
    fn set_stat(&mut self, stat: Stat, new_val: i8) {
        self.character.stats[stat] = Some(new_val);
    }
    fn get_stat(&self, stat: Stat) -> Option<i8> {
        self.character.stats[stat]
    }

    fn gain_trait(&mut self, description: String) {
        // just don't
        // normally you'd prompt the user for input and store it somewhere
        println!("{}", description)
    }
}
