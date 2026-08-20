#[derive(thiserror::Error, Debug)]
pub enum CanvasError {
    #[error("File write error: {0}")]
    FileWrite(#[from] image::ImageError),
}
