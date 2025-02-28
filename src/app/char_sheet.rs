use crate::SoFCharGenApp;
use crate::app::AppTab;
use egui::{Layout, RichText, Ui};
use sof_chargen::ipc::Choice;
use sof_chargen::ipc::Selection;
use sof_chargen::{CORE_STATS, Character, Stat, event};

impl SoFCharGenApp {
    fn choose(&self, i: usize) {
        match &self.current_event {
            None => {}
            Some(t) => t.choose(i),
        }
    }

    fn submit_trait(&mut self) {
        if let Some(t) = &self.current_event {
            self.log_choice(&self.trait_submission);
            t.submit_trait(std::mem::take(&mut self.trait_submission));
        }
    }

    fn poll_current_choice(&mut self) {
        if self.current_event.is_some() {
            self.current_choice = self
                .current_event
                .as_ref()
                .unwrap()
                .current_choice()
                .clone()
        }
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
            self.submit_trait();
        }
    }

    fn trait_window(&mut self, ctx: &egui::Context) {
        // if we're requesting a trait, put that up in front of the choice buttons
        if let Some(Choice::String(t)) = &self.current_choice {
            egui::Window::new("Gain a Trait").show(ctx, |ui| {
                ui.label(&*t.description);
                self.trait_buttons(ui);
            });
        }
    }

    fn choice_buttons(&self, ui: &mut Ui, choice: &Selection) {
        ui.horizontal(|ui| {
            for (i, option) in choice.options.iter().enumerate() {
                let as_str = format!("{:?}", option);
                if ui.button(&as_str).clicked() {
                    self.choose(i);
                    self.log_choice(&as_str);
                }
            }
        });
    }
    fn choice_window(&self, ctx: &egui::Context) {
        if let Some(Choice::Selection(s)) = &self.current_choice {
            egui::Window::new("Choice").show(ctx, |ui| {
                ui.label(s.description);
                self.choice_buttons(ui, s);
            });
        }
    }

    fn stats(&self, ui: &mut egui::Ui) {
        // first row: name, [blank], luck, magic
        ui.columns(4, |columns| {
            columns[0].text_edit_singleline(&mut self.character.borrow_mut().name);
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
        let traits = &self.backend.character.borrow().traits;
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

    fn debug_buttons(&mut self, ui: &mut egui::Ui) {
        if ui.button("Generate Core Stats").clicked() {
            self.current_event = Some(event::roll_core_stats(self.backend.clone()).into());
        }
        if ui.button("Roll Magic and Luck").clicked() {
            event::roll_magic(&mut self.backend);
            event::roll_luck(&mut self.backend);
        }
        if ui.button("Reset").clicked() {
            *self.backend.character.borrow_mut() = Character {
                stats: Default::default(),
                name: "Enter Name".to_string(),
                traits: vec![],
            };
            self.reset_log();
        }
        if ui.button("Pick a Star").clicked() {
            self.current_event =
                Some(event::prosperous_constellations(self.backend.clone()).into());
        }
        ui.separator();
    }

    fn render_sheet(&mut self, ctx: &egui::Context) {
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
                    match &self.current_choice {
                        Some(Choice::String(t)) => {
                            ui.label(t.description);
                            self.trait_buttons(ui);
                        }
                        Some(Choice::Selection(s)) => {
                            ui.label(s.description);
                            self.choice_buttons(ui, s);
                        }
                        _ => {}
                    }
                });
        });
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        self.poll_current_choice();

        self.tab = self.render_top_panel(ctx);

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
