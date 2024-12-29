use cosmic::{
    iced::Length,
    widget::{
        button::{Button, custom_image_button},
        svg,
        svg::{Catalog, Handle, Svg},
    },
};

use crate::Action;

pub fn icon<'a, Theme: Catalog>(b: &[u8]) -> Svg<'a, Theme> {
    svg(Handle::from_memory(b.to_owned()))
}

pub fn icon_button(b: &[u8], size: f32) -> Button<Action> {
    custom_image_button(icon::<cosmic::Theme>(b), None)
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
}
