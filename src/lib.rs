//! `crabgrid` is a command-line tool and library that compiles multiple images into a clean, uniform grid collage.
//! It is particularly useful for generating visual summaries, cataloging experiment results, or creating structured layouts
//! for lab reports and documentation.
//!
//! # Features
//! - **Grid Collage Generation**: Automatically grids any set of images (`.jpg`, `.jpeg`, `.png`, `.webp`) from an input directory.
//! - **Customizable Layouts**: Define the number of columns, the dimensions of individual images, and the spacing (gaps) between them.
//! - **Multi-Directory Mode**: Process multiple subdirectories at once, producing a separate grid collage for each.
//! - **Safe Handling**: Implements clean error propagation using `thiserror` and `anyhow`.
//!
//! # Examples
//!
//! ```rust
//! use crabgrid::canvas::{Canvas, Columns, Dimensions, Gaps, ImageCount};
//! use image::DynamicImage;
//!
//! // Create a new 5-column canvas for 21 images, each resized to 300x300 pixels,
//! // with 2-pixel gaps between images and as outer padding.
//! let mut canvas = Canvas::new(
//!     Columns(5),
//!     Dimensions { width: 300, height: 300 },
//!     ImageCount(21),
//!     Gaps(2),
//! );
//!
//! // You can append images to the grid sequentially:
//! // let img = image::open("path/to/image.png").unwrap();
//! // canvas.append_image(&img);
//! // canvas.write_to_file("output.jpg").unwrap();
//! ```

pub mod canvas;
pub mod error;
