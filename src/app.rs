#[cfg(not(target_arch = "wasm32"))]
use async_std::task;

use egui::{Layout, RichText, Ui};
use sof_chargen::{event, Backend, Character, Stat, CORE_STATS};
use std::fmt;
use std::future::Future;
use std::sync::{mpsc, Arc, RwLock};

#[cfg(target_arch = "wasm32")]
fn spawn_thread<F: Future>(callback: F)
where
    F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(callback);
}

#[cfg(not(target_arch = "wasm32"))]
fn spawn_thread<F>(callback: F)
where
    F: Future<Output = ()> + 'static + Future + Send,
{
    task::spawn(callback);
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)]
    backend: AppBackend,

    character: Arc<RwLock<Character>>,

    choice: Vec<String>,

    #[serde(skip)]
    choice_vec: mpsc::Receiver<(String, Vec<String>, async_channel::Sender<usize>)>,
    #[serde(skip)]
    choice_send: Option<async_channel::Sender<usize>>,
}

#[derive(Clone)]
struct AppBackend {
    character: Arc<RwLock<Character>>,
    choice_vec: mpsc::Sender<(String, Vec<String>, async_channel::Sender<usize>)>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let (cv_s, cv_r) = mpsc::channel();
        let character: Arc<RwLock<Character>> = Default::default();
        Self {
            backend: AppBackend {
                character: Arc::clone(&character),
                choice_vec: cv_s,
            },
            character,
            choice: Default::default(),
            choice_vec: cv_r,
            choice_send: Default::default(),
        }
    }
}

impl Backend for AppBackend {
    async fn choose<T: Copy + fmt::Display>(&self, description: &str, options: &Vec<T>) -> T {
        let (s, r) = async_channel::bounded(1);
        self.choice_vec
            .send((
                description.to_string(),
                options.iter().map(|x| x.to_string()).collect(),
                s,
            ))
            .unwrap();
        let choice = r.recv().await.unwrap();
        options[choice]
    }

    fn set_stat(&mut self, stat: Stat, new_val: i8) {
        self.character.write().unwrap().stats[stat] = new_val;
    }
    fn get_stat(&self, stat: Stat) -> i8 {
        self.character.read().unwrap().stats[stat]
    }

    fn gain_trait(&mut self, description: &str) {
        // just don't
        // normally you'd prompt the user for input and store it somewhere
        println!("{}", description)
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            let mut this: Self = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            this.backend.character = this.character.clone();
            return this;
        }

        Default::default()
    }

    fn stat_box(&self, ui: &mut Ui, stat: Stat) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(stat.to_string());
                for subskill in stat.subskills() {
                    ui.label(subskill.to_string());
                }
            });
            ui.vertical(|ui| {
                ui.label(self.backend.get_stat(stat).to_string());
                for subskill in stat.subskills() {
                    ui.label(self.backend.get_stat(subskill).to_string());
                }
            });
        });
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        assert!(Arc::ptr_eq(&self.character, &self.backend.character));

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        if let Ok((_desc, vec, s)) = self.choice_vec.try_recv() {
            self.choice = vec;
            self.choice_send = Some(s);
        }

        if !self.choice.is_empty() {
            egui::Window::new("Choice").show(ctx, |ui| {
                let mut chosen = false;
                for (i, option) in self.choice.iter().enumerate() {
                    if ui.button(option).clicked() {
                        let s = self.choice_send.as_mut().unwrap().clone();
                        spawn_thread(async move {
                            s.send(i).await.unwrap();
                        });
                        chosen = true;
                    }
                }
                if chosen {
                    self.choice = vec![];
                }
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("SoF Chargen");

            // add the core stats
            ui.columns(5, |columns| {
                for (i, &stat) in CORE_STATS.iter().enumerate() {
                    self.stat_box(&mut columns[i], stat);
                }
            });

            if ui.button("Generate column").clicked() {
                let mut b = self.backend.clone();
                spawn_thread(async move {
                    event::pick_stat(&mut b).await;
                });
            }
            if ui.button("Reset").clicked() {
                *self.backend.character.write().unwrap() = Character {
                    stats: Default::default(),
                };
            }

            ui.separator();

            ui.with_layout(Layout::bottom_up(egui::Align::LEFT), |ui| {
                disclaimer(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn disclaimer(ui: &mut egui::Ui) {
    ui.label(
        RichText::new("⚠ Not Official")
            .small()
            .color(ui.visuals().warn_fg_color),
    )
    .on_hover_text("For personal use only, not endorsed by Lys");
}
