use cosmic::{Element, iced, widget};
use iced::{Alignment, Length};

use crate::{Action, icons::*};

pub fn window_welcome<'a>() -> Element<'a, Action> {
    let mut column = widget::column()
        .align_x(Alignment::Center)
        .padding(20)
        .spacing(20);

    let column = column.push(widget::text::title1("Sticker Printer"));

    let mut row = widget::row()
        .align_y(Alignment::Center)
        .padding(20)
        .spacing(20)
        .push(
            icon(IMAGE_AREA)
                .height(Length::Fixed(128.0))
                .width(Length::Shrink)
                .symbolic(true),
        )
        .push(
            widget::text("  Drop a file here")
                .size(32)
                .width(Length::Shrink),
        );
    let mut drop_container = iced::widget::container(row).style(|theme| widget::container::Style {
        border: cosmic::iced_winit::graphics::core::border::rounded(10.0)
            .color(iced::Color::WHITE)
            .width(1),
        ..widget::container::Style::default()
    });

    let column = column
        .push(drop_container)
        .push(widget::button::suggested("Load image").on_press(Action::SelectImage));

    let mut screen_container = iced::widget::container(column)
        .padding(20)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill);
    screen_container.into()
}
