use cosmic::iced_winit::graphics::core::{Radians, Rotation};
use cosmic::{
    Element,
    app::{Core, Message, Settings, Task},
    executor, iced, widget,
};
use iced::{Alignment, Color, Length, Subscription, keyboard};
use rfd::FileDialog;

use std::boxed::Box;
use std::path::{Path, PathBuf};

pub mod icons;
use icons::*;

#[derive(Debug, Clone)]
pub enum Action {
    SelectImage,
    LoadImage(PathBuf),
    LoadedImage(Result<Vec<u8>, String>),
    Rotate(Direction),
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub enum Image {
    None,
    Loading { path: PathBuf },
    Loaded { path: PathBuf, data: Vec<u8> },
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
                Ok(data) => Self::Loaded {
                    path: path.to_path_buf(),
                    data,
                },
                Err(error) => Self::Errored {
                    path: path.to_path_buf(),
                    error,
                },
            },
            _ => {
                unreachable!();
            }
        }
    }
}

#[derive(Clone)]
pub struct StickerPrinter {
    core: Core,
    image: Image,
    rotation: Rotation,
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
            rotation: Rotation::Solid(Radians(0.0)),
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
                let angle = match direction {
                    Direction::Left => Radians::PI / -2.0,
                    Direction::Right => Radians::PI / 2.0,
                };
                self.rotation = Rotation::Solid(self.rotation.radians() + angle);
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
            Image::Loaded { path, data } => {
                content = content
                    .push(widget::button::text("Load new image").on_press(Action::SelectImage))
                    .push(widget::text(format!("{}", path.display())))
                    .push(
                        widget::row()
                            .push(
                                icon_button(ROTATE_LEFT, 40.0)
                                    .on_press(Action::Rotate(Direction::Left)),
                            )
                            .push(
                                icon_button(ROTATE_RIGHT, 40.0)
                                    .on_press(Action::Rotate(Direction::Right)),
                            ),
                    )
                    .push(
                        widget::container(
                            widget::image(iced::widget::image::Handle::from_bytes(data.clone()))
                                .rotation(self.rotation),
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
                (key::Named::ArrowLeft, true) => Some(Action::Rotate(Direction::Left)),
                (key::Named::ArrowRight, true) => Some(Action::Rotate(Direction::Right)),
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
