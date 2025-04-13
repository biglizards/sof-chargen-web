// SoFCharGenApp contains:
// - state needed to run the web/app interface

use crate::app::backend::BACKEND;
use crate::gui_event::GUIEvent;
use egui::os::OperatingSystem;
use sof_chargen::ipc::Choice;
use sof_chargen::{Backend, Stat};
use std::cell::Cell;

mod backend;
mod char_sheet;

#[derive(Default)]
pub struct SoFCharGenApp {
    tab: AppTab,

    // state of UI elements which persists between frames:
    picking_roll: Cell<i8>, // selecting a number to roll when using prophetic stars
    trait_submission: String, // any time the user is entering a string

    current_event: Option<Box<dyn Iterator<Item = Choice>>>,
    current_choice: Option<Choice>,

    // should replace made_choice
    current_gui_event: Cell<Option<GUIEvent>>,
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

impl SoFCharGenApp {
    fn apply_event(&mut self, event: GUIEvent) {
        let should_advance = event.should_advance();

        match event {
            GUIEvent::Choose(i) => match &self.current_choice {
                Some(Choice::Selection(s)) => s.chosen.set(i),
                _ => panic!("attempted to choose when there is no choice!"),
            },
            GUIEvent::SubmitTrait(submission) => {
                if self.current_event.is_some() {
                    self.log_choice(&submission);
                }

                match &self.current_choice {
                    Some(Choice::String(t)) => {
                        t.chosen.set(submission);
                    }
                    _ => panic!("attempted to choose when there is no choice!"),
                }
            }
            GUIEvent::PickRoll(choice) => match &self.current_choice {
                Some(Choice::PickRoll(p)) => {
                    p.chosen.set(choice);
                }
                _ => panic!("attempted to pick roll when there is no choice!"),
            },

            GUIEvent::ResetAll => {}
        }

        if should_advance {
            self.advance_event();
        }
    }

    fn handle_ui_events(&mut self) {
        match self.current_gui_event.take() {
            None => {}
            Some(e) => self.apply_event(e),
        }

        if self.current_event.is_some() && self.current_choice.is_none() {
            self.advance_event()
        }
    }

    fn advance_event(&mut self) {
        self.current_choice = None;
        self.current_choice = self.current_event.as_mut().unwrap().next();
        if self.current_choice.is_none() {
            self.current_event = None;
        }
    }
}

impl eframe::App for SoFCharGenApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render(ctx);
        self.handle_ui_events();
    }

    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, backend::BACKEND_KEY, &*BACKEND);
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
enum AppTab {
    Sheet,
    DEMode,
}

impl Default for AppTab {
    fn default() -> Self {
        Self::Sheet
    }
}
