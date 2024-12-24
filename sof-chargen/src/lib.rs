#[macro_use]
extern crate enum_map;

use crate::dice::d100;
use crate::event::*;
use backend::{Backend, BaseBackend};
use character::Stat;
use event::Event;

mod backend;
mod character;
mod dice;
mod event;

fn not_main() {
    let mut backend: BaseBackend = Default::default();
    for _ in 0..5 {
        pick_stat.run(&mut backend);
    }

    roll_luck
        .then(roll_magic)
        .then(prosperous_constellations)
        .run(&mut backend);

    println!("Hello, world! {:?}", backend);
}
fn main() {
    not_main()
}

/*
 ok so rough structure of chargen
 - rolling core stats
   - choice: ordering
 - choice: star sign
   - this may involve even more choices
 - choice: age
 - rolling events
   - career table
   - choice: what to take from a career

 given the massive amount of choices lets make it a state machine,
 and test every transition before adding any randomness

 hardcode:
 - core stats
 - types of transition
 - star signs?

 load from files:
 - events
 - careers
 - career tables
 - flavour text
*/
