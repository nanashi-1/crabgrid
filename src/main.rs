use anyhow::Context;
use clap::Parser;
use crabgrid::canvas::{Canvas, Columns, Dimensions, Gaps, ImageCount};
use std::path::Path;

#[derive(Parser, Debug, Clone)]
#[command(version, about = "Creates a grid collage that is very nifty for documentation in lab experiments.", long_about = None)]
struct Cli {
    /// Number of columns in the collage grid.
    #[arg(short, long, default_value_t = 5)]
    columns: u32,

    /// The dimensions of every image in the collage.
    #[arg(short, long, default_value_t = "300x300".into())]
    dimension: String,

    /// Gaps between images and the padding around the entire collage.
    #[arg(short, long, default_value_t = 2)]
    gaps: u32,

    /// Compile multiple directories at once.
    #[arg(short, long)]
    multi_directory: bool,

    /// Input directory. Or directory of multiple input directories in the case of multi-directory mode.
    input: String,

    /// Output file. Or output directory in the case of multi-directory mode.
    output: String,
}

fn compile_images(cli: &Cli, input: &Path, output: &Path) -> anyhow::Result<()> {
    let directory = Path::new(input);

    let mut image_list: Vec<_> = std::fs::read_dir(directory)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| is_image_file(path))
        .collect();

    image_list.sort();

    let dimen = cli.dimension.to_uppercase().clone();
    let mut split_dimen = dimen.split("X");
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

    for image_path in image_list {
        let image = image::open(image_path).unwrap();

        canvas.append_image(&image);
    }

    canvas.write_to_file(output).unwrap();

    Ok(())
}

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

        main_directory
            .read_dir()?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                if let Ok(file_type) = entry.file_type() {
                    return file_type.is_dir();
                }
                false
            })
            .for_each(|directory| {
                compile_images(
                    &cli,
                    &directory.path(),
                    Path::new(&format!(
                        "{}/{}.jpg",
                        &cli.output,
                        directory.file_name().into_string().unwrap()
                    )),
                )
                .unwrap();
            });

        return Ok(());
    }

    compile_images(&cli, Path::new(&cli.input), Path::new(&cli.output))?;

    Ok(())
}
