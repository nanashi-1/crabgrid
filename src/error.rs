//! Error types for the `crabgrid` library operations.

/// Errors that can occur during canvas operations, such as saving the final image.
#[derive(thiserror::Error, Debug)]
pub enum CanvasError {
    /// An error occurred while writing the image to disk.
    #[error("File write error: {0}")]
    FileWrite(#[from] image::ImageError),
}
