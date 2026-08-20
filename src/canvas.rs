use std::path::Path;

use image::{DynamicImage, RgbImage};

use crate::error::CanvasError;
#[derive(Debug)]
pub struct Canvas {
    canvas: RgbImage,
    image_dimensions: Dimensions,
    gaps: Gaps,
    columns: Columns,
    current_image: u32,
}

#[derive(Debug, PartialEq)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct Gaps(pub u32);

#[derive(Debug)]
pub struct ImageCount(pub u32);

#[derive(Debug)]
pub struct Columns(pub u32);

#[derive(Debug)]
pub struct Rows(pub u32);

impl Canvas {
    pub fn new(
        columns: Columns,
        image_dimensions: Dimensions,
        image_count: ImageCount,
        gaps: Gaps,
    ) -> Self {
        let canvas_width = columns.0 * (image_dimensions.width + gaps.0) + gaps.0;
        let rows = image_count.0.div_ceil(columns.0);
        let canvas_height = rows * (image_dimensions.height + gaps.0) + gaps.0;

        Self {
            canvas: RgbImage::from_pixel(canvas_width, canvas_height, image::Rgb([255, 255, 255])),
            image_dimensions,
            gaps,
            columns,
            current_image: 0,
        }
    }

    pub fn get_dimensions(&self) -> Dimensions {
        Dimensions {
            width: self.canvas.width(),
            height: self.canvas.height(),
        }
    }

    pub fn append_image(&mut self, image: &DynamicImage) {
        let column = self.current_image % self.columns.0;
        let row = self.current_image / self.columns.0;

        self.insert_image(image, Columns(column), Rows(row));

        self.current_image += 1;
    }

    pub fn insert_image(&mut self, image: &DynamicImage, column: Columns, row: Rows) {
        let image = image.resize_exact(
            self.image_dimensions.width,
            self.image_dimensions.height,
            image::imageops::FilterType::Nearest,
        );

        let x = self.gaps.0 + column.0 * (self.image_dimensions.width + self.gaps.0);
        let y = self.gaps.0 + row.0 * (self.image_dimensions.height + self.gaps.0);

        image::imageops::replace(&mut self.canvas, &image.to_rgb8(), x.into(), y.into());
    }

    pub fn write_to_file<Q: AsRef<Path>>(&self, path: Q) -> Result<(), CanvasError> {
        Ok(self.canvas.save(path)?)
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
        )
    }
}
