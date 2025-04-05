// SoFCharGenApp contains:
// - state needed to run the web/app interface

use crate::app::backend::BACKEND;
use egui::os::OperatingSystem;
use sof_chargen::ipc::Choice;
use sof_chargen::{Backend, Stat};
use std::cell::Cell;

mod backend;
mod char_sheet;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SoFCharGenApp {
    tab: AppTab,

    #[serde(skip)]
    trait_submission: String,
    #[serde(skip)]
    current_event: Option<Box<dyn Iterator<Item = Choice>>>,
    #[serde(skip)]
    current_choice: Option<Choice>,
    #[serde(skip)]
    made_choice: Cell<bool>,
}

impl Default for SoFCharGenApp {
    fn default() -> Self {
        Self {
            trait_submission: String::new(),
            tab: AppTab::Sheet,
            current_event: None,
            current_choice: None,
            made_choice: Default::default(),
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
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn get_stat_str(&self, stat: Stat) -> String {
        if let Some(v) = BACKEND.get_stat(stat) {
            return v.to_string();
        }
        "-".to_string()
    }

    fn get_current_prompt(&self) -> Option<&'static str> {
        match &self.current_choice {
            None => None,
            Some(o) => Some(o.description()),
        }
    }

    fn log_choice(&self, choice: &str) {
        let mut log = BACKEND.log.borrow_mut();
        log.push('\n');

        if let Some(description) = self.get_current_prompt() {
            log.push_str(description)
        } else {
            log.push_str("[TRIED TO LOG CHOICE WITH NO PENDING CHOICE]")
        }
        log.push_str(": ");
        log.push_str(choice);
    }
    fn reset_log(&self) {
        BACKEND.log.borrow_mut().clear()
    }
}

impl eframe::App for SoFCharGenApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render(ctx);
        if self.made_choice.take() || self.current_event.is_some() && self.current_choice.is_none()
        {
            self.current_choice = None;
            self.current_choice = self.current_event.as_mut().unwrap().next();
            if self.current_choice.is_none() {
                self.current_event = None;
            }
        }
    }

    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
        eframe::set_value(storage, backend::BACKEND_KEY, &*BACKEND);
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
enum AppTab {
    Sheet,
    DEMode,
}
