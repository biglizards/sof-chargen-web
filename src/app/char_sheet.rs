use crate::app::backend::AppBackend;
use crate::app::AppTab;
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

    fn trait_buttons(&mut self, ui: &mut Ui) {
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
    }

    fn trait_window(&mut self, ctx: &egui::Context) {
        // if we're requesting a trait, put that up in front of the choice buttons
        if !self.trait_description.is_empty() {
            egui::Window::new("Gain a Trait").show(ctx, |ui| {
                ui.label(&self.trait_description);
                self.trait_buttons(ui);
            });
        }
    }

    fn choice_buttons(&mut self, ui: &mut Ui) {
        let mut chosen = false;
        ui.horizontal(|ui| {
            for (i, option) in self.choice.iter().enumerate() {
                if ui.button(option).clicked() {
                    let s = self.choice_send.as_mut().unwrap().clone();
                    util::spawn_thread(async move {
                        s.send(i).await.unwrap();
                    });
                    chosen = true;
                    self.log(option);
                }
            }
        });
        if chosen {
            self.choice = vec![];
            self.choice_send = None;
        }
    }
    fn choice_window(&mut self, ctx: &egui::Context) {
        if !self.choice.is_empty() {
            egui::Window::new("Choice").show(ctx, |ui| {
                ui.label(&self.choice_description);
                self.choice_buttons(ui);
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
            self.reset_log();
        }
        if ui.button("Pick a Star").clicked() {
            let mut b = self.backend.clone();
            util::spawn_thread(async move {
                event::prosperous_constellations(&mut b).await;
            });
        }
        ui.separator();
    }

    fn render_sheet(&self, ctx: &egui::Context) {
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

    fn render_log(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.with_layout(Layout::top_down_justified(egui::Align::LEFT), |ui| {
                        ui.label(&*self.log.borrow());
                    });
                    if !self.trait_description.is_empty() {
                        self.trait_buttons(ui);
                    } else if !self.choice.is_empty() {
                        self.choice_buttons(ui);
                    }
                });
        });
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        self.tab = self.render_top_panel(ctx);

        self.check_channels();

        match self.tab {
            AppTab::Sheet => {
                self.trait_window(ctx);
                self.choice_window(ctx);
                self.render_sheet(ctx);
            }
            AppTab::DEMode => self.render_log(ctx),
        }
    }

    fn render_top_panel(&self, ctx: &egui::Context) -> AppTab {
        egui::TopBottomPanel::top("top_panel")
            .show(ctx, |ui| {
                // The top panel is often a good place for a menu bar:

                egui::menu::bar(ui, |ui| {
                    egui::widgets::global_theme_preference_buttons(ui);
                    if ui.button("📝 Sheet").clicked() {
                        return AppTab::Sheet;
                    }
                    if ui.button("🔎 DE Mode").clicked() {
                        return AppTab::DEMode;
                    }
                    self.tab
                })
                .inner
            })
            .inner
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
