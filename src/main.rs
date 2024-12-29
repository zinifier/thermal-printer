use std::boxed::Box;

const WINDOW_TITLE: &'static str = "StickerPrinter";

mod backends;
#[cfg(feature = "cosmic")]
pub use backends::cosmic as backend;
#[cfg(feature = "egui")]
pub use backends::egui as backend;

pub mod icons;
mod state;
pub use state::*;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    #[cfg(feature = "cosmic")]
    backend::main()?;

    #[cfg(not(feature = "cosmic"))]
    backend::main()?;

    Ok(())
}
