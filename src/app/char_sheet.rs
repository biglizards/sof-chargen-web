use crate::app::backend::AppBackend;
use crate::{util, SoFCharGenApp};
use egui::{Layout, RichText, Ui};
use sof_chargen::event::{roll_core_stats, Event};
use sof_chargen::{event, Character, Stat, CORE_STATS};

impl SoFCharGenApp {
    fn stat_box(&self, ui: &mut Ui, stat: Stat) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(stat.to_string());
                for subskill in stat.subskills() {
                    ui.label(subskill.to_string());
                }
            });
            ui.vertical(|ui| {
                ui.label(self.get_stat_str(stat));
                for subskill in stat.subskills() {
                    ui.label(self.get_stat_str(subskill));
                }
            });
        });
    }

    fn trait_window(&mut self, ctx: &egui::Context) {
        // if we're requesting a trait, put that up in front of the choice buttons
        if !self.trait_description.is_empty() {
            egui::Window::new("Gain a Trait").show(ctx, |ui| {
                ui.label(&self.trait_description);
                ui.text_edit_multiline(&mut self.trait_submission);
                if ui.button("Submit").clicked() {
                    let s = self.trait_sender.as_mut().unwrap().clone();
                    let submission = self.trait_submission.clone();
                    util::spawn_thread(async move {
                        s.send(submission).await.unwrap();
                    });
                    self.trait_sender = None;
                    self.trait_description = String::new();
                    self.trait_submission = String::new();
                }
            });
        }
    }
    fn choice_window(&mut self, ctx: &egui::Context) {
        if !self.choice.is_empty() {
            egui::Window::new("Choice").show(ctx, |ui| {
                let mut chosen = false;
                ui.label(&self.choice_description);
                ui.horizontal(|ui| {
                    for (i, option) in self.choice.iter().enumerate() {
                        if ui.button(option).clicked() {
                            let s = self.choice_send.as_mut().unwrap().clone();
                            util::spawn_thread(async move {
                                s.send(i).await.unwrap();
                            });
                            chosen = true;
                        }
                    }
                });
                if chosen {
                    self.choice = vec![];
                    self.choice_send = None;
                }
            });
        }
    }

    fn stats(&self, ui: &mut egui::Ui) {
        // first row: name, [blank], luck, magic
        ui.columns(4, |columns| {
            columns[0].text_edit_singleline(&mut self.character.write().unwrap().name);
            // luck and magic
            columns[2].label(format!("Magic: {}", self.get_stat_str(Stat::Magic)));
            columns[3].label(format!("Luck: {}", self.get_stat_str(Stat::Luck)));
        });

        // add the core stats
        ui.columns(5, |columns| {
            for (i, &stat) in CORE_STATS.iter().enumerate() {
                self.stat_box(&mut columns[i], stat);
            }
        });

        ui.separator();
    }

    fn traits(&self, ui: &mut egui::Ui) {
        // add all the traits
        let traits = &self.backend.character.read().unwrap().traits;
        if !traits.is_empty() {
            ui.label("Traits");
        }
        ui.columns(traits.len(), |columns| {
            for (i, str) in traits.iter().enumerate() {
                columns[i].label(str);
            }
        });
        if !traits.is_empty() {
            ui.separator();
        }
    }

    fn debug_buttons(&self, ui: &mut egui::Ui) {
        if ui.button("Generate Core Stats").clicked() {
            let mut b = self.backend.clone();
            util::spawn_thread(async move {
                roll_core_stats::<AppBackend>().run(&mut b).await;
            });
        }
        if ui.button("Roll Magic and Luck").clicked() {
            let mut b = self.backend.clone();
            util::spawn_thread(async move {
                event::roll_magic(&mut b).await;
                event::roll_luck(&mut b).await;
            });
        }
        if ui.button("Reset").clicked() {
            *self.backend.character.write().unwrap() = Character {
                stats: Default::default(),
                name: "Enter Name".to_string(),
                traits: vec![],
            };
        }
        if ui.button("Pick a Star").clicked() {
            let mut b = self.backend.clone();
            util::spawn_thread(async move {
                event::prosperous_constellations(&mut b).await;
            });
        }
        ui.separator();
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        render_top_panel(ctx);

        self.check_channels();

        self.trait_window(ctx);
        self.choice_window(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("SoF Chargen");

            self.stats(ui);

            self.traits(ui);

            self.debug_buttons(ui);

            // footer
            ui.with_layout(Layout::bottom_up(egui::Align::LEFT), |ui| {
                crate::app::char_sheet::disclaimer(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
fn render_top_panel(ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        // The top panel is often a good place for a menu bar:

        egui::menu::bar(ui, |ui| {
            egui::widgets::global_theme_preference_buttons(ui);
        });
    });
}

fn disclaimer(ui: &mut egui::Ui) {
    ui.label(
        RichText::new("⚠ Not Official")
            .small()
            .color(ui.visuals().warn_fg_color),
    )
    .on_hover_text("For personal use only, not endorsed by Lys");
}