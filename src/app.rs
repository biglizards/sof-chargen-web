use backend::AppBackend;
use egui::os::OperatingSystem;
use sof_chargen::{Backend, Character, Stat};
use std::sync::{mpsc, Arc, RwLock};

mod backend;
mod char_sheet;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SoFCharGenApp {
    #[serde(skip)]
    backend: AppBackend,

    character: Arc<RwLock<Character>>,

    #[serde(skip)]
    choice: Vec<String>,
    #[serde(skip)]
    choice_description: String,

    #[serde(skip)]
    choice_vec: mpsc::Receiver<(String, Vec<String>, async_channel::Sender<usize>)>,
    #[serde(skip)]
    choice_send: Option<async_channel::Sender<usize>>,

    // it really feels like there should be a less verbose way of representing this idiom
    #[serde(skip)]
    trait_read: mpsc::Receiver<(String, async_channel::Sender<String>)>,
    #[serde(skip)]
    trait_description: String,
    #[serde(skip)]
    trait_submission: String,
    #[serde(skip)]
    trait_sender: Option<async_channel::Sender<String>>,
}

impl Default for SoFCharGenApp {
    fn default() -> Self {
        let (cv_s, cv_r) = mpsc::channel();
        let (t_s, t_r) = mpsc::channel();
        let character: Arc<RwLock<Character>> = Default::default();
        Self {
            backend: AppBackend {
                character: Arc::clone(&character),
                choice_vec: cv_s,
                trait_send: t_s,
            },
            character,
            choice: Default::default(),
            choice_description: Default::default(),
            choice_vec: cv_r,
            choice_send: Default::default(),
            trait_read: t_r,
            trait_description: Default::default(),
            trait_submission: Default::default(),
            trait_sender: Default::default(),
        }
    }
}

impl SoFCharGenApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        cc.egui_ctx.set_zoom_factor(match cc.egui_ctx.os() {
            // mobile screens are smaller, so don't zoom
            OperatingSystem::Android | OperatingSystem::IOS => 1.0,
            // desktop screens have enough real estate for this
            _ => 2.0,
        });

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            let mut this: Self = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            this.backend.character = this.character.clone();
            return this;
        }

        Default::default()
    }

    fn check_channels(&mut self) {
        // check the buffers to see if we've been sent any interaction requests from events
        if let Ok((desc, vec, s)) = self.choice_vec.try_recv() {
            self.choice = vec;
            self.choice_send = Some(s);
            self.choice_description = desc;
        }
        if let Ok((desc, s)) = self.trait_read.try_recv() {
            self.trait_description = desc;
            self.trait_sender = Some(s);
            self.trait_submission = String::new();
        }
    }

    fn get_stat_str(&self, stat: Stat) -> String {
        if let Some(v) = self.backend.get_stat(stat) {
            return v.to_string();
        }
        "-".to_string()
    }
}

impl eframe::App for SoFCharGenApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        assert!(Arc::ptr_eq(&self.character, &self.backend.character));
        self.render(ctx);
    }

    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
