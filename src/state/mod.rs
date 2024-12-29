mod action;
pub use action::Action;
mod image;
pub use image::Image;

#[derive(Clone)]
pub struct AppState {
    #[cfg(feature = "cosmic")]
    pub core: cosmic::app::Core,
    pub image: Image,
    pub preview: bool,
}
