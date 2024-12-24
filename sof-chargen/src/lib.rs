#[macro_use]
extern crate enum_map;

pub use backend::Backend;
pub use character::{Character, Stat, CORE_STATS};

mod backend;
mod character;
mod dice;
pub mod event;


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
