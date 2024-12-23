use crate::Action;
use crate::iced::Length;
use cosmic::widget::button::{Button, custom_image_button};
use cosmic::widget::{
    svg,
    svg::{Catalog, Handle, Svg},
};

pub const ROTATE_LEFT: &[u8] = include_bytes!("../vendor/MaterialDesign/svg/rotate-left.svg");
pub const ROTATE_RIGHT: &[u8] = include_bytes!("../vendor/MaterialDesign/svg/rotate-right.svg");

pub fn icon<'a, Theme: Catalog>(b: &[u8]) -> Svg<'a, Theme> {
    svg(Handle::from_memory(b.to_owned()))
}

pub fn icon_button(b: &[u8], size: f32) -> Button<Action> {
    custom_image_button(icon::<cosmic::Theme>(b), None)
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
}
