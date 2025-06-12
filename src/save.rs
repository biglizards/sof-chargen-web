use std::sync::LazyLock;

use sof_chargen::Backend;

#[cfg(target_arch = "wasm32")]
fn get_string_from_storage() -> Option<String> {
    web_sys::window()?
        .local_storage()
        .ok()?
        .map(|storage| storage.get_item("save").ok())??
}

#[cfg(target_arch = "wasm32")]
fn save_string_to_storage(save: &str) {
    web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .map(|storage| storage.set_item("save", save).unwrap())
        .unwrap()
}

#[cfg(not(target_arch = "wasm32"))]
fn get_string_from_storage() -> Option<String> {
    std::fs::read_to_string("character.sof").ok()
}

#[cfg(not(target_arch = "wasm32"))]
fn save_string_to_storage(save: &str) {
    std::fs::write("character.sof", save).unwrap();
}

static BACKEND_STR: LazyLock<String> = LazyLock::new(|| {
    // the string is also a static because the backend holds the character, which holds careers,
    // which have names and those names are static strings (even though they're serialised)
    // really i should replace all the strings with enums and have a lookup table for the strings
    // (since that'd be needed for supporting multiple languages anyway)
    // but this isn't the worst thing in the world
    get_string_from_storage().unwrap_or_default()
});

pub fn save_backend(backend: &Backend) {
    let save = ron::to_string(backend).expect("failed to serialize backend!");
    save_string_to_storage(&save);
}
