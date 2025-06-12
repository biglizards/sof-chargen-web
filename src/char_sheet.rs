use crate::Message;
use crate::util::{column, present, row};
use iced::widget::{Column, button, horizontal_rule, text_input};
use iced::widget::{Row, text};
use iced::{Length, Padding, font};
use sof_chargen::Backend;
use sof_chargen::CORE_STATS;
use sof_chargen::Stat::{Luck, Magic, Stamina};
use std::iter::once;

pub fn stats(backend: &Backend) -> Row<Message> {
    row(CORE_STATS.map(|stat| {
        row([
            column(
                once(text(stat.to_string()).font(iced::Font {
                    weight: font::Weight::Bold,
                    family: font::Family::Name("Roboto Mono"),
                    ..Default::default()
                }))
                .chain(stat.subskills().iter().map(|&x| text(x.to_string()))),
            )
            .padding(Padding {
                right: 20.0,
                ..Default::default()
            }),
            column(
                once(text(backend.get_stat(stat).unwrap_or_default())).chain(
                    stat.subskills()
                        .iter()
                        .map(|&x| text(backend.get_stat(x).unwrap_or_default())),
                ),
            ),
        ])
        .width(Length::Fill)
    }))
}

fn top_row(backend: &Backend) -> Row<Message> {
    iced::widget::row![
        iced::widget::row![
            text_input("character name", &backend.character().name).on_input(Message::NameChanged)
        ]
        .width(Length::FillPortion(3)),
        present("Stamina", backend.get_stat(Stamina)).width(Length::FillPortion(2)),
        present("Magic", backend.get_stat(Magic)).width(Length::FillPortion(2)),
        present("Luck", backend.get_stat(Luck)).width(Length::FillPortion(2)),
    ]
    .width(Length::Fill)
    .spacing(8)
}

fn culture_row(backend: &Backend) -> Row<Message> {
    let char = backend.character();
    iced::widget::row![
        present("Born", char.birth_location.as_ref().map(|l| &l.name))
            .width(Length::FillPortion(3)),
        present("Culture", char.culture).width(Length::FillPortion(2)),
        present("Faith", char.faith).width(Length::FillPortion(2)),
        present("Omen", char.omen).width(Length::FillPortion(2)),
    ]
    .width(Length::Fill)
    .spacing(8)
}

fn debug_buttons() -> Row<'static, Message> {
    iced::widget::row![
        button("Roll stats").on_press(Message::RollStats),
        button("Roll location").on_press(Message::RollLocation),
        button("Roll careers").on_press(Message::RollCareers),
        button("Pick star").on_press(Message::PickStar),
        button("Reset").on_press(Message::ResetAll),
        button("Slider").on_press(Message::DebugSlider),
        button("Debug 1").on_press(Message::DebugScenario(1)),
        button("Debug 2").on_press(Message::DebugScenario(2)),
        button("Debug 3").on_press(Message::DebugScenario(3)),
        button("Debug 4").on_press(Message::DebugScenario(4)),
    ]
}

pub fn char_sheet(backend: &Backend) -> Column<Message> {
    iced::widget::column! {
        top_row(backend),
        culture_row(backend),
        horizontal_rule(1),
        stats(backend),
        horizontal_rule(1),
        debug_buttons().padding(5).spacing(5).wrap(),
    }
    .width(20 * 10 * 5)
}
