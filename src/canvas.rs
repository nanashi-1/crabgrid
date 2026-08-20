//! Core canvas and layout management for building grid collages.

use std::path::Path;
use image::{DynamicImage, RgbImage};
use crate::error::CanvasError;

/// A canvas that holds the generated grid collage.
///
/// Images are appended one by one to this canvas and laid out in rows and columns
/// separated by a configurable gap size.
#[derive(Debug)]
pub struct Canvas {
    /// The underlying image buffer representing the collage.
    canvas: RgbImage,
    /// The target dimensions for each individual image cell.
    image_dimensions: Dimensions,
    /// The size of the spacing/gaps between cells and outer padding in pixels.
    gaps: Gaps,
    /// The number of columns in the grid.
    columns: Columns,
    /// The index of the next image to be appended (0-based).
    current_image: u32,
}

/// Represents the width and height of an image or canvas.
#[derive(Debug, PartialEq, Eq)]
pub struct Dimensions {
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
}

/// A wrapper type for specifying gap/padding size in pixels.
#[derive(Debug)]
pub struct Gaps(pub u32);

/// A wrapper type for specifying the total count of images to layout.
#[derive(Debug)]
pub struct ImageCount(pub u32);

/// A wrapper type for specifying column indices or grid width in columns.
#[derive(Debug)]
pub struct Columns(pub u32);

/// A wrapper type for specifying row indices or grid height in rows.
#[derive(Debug)]
pub struct Rows(pub u32);

impl Canvas {
    /// Creates a new, blank white canvas sized dynamically to fit the specified number of columns,
    /// target image dimensions, total image count, and gap spacing.
    ///
    /// The width and height calculation includes the padding gaps between cells and an outer border.
    pub fn new(
        columns: Columns,
        image_dimensions: Dimensions,
        image_count: ImageCount,
        gaps: Gaps,
    ) -> Self {
        // Calculate the total width of the canvas:
        // (number of columns * (image width + gap size)) + trailing gap size for the right border.
        let canvas_width = columns.0 * (image_dimensions.width + gaps.0) + gaps.0;

        // Calculate rows needed, rounding up if the count isn't a perfect multiple of columns.
        let rows = image_count.0.div_ceil(columns.0);

        // Calculate the total height of the canvas:
        // (number of rows * (image height + gap size)) + trailing gap size for the bottom border.
        let canvas_height = rows * (image_dimensions.height + gaps.0) + gaps.0;

        // Initialize the canvas with a solid white background.
        let white_pixel = image::Rgb([255, 255, 255]);
        let canvas = RgbImage::from_pixel(canvas_width, canvas_height, white_pixel);

        Self {
            canvas,
            image_dimensions,
            gaps,
            columns,
            current_image: 0,
        }
    }

    /// Returns the overall dimensions of the canvas.
    pub fn get_dimensions(&self) -> Dimensions {
        Dimensions {
            width: self.canvas.width(),
            height: self.canvas.height(),
        }
    }

    /// Appends an image to the next available position in the grid, progressing row-by-row.
    pub fn append_image(&mut self, image: &DynamicImage) {
        let column = self.current_image % self.columns.0;
        let row = self.current_image / self.columns.0;

        self.insert_image(image, Columns(column), Rows(row));
        self.current_image += 1;
    }

    /// Inserts an image at a specific column and row in the grid.
    ///
    /// The image is resized using a nearest-neighbor filter to match the target dimensions.
    pub fn insert_image(&mut self, image: &DynamicImage, column: Columns, row: Rows) {
        let resized_image = image.resize_exact(
            self.image_dimensions.width,
            self.image_dimensions.height,
            image::imageops::FilterType::Nearest,
        );

        // Compute coordinate offsets based on grid gaps and cell positions.
        let x = self.gaps.0 + column.0 * (self.image_dimensions.width + self.gaps.0);
        let y = self.gaps.0 + row.0 * (self.image_dimensions.height + self.gaps.0);

        // Copy the resized image onto the canvas buffer.
        image::imageops::replace(&mut self.canvas, &resized_image.to_rgb8(), x.into(), y.into());
    }

    /// Saves the canvas image to the filesystem at the specified path.
    pub fn write_to_file<Q: AsRef<Path>>(&self, path: Q) -> Result<(), CanvasError> {
        self.canvas.save(path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas_size() {
        let canvas = Canvas::new(
            Columns(5),
            Dimensions {
                width: 500,
                height: 500,
            },
            ImageCount(21),
            Gaps(2),
        );

        assert_eq!(
            Dimensions {
                width: 2512,
                height: 2512
            },
            canvas.get_dimensions()
        );
    }
}
