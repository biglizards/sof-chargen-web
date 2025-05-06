mod backend;
mod char_sheet;
mod save;
mod sidebar;
mod util;

use iced::font::Family;
use iced::widget::Row;
use iced::{Font, Settings, Theme};
use sof_chargen::event::Event;
use sof_chargen::event::{birth, scenarios};
use sof_chargen::ipc::Choice;
use sof_chargen::{Backend, Character, event};
use std::borrow::Cow;

fn load_fonts() -> Vec<Cow<'static, [u8]>> {
    vec![
        include_bytes!("../Roboto_Mono/static/RobotoMono-Regular.ttf")
            .as_slice()
            .into(),
        include_bytes!("../Roboto_Mono/static/RobotoMono-Bold.ttf")
            .as_slice()
            .into(),
    ]
}

pub fn main() -> iced::Result {
    println!("starting...");
    let settings = Settings {
        fonts: load_fonts(),
        default_font: Font {
            family: Family::Name("Roboto Mono"),
            ..Default::default()
        },
        default_text_size: 20.0.into(),
        ..Default::default()
    };

    iced::application("App Title", App::update, App::view)
        .theme(App::theme)
        .settings(settings)
        .window_size((1500.0, 600.0))
        .run()
}

#[derive(Default)]
struct App {
    current_event: Option<Box<dyn Event>>,
    current_choice: Option<Choice>,

    trait_entry: String,
    dice_slider: i16,
}

#[derive(Debug, Clone)]
enum Message {
    NameChanged(String),
    Choose(usize),
    SubmitTrait(String),
    PickRoll(Option<i8>),
    SliderChanged(i16),
    QuestionAnswer(bool),
    ResetAll,
    // events
    RollStats,
    PickStar,
    RollLocation,
    RollCareers,
    DebugSlider,
    DebugScenario(i8),
    AdvanceLifeStage,
}

impl Message {
    pub(crate) fn should_advance(&self) -> bool {
        matches!(
            self,
            // these are the gui events corresponding to IPCs
            // ie thees ones should cause the event iter to advance now that we're done responding
            Self::Choose(_) | Self::SubmitTrait(_) | Self::PickRoll(_) | Self::QuestionAnswer(_)
        )
    }
}

impl App {
    fn update(&mut self, message: Message) {
        let should_advance = message.should_advance();
        let backend = &*save::BACKEND;

        match message {
            Message::NameChanged(name) => backend.get_character_mut().name = name,
            Message::Choose(i) => match &self.current_choice {
                Some(Choice::Selection(s)) => s.chosen.set(i),
                _ => panic!("attempted to choose when there is no choice!"),
            },
            Message::SubmitTrait(submission) => {
                if self.current_event.is_some() {
                    println!("TODO: trait submitted, do something {submission}");
                    // self.log_choice(&submission);
                }

                match &self.current_choice {
                    Some(Choice::String(t)) => t.chosen.set(submission),
                    _ => panic!("attempted to choose when there is no choice!"),
                }
            }
            Message::PickRoll(choice) => match &self.current_choice {
                Some(Choice::PickRoll(p)) => p.chosen.set(choice),
                _ => panic!("attempted to pick roll when there is no choice!"),
            },
            Message::QuestionAnswer(a) => match &self.current_choice {
                Some(Choice::Question(q)) => q.chosen.set(a),
                _ => panic!("attempted to answer a question when none were posed!"),
            },
            Message::ResetAll => {
                *backend.get_character_mut() = Character::default();
                backend.log.borrow_mut().clear();
            }
            Message::RollStats => {
                self.current_event = Some(Box::new(birth::roll_core_stats(backend)));
            }
            Message::PickStar => self.current_event = Some(Box::new(birth::pick_omens(backend))),
            Message::RollLocation => birth::roll_location_of_birth(backend),
            Message::RollCareers => {
                self.current_event = Some(Box::new(birth::affiliation_rank_careers(backend)))
            }
            Message::SliderChanged(v) => self.dice_slider = v,
            Message::DebugSlider => {
                self.current_event = Some(Box::new(event::test_pick_dice(backend)))
            }
            Message::DebugScenario(i) => {
                self.current_choice = None;
                match i {
                    1 => self.current_event = Some(Box::from(scenarios::kremish_accorder(backend))),
                    2 => {
                        self.current_event =
                            Some(Box::from(scenarios::non_kremish_accorder(backend)))
                    }
                    _ => println!("invalid debug scenario!"),
                }
            }
            Message::AdvanceLifeStage => {
                self.current_choice = None;
                self.current_event = backend.next_stage();
            }
        }

        if should_advance || (self.current_choice.is_none() && self.current_event.is_some()) {
            self.advance_event();
        }

        if self.current_event.is_none() {
            save::save_backend();
        }
    }

    fn advance_event(&mut self) {
        self.current_choice = None;
        self.current_choice = self.current_event.as_mut().unwrap().next();
        if self.current_choice.is_none() {
            self.current_event = None;
        }
    }

    fn view(&self) -> Row<Message> {
        iced::widget::row! {
            char_sheet::char_sheet(&save::BACKEND),
            self.sidebar(&save::BACKEND),
        }
    }

    fn theme(&self) -> Theme {
        let Some(theme) = Theme::ALL.get(1usize) else {
            return Theme::Dark;
        };
        theme.clone()
    }
}
