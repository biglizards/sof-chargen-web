use crate::SoFCharGenApp;
use crate::app::AppTab;
use crate::app::backend::BACKEND;
use crate::gui_event::GUIEvent;
use egui::{Layout, RichText, Slider, Ui};
use sof_chargen::ipc::Selection;
use sof_chargen::ipc::{Choice, PickRoll};
use sof_chargen::{CORE_STATS, Character, Stat, event};

impl SoFCharGenApp {
    fn choose(&self, i: usize) {
        self.current_gui_event.set(Some(GUIEvent::Choose(i)));
    }

    fn submit_trait(&mut self) {
        self.current_gui_event
            .set(Some(GUIEvent::SubmitTrait(std::mem::take(
                &mut self.trait_submission,
            ))));
    }

    fn pick_roll(&self, choice: i8) {
        self.current_gui_event.set(Some(GUIEvent::PickRoll(choice)));
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
    fn trait_window(&mut self, ctx: &egui::Context, description: &'static str) {
        // if we're requesting a trait, put that up in front of the choice buttons
        egui::Window::new("Gain a Trait").show(ctx, |ui| {
            ui.label(description);
            self.trait_buttons(ui);
        });
    }

    fn choice_buttons(&self, ui: &mut Ui, choice: &Selection) {
        ui.horizontal_wrapped(|ui| {
            for (i, option) in choice.options.iter().enumerate() {
                if ui.button(&option.description).clicked() {
                    self.log_choice(&option.description);
                    self.choose(i);
                }
            }
        });
    }
    fn choice_window(&self, ctx: &egui::Context, s: &Selection) {
        egui::Window::new("Choice").show(ctx, |ui| {
            ui.label(s.description);
            self.choice_buttons(ui, s);
        });
    }

    fn pick_roll_slider(&self, ui: &mut Ui, p: &PickRoll) {
        let mut value = self.picking_roll.get();
        ui.add(Slider::new(&mut value, p.roll.range()).text(p.description));
        self.picking_roll.set(value);
        ui.horizontal(|ui| {
            if ui.button("Submit").clicked() {
                self.pick_roll(value);
            } else if ui.button("Roll").clicked() {
                self.pick_roll(p.roll.result());
            }
        });
    }
    fn pick_roll_window(&self, ctx: &egui::Context, p: &PickRoll) {
        egui::Window::new("Pick Roll").show(ctx, |ui| {
            self.pick_roll_slider(ui, p);
        });
    }

    fn stats(&self, ui: &mut egui::Ui) {
        // first row: name, [blank], luck, magic
        ui.columns(4, |columns| {
            columns[0].text_edit_singleline(&mut BACKEND.character.borrow_mut().name);
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
        let traits = &BACKEND.character.borrow().traits;
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
            self.current_event = Some(Box::new(event::roll_core_stats(&*BACKEND)));
        }
        if ui.button("Roll Magic and Luck").clicked() {
            event::roll_magic(&*BACKEND);
            event::roll_luck(&*BACKEND);
        }
        if ui.button("Reset").clicked() {
            *BACKEND.character.borrow_mut() = Character {
                stats: Default::default(),
                name: "Enter Name".to_string(),
                traits: vec![],
            };
            self.reset_log();
        }
        if ui.button("Pick a Star").clicked() {
            self.current_event = Some(Box::new(event::prosperous_constellations(&*BACKEND)));
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
                        ui.label(&*BACKEND.log.borrow());
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
                        Some(Choice::PickRoll(p)) => {
                            ui.label(p.description);
                        }
                        None => {}
                    }
                });
        });
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        self.tab = self.render_top_panel(ctx);

        match self.tab {
            AppTab::Sheet => {
                // if there's a choice going, render it
                match &self.current_choice {
                    Some(Choice::Selection(s)) => self.choice_window(ctx, s),
                    Some(Choice::String(t)) => self.trait_window(ctx, t.description),
                    Some(Choice::PickRoll(p)) => self.pick_roll_window(ctx, p),
                    None => {}
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
