use sticker_printer::{FileType, Rotation, Sticker};

use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum Image {
    None,
    Loading {
        path: PathBuf,
    },
    Loaded {
        path: PathBuf,
        data: Vec<u8>,
        sticker: Sticker,
    },
    Errored {
        path: PathBuf,
        error: String,
    },
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
                        }
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
            &mut Self::Loaded {
                path: _,
                data: _,
                ref mut sticker,
            } => sticker.rotate(direction),
            _ => unreachable!(),
        }
    }

    pub fn sticker(&self) -> &Sticker {
        match self {
            Self::Loaded {
                path: _,
                data: _,
                sticker,
            } => sticker,
            _ => unreachable!(),
        }
    }
}
