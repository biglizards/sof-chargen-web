use sof_chargen::dice::Roll;
use sof_chargen::{Backend, Character, Stat};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct AppBackend {
    pub character: Rc<RefCell<Character>>,
}

impl Backend for AppBackend {
    fn set_stat(&mut self, stat: Stat, new_val: i8) {
        self.character.borrow_mut().stats[stat] = Some(new_val);
    }

    fn set_stat_by_roll(&mut self, stat: Stat, roll: &Roll) {
        self.set_stat(stat, roll.result());
    }
    fn get_stat(&self, stat: Stat) -> Option<i8> {
        self.character.borrow().stats[stat]
    }

    fn gain_trait(&mut self, description: String) {
        self.character.borrow_mut().traits.push(description);
    }
}
