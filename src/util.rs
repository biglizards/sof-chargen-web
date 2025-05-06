use crate::Message;
use iced::widget::{Column, Row, rich_text, span};
use iced::{Font, font};
use std::fmt::Display;

// maybe turn this into a macro?
pub fn row<'a>(
    x: impl IntoIterator<Item = impl Into<iced::Element<'a, Message>>>,
) -> Row<'a, Message> {
    Row::with_children(x.into_iter().map(Into::into))
}

pub fn column<'a>(
    x: impl IntoIterator<Item = impl Into<iced::Element<'a, Message>>>,
) -> Column<'a, Message> {
    Column::with_children(x.into_iter().map(Into::into))
}

pub fn render(thing: Option<impl Display>) -> String {
    match thing {
        None => "-".to_owned(),
        Some(s) => s.to_string(),
    }
}

pub fn present(name: &str, thing: Option<impl Display>) -> iced::widget::text::Rich<Message> {
    rich_text([
        span(name).font(Font {
            weight: font::Weight::Bold,
            ..Font::default()
        }),
        span(": "),
        span(render(thing)),
    ])
}
