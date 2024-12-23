use cosmic::{
    Element,
    app::{Core, Message, Settings, Task},
    executor, iced, widget,
};
use iced::{Alignment, Color, Length, Subscription, keyboard};
use rfd::FileDialog;
use sticker_printer::{FileType, Rotation, Sticker, print};

use std::boxed::Box;
use std::path::{Path, PathBuf};

pub mod icons;
use icons::*;

#[derive(Debug, Clone)]
pub enum Action {
    SelectImage,
    TogglePreview,
    LoadImage(PathBuf),
    LoadedImage(Result<Vec<u8>, String>),
    Rotate(Rotation),
    PrintSticker(u8),
}

#[derive(Debug, Clone)]
pub enum Image {
    None,
    Loading { path: PathBuf },
    Loaded { path: PathBuf, data: Vec<u8>, sticker: Sticker },
    Errored { path: PathBuf, error: String },
}

impl Image {
    pub fn none() -> Self {
        Self::None
    }

    pub fn loading(&mut self, path: impl AsRef<Path>) {
        *self = Self::Loading {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn loaded(&mut self, res: Result<Vec<u8>, String>) {
        *self = match self {
            Self::Loading { path } => match res {
                Ok(data) => match FileType::from_ext(&path) {
                    Ok(filetype) => match Sticker::from_bytes(data.clone(), filetype) {
                        Ok(mut sticker) => {
                            // Apply initial operations here
                            sticker.dither();

                            Self::Loaded {
                                path: path.to_path_buf(),
                                data,
                                sticker,
                            }
                        },
                        Err(error) => Self::Errored {
                            path: path.to_path_buf(),
                            error: error.to_string(),
                        },
                    },
                    Err(error) => Self::Errored {
                        path: path.to_path_buf(),
                        error: error.to_string(),
                    },
                },
                Err(error) => Self::Errored {
                    path: path.to_path_buf(),
                    error: error.to_string(),
                },
            },
            _ => {
                unreachable!();
            }
        }
    }

    pub fn rotate(&mut self, direction: Rotation) {
        match self {
            &mut Self::Loaded { path: _, data: _, ref mut sticker } => sticker.rotate(direction),
            _ => unreachable!(),
        }
    }

    pub fn to_widget(&self, preview: bool) -> widget::Image {
        use iced::widget::image::Handle;
        use sticker_printer::image::codecs::png::PngEncoder;
        use std::io::BufWriter;

        let mut buffer = BufWriter::new(Vec::<u8>::new());
        let encoder = PngEncoder::new(&mut buffer);
        match self {
            Self::Loaded { path: _, data: _, sticker } => {
                if preview {
                    sticker.transformed.write_with_encoder(encoder).unwrap();
                } else {
                    sticker.raw.write_with_encoder(encoder).unwrap();
                }
            }
            _ => unreachable!(),
        };

        let handle = Handle::from_bytes(buffer.into_inner().unwrap());
        widget::image(handle)
    }

    pub fn sticker(&self) -> &Sticker {
        match self {
            Self::Loaded { path: _, data: _, sticker } => sticker,
            _ => unreachable!()
        }
    }
}

#[derive(Clone)]
pub struct StickerPrinter {
    core: Core,
    image: Image,
    preview: bool,
}

impl cosmic::Application for StickerPrinter {
    /// Default async executor to use with the app.
    type Executor = executor::Default;

    /// Argument received [`cosmic::Application::new`].
    type Flags = ();

    /// Message type specific to our [`App`].
    type Message = Action;

    /// The unique application ID to supply to the window manager.
    const APP_ID: &'static str = "org.sticker.StickerPrinter";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// Creates the application, and optionally emits task on initialize.
    fn init(core: Core, _input: Self::Flags) -> (Self, Task<Self::Message>) {
        let app = Self {
            core,
            image: Image::None,
            preview: true,
        };

        (app, Task::none())
    }

    fn update(&mut self, message: Action) -> Task<Self::Message> {
        match message {
            Action::SelectImage => {
                if let Some(file) = FileDialog::new()
                    .add_filter("image", &["jpg", "jpeg", "png", "svg"])
                    .pick_file()
                {
                    return Task::done(Message::App(Action::LoadImage(file)));
                } else {
                    return Task::none();
                }
            }
            Action::LoadImage(path) => {
                self.image.loading(&path);
                return Task::perform(
                    async { tokio::fs::read(path).await.map_err(|e| e.to_string()) },
                    |res| Message::App(Action::LoadedImage(res)),
                );
            }
            Action::LoadedImage(res) => {
                self.image.loaded(res);
            }
            Action::Rotate(direction) => {
                self.image.rotate(direction);
            }
            Action::TogglePreview => {
                self.preview = !self.preview;
            }
            Action::PrintSticker(mut n) => {
                let mut lp = std::fs::File::options()
                    .write(true)
                    .append(true)
                    // TODO: configure printer + handle error
                    .open("/dev/usb/lp0").unwrap();

                while n > 0 {
                    n = n - 1;
                    // TODO: error
                    print(&mut lp, self.image.sticker()).unwrap()
                }
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<Action> {
        let mut content = widget::column().spacing(12).align_x(Alignment::Center);

        match &self.image {
            Image::None => {
                content =
                    content.push(widget::button::text("Load image").on_press(Action::SelectImage));
            }
            Image::Errored { path, error } => {
                content = content.push(widget::text(format!(
                    "Failed to load {} due to error: {}",
                    path.display(),
                    error
                )));
                content =
                    content.push(widget::button::text("Load image").on_press(Action::SelectImage));
            }
            Image::Loading { path: _ } => {
                // TODO: loading spinner
            }
            Image::Loaded { path, data: _, sticker: _ } => {
                content = content
                    .push(widget::button::text("Load new image").on_press(Action::SelectImage))
                    .push(widget::text(format!("{}", path.display())))
                    .push(
                        widget::row()
                            .push(
                                icon_button(ROTATE_LEFT, 40.0)
                                    .on_press(Action::Rotate(Rotation::Left)),
                            )
                            .push(
                                icon_button(ROTATE_RIGHT, 40.0)
                                    .on_press(Action::Rotate(Rotation::Right)),
                            ),
                    )
                    .push(
                        widget::checkbox(
                            "Enable greyscale preview",
                            self.preview
                        ).on_toggle(|_| Action::TogglePreview)
                    )
                    .push(
                        widget::button::text("PRINT").on_press(Action::PrintSticker(1))
                    )
                    .push(
                        widget::container(
                            self.image.to_widget(self.preview)
                        )
                        .width(Length::Fixed(720.0))
                        .align_x(Alignment::Center)
                        .padding(5)
                        .style(|_| Color::WHITE.into()),
                    );
            }
        }

        let centered = widget::container(content)
            .width(Length::Fill)
            .height(Length::Shrink)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center);

        Element::from(centered)
    }

    fn subscription(&self) -> Subscription<Action> {
        use keyboard::key;

        keyboard::on_key_press(|key, modifiers| {
            let keyboard::Key::Named(key) = key else {
                return None;
            };

            match (key, modifiers.is_empty()) {
                (key::Named::ArrowLeft, true) => Some(Action::Rotate(Rotation::Left)),
                (key::Named::ArrowRight, true) => Some(Action::Rotate(Rotation::Right)),
                _ => None,
            }
        })
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    cosmic::app::run::<StickerPrinter>(Settings::default(), ())?;
    Ok(())
}
