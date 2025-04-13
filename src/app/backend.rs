use sof_chargen::{Backend, Character};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::sync::LazyLock;

#[cfg(not(target_arch = "wasm32"))]
use epi::Storage;

pub static BACKEND_KEY: &str = "backend";

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct AppBackend {
    pub character: RefCell<Character>,
    pub log: RefCell<String>,
}

// we don't use threads, so all types are vacuously Sync
unsafe impl Sync for AppBackend {}

// largely taken from epi/eframe/egui, which don't make these functions public enough to use
#[cfg(target_arch = "wasm32")]
fn get_string_from_storage(key: &str) -> Option<String> {
    web_sys::window()?
        .local_storage()
        .ok()?
        .map(|storage| storage.get_item(key).ok())??
}

#[cfg(not(target_arch = "wasm32"))]
fn get_string_from_storage(key: &str) -> Option<String> {
    epi::file_storage::FileStorage::from_app_name("sofchargen")
        .expect("Failed to create save dir")
        .get_string(key)
}

// the app backend is a static because it needs to be passed by reference to gen blocks,
//   but gen blocks can only hold references to things with lifetimes as long as their own
// Since I don't know how long the lifetime of the Box<Dyn Event> held in current_event,
//   I can't give it a reference any shorter than 'static
// (I tried to constrain the lifetime to be the same as the App,
//  but couldn't figure out how to name that lifetime in a sane way)
pub static BACKEND: LazyLock<AppBackend> = LazyLock::new(|| {
    get_string_from_storage(BACKEND_KEY)
        .map(|value| ron::from_str(&value).unwrap_or_default())
        .unwrap_or_default()
});

impl Backend for AppBackend {
    fn get_character_mut(&self) -> impl DerefMut<Target = Character> {
        self.character.borrow_mut()
    }

    fn get_character(&self) -> impl Deref<Target = Character> {
        self.character.borrow()
    }
}
