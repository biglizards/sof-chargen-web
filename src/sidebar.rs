use crate::{App, Message, util};
use iced::Length;
use iced::widget::{Column, button, horizontal_rule, row, slider, text, vertical_rule};
use iced::widget::{column, text_input};
use sof_chargen::Backend;
use sof_chargen::ipc::Choice;
use sof_chargen::ipc::Choice::Selection;

impl App {
    fn choice_input(&self) -> Column<Message> {
        println!("doing choice input...");
        match &self.current_choice {
            None => column![text("No choice. Press a button.")],
            Some(Choice::String(s)) => column![
                text(s.description),
                text_input("type trait here", &self.trait_entry).on_input(Message::SubmitTrait)
            ],
            Some(Selection(s)) => {
                let x = column![
                    text(s.description),
                    util::row(
                        s.options
                            .iter()
                            .enumerate()
                            .map(|(i, c)| button(&*c.description).on_press(Message::Choose(i))),
                    )
                    .spacing(5)
                    .wrap()
                ];
                println!(
                    "selection is {} [{:?}]",
                    s.description,
                    s.options
                        .iter()
                        .map(|x| &x.description)
                        .collect::<Vec<&String>>()
                );
                x
            }
            Some(Choice::PickRoll(r)) => {
                // annoying iced quirk: sliders require T: From<u8> which precludes using i8
                let range = r.roll.range().into_inner();
                let range = (range.0 as i16)..=(range.1 as i16);
                let value = if range.contains(&self.dice_slider) {
                    self.dice_slider
                } else {
                    *range.start()
                };
                column![
                    text(r.description),
                    row![
                        button("Roll Randomly").on_press(Message::PickRoll(None)),
                        vertical_rule(1),
                        slider(range, value, Message::SliderChanged),
                        button(text(format!("Pick {}", value)))
                            .on_press(Message::PickRoll(Some(value as i8))),
                    ]
                    .height(Length::Shrink)
                    .spacing(5)
                ]
            }
            Some(Choice::Question(q)) => column![
                text(&q.description),
                row![
                    button("Yes").on_press(Message::QuestionAnswer(true)),
                    button("No").on_press(Message::QuestionAnswer(false)),
                ]
                .spacing(5)
            ],
        }
    }

    pub(crate) fn sidebar(&self, backend: &Backend) -> Column<Message> {
        column!(
            text(&self.log).size(16),
            horizontal_rule(1),
            self.choice_input().padding([20, 0])
        )
    }
}
