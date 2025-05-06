pub mod birth;
mod util;

use crate::backend::Backend;
use crate::data::locations::{Culture, Faith};
use crate::dice::DiceRoll;
use crate::ipc::Choice;
use crate::{maybe_roll, roll};
pub trait Event = Iterator<Item = Choice>;

pub gen fn test_pick_dice<T: Backend>(backend: &T) -> Choice {
    let roll = maybe_roll!("test roll please ignore", backend, 1 d 10);
    println!("got {} in the gen fn", roll.result());
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
    pub fn kremish_accorder(backend: &impl Backend) -> impl Event {
        // scenario 1. You rolled a rank 3 slum folk kremish accorder
        // you should be offered the option to convert to Gytungrug
        *backend.get_character_mut() = Character {
            birth_location: test_location(),
            culture: Some(Culture::Kremish),
            faith: Some(Faith::Accorder),
            affiliation: Some(Affiliation::Slumfolk),
            rank: Some(3),
            ..Default::default()
        };

        util::change_rank(backend, 3)
    }

    pub fn non_kremish_accorder(backend: &impl Backend) -> impl Event {
        // scenario 2. You rolled a rank 3 slum folk valish accorder
        // you should NOT be offered the option to convert to Gytungrug
        *backend.get_character_mut() = Character {
            birth_location: test_location(),
            culture: Some(Culture::Varlish),
            faith: Some(Faith::Accorder),
            affiliation: Some(Affiliation::Slumfolk),
            rank: Some(3),
            ..Default::default()
        };

        util::change_rank(backend, 3)
    }
}
