use sof_chargen::event::Event;
use sof_chargen::{Backend, Character};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct AppBackend {
    character: RefCell<Character>,
    pub(crate) log: RefCell<String>,
}

impl sof_chargen::Backend for AppBackend {
    fn get_character_mut(&self) -> impl DerefMut<Target = Character> {
        self.character.borrow_mut()
    }

    fn get_character(&self) -> impl Deref<Target = Character> {
        self.character.borrow()
    }

    fn log(&self, text: String) {
        let mut log = self.log.borrow_mut();
        log.push('\n');
        log.push_str(&text);
    }
}

impl AppBackend {
    pub(crate) fn next_stage(&'static self) -> Option<Box<dyn Event>> {
        let stage = self.get_character().life_stage;
        if let Some((stage, event)) = stage.next(self) {
            self.get_character_mut().life_stage = stage;
            Some(event)
        } else {
            None
        }
    }
}
