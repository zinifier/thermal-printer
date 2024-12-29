use sticker_printer::Rotation;

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Action {
    SelectImage,
    TogglePreview,
    LoadImage(PathBuf),
    LoadedImage(Result<Vec<u8>, String>),
    Rotate(Rotation),
    PrintSticker(u8),
}
