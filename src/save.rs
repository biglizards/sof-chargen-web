use std::sync::LazyLock;

use crate::App;

const SAVE_FILE: &'static str = "character.sof";
const LOG_FILE: &'static str = "character.log";

#[cfg(target_arch = "wasm32")]
fn get_string_from_storage(key: &str) -> Option<String> {
    web_sys::window()?
        .local_storage()
        .ok()?
        .map(|storage| storage.get_item(key).ok())??
}

#[cfg(target_arch = "wasm32")]
fn save_string_to_storage(key: &str, save: &str) {
    web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .map(|storage| storage.set_item(key, save).unwrap())
        .unwrap()
}

#[cfg(not(target_arch = "wasm32"))]
fn get_string_from_storage(key: &str) -> Option<String> {
    std::fs::read_to_string(key).ok()
}

#[cfg(not(target_arch = "wasm32"))]
fn save_string_to_storage(key: &str, save: &str) {
    std::fs::write(key, save).unwrap();
}

static BACKEND_STR: LazyLock<String> = LazyLock::new(|| {
    // the string is a static because the backend holds the character, which holds careers,
    // which have names and those names are static strings (even though they're serialised)
    // really i should replace all the strings with enums and have a lookup table for the strings
    // (since that'd be needed for supporting multiple languages anyway)
    // but this isn't the worst thing in the world
    get_string_from_storage(SAVE_FILE).unwrap_or_default()
});

pub fn save_app(app: &App) {
    save_string_to_storage(
        SAVE_FILE,
        &ron::to_string(&app.backend).expect("failed to serialize backend!"),
    );

    save_string_to_storage(LOG_FILE, &app.log);
}

pub fn load_app() -> App {
    App {
        backend: ron::from_str(&BACKEND_STR).unwrap_or_default(),
        log: get_string_from_storage(LOG_FILE).unwrap_or_default(),
        ..Default::default()
    }
}
