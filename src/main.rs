//! CLI tool entrypoint for compiling directory/directories of images into grid collages.

use anyhow::Context;
use clap::Parser;
use crabgrid::canvas::{Canvas, Columns, Dimensions, Gaps, ImageCount};
use std::path::Path;

/// Command-line arguments for the `crabgrid` binary.
#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "Creates a grid collage suitable for visual documentation and lab experiments.",
    long_about = None
)]
struct Cli {
    /// Number of columns in the collage grid.
    #[arg(short, long, default_value_t = 5)]
    columns: u32,

    /// The target dimensions for every image cell in the collage (e.g., "300x300").
    #[arg(short, long, default_value_t = "300x300".into())]
    dimension: String,

    /// Gaps between images and padding around the collage, in pixels.
    #[arg(short, long, default_value_t = 2)]
    gaps: u32,

    /// Compile multiple subdirectories inside the input directory at once.
    #[arg(short, long)]
    multi_directory: bool,

    /// Path to the input directory (or the parent directory when running in multi-directory mode).
    input: String,

    /// Path to the output file (or the destination folder when running in multi-directory mode).
    output: String,
}

/// Reads image files from the specified directory, resizes them, arranges them into
/// a grid layout, and saves the final collage to the output path.
fn compile_images(cli: &Cli, input: &Path, output: &Path) -> anyhow::Result<()> {
    // Collect all supported image files in the directory and sort them alphabetically.
    let mut image_list: Vec<_> = std::fs::read_dir(input)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| is_image_file(path))
        .collect();

    image_list.sort();

    // Parse the image dimension string (e.g. "300X300").
    let dimen = cli.dimension.to_uppercase();
    let mut split_dimen = dimen.split('X');
    let width = split_dimen
        .next()
        .context("Invalid width dimension")?
        .parse()?;
    let height = split_dimen
        .next()
        .context("Invalid height dimension")?
        .parse()?;

    let mut canvas = Canvas::new(
        Columns(cli.columns),
        Dimensions { width, height },
        ImageCount(image_list.len() as u32),
        Gaps(cli.gaps),
    );

    // Append each image to the canvas grid sequentially.
    for image_path in image_list {
        let image = image::open(image_path).context("Failed to open image file")?;
        canvas.append_image(&image);
    }

    // Write the compiled grid collage to the filesystem.
    canvas.write_to_file(output).context("Failed to save the collage image")?;

    Ok(())
}

/// Helper function to determine if a file is a supported image type based on its extension.
fn is_image_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext_str| {
            let lower = ext_str.to_lowercase();
            matches!(lower.as_str(), "jpg" | "jpeg" | "png" | "webp")
        })
        .unwrap_or(false)
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    if cli.multi_directory {
        let main_directory = Path::new(&cli.input);

        // Iterate through all subdirectories in the main directory.
        for entry in main_directory.read_dir()? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let dir_path = entry.path();
                let dir_name = entry.file_name().into_string().unwrap_or_default();
                let output_path = Path::new(&cli.output).join(format!("{}.jpg", dir_name));
                
                compile_images(&cli, &dir_path, &output_path)?;
            }
        }

        return Ok(());
    }

    compile_images(&cli, Path::new(&cli.input), Path::new(&cli.output))?;

    Ok(())
}
