use crate::SoFCharGenApp;
use crate::app::AppTab;
use egui::{Layout, RichText, Ui};
use sof_chargen::ipc::Selection;
use sof_chargen::ipc::{Choice, TraitChoice};
use sof_chargen::{CORE_STATS, Character, Stat, event};
use std::rc::Rc;

macro_rules! peek_choice {
    ($self: ident) => {
        &*if let Some(e) = &($self).current_event {
            e.current_choice()
        } else {
            // hoping this gets optimised out since it's immediately dereferenced above
            Rc::new(None)
        }
    };
}
pub(crate) use peek_choice;

impl SoFCharGenApp {
    fn choose(&mut self, i: usize) {
        match &mut self.current_event {
            None => {}
            Some(t) => t.choose(i),
        }
    }

    fn submit_trait(&mut self) {
        if self.current_event.is_some() {
            self.log_choice(&self.trait_submission);
        }
        if let Some(t) = &mut self.current_event {
            t.submit_trait(std::mem::take(&mut self.trait_submission));
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

    fn trait_window(&mut self, ctx: &egui::Context, t: &TraitChoice) {
        // if we're requesting a trait, put that up in front of the choice buttons
        egui::Window::new("Gain a Trait").show(ctx, |ui| {
            ui.label(&*t.description);
            self.trait_buttons(ui);
        });
    }

    fn choice_buttons(&mut self, ui: &mut Ui, choice: &Selection) {
        ui.horizontal(|ui| {
            for (i, option) in choice.options.iter().enumerate() {
                let as_str = format!("{:?}", option);
                if ui.button(&as_str).clicked() {
                    self.log_choice(&as_str);
                    self.choose(i);
                }
            }
        });
    }
    fn choice_window(&mut self, ctx: &egui::Context, s: &Selection) {
        egui::Window::new("Choice").show(ctx, |ui| {
            ui.label(s.description);
            self.choice_buttons(ui, s);
        });
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
                    match peek_choice!(self) {
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
        self.tab = self.render_top_panel(ctx);

        match self.tab {
            AppTab::Sheet => {
                // if there's a choice going, render it
                match peek_choice!(self) {
                    Some(Choice::Selection(s)) => self.choice_window(ctx, s),
                    Some(Choice::String(t)) => self.trait_window(ctx, t),
                    _ => {}
                }

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
