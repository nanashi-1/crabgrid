# crabgrid

`crabgrid` is a command-line tool that compiles multiple images into a clean, uniform grid collage. It is particularly useful for generating visual summaries, cataloging experiment results, or creating structured layouts for lab reports and documentation.

## Features

- **Grid Collage Generation**: Automatically grids any set of images (`.jpg`, `.jpeg`, `.png`, `.webp`) from an input directory.
- **Customizable Layouts**: Define the number of columns, the dimensions of individual images, and the spacing (gaps) between them.
- **Multi-Directory Mode**: Process multiple subdirectories at once, producing a separate grid collage for each.
- **Safe Handling**: Implements clean error propagation using `thiserror` and `anyhow`.

## Installation

To build `crabgrid` from source, ensure you have Rust and Cargo installed, then run:

```bash
cargo build --release
```

The compiled binary will be available at `target/release/crabgrid`.

## Usage

### Command-Line Interface

```text
Creates a grid collage that is very nifty for documentation in lab experiments

Usage: crabgrid [OPTIONS] <INPUT> <OUTPUT>

Arguments:
  <INPUT>   Input directory (or parent directory in multi-directory mode)
  <OUTPUT>  Output file path (or output directory in multi-directory mode)

Options:
  -c, --columns <COLUMNS>      Number of columns in the collage grid [default: 5]
  -d, --dimension <DIMENSION>  The dimensions of every image in the collage [default: 300x300]
  -g, --gaps <GAPS>            Gaps between images and the padding around the entire collage [default: 2]
  -m, --multi-directory        Compile multiple directories at once
  -h, --help                   Print help
  -V, --version                Print version
```

### Examples

#### Single Directory Mode

Compile all images inside `./raw_images` into a single collage named `collage.jpg` with a grid layout of 4 columns and resized to `250x250` pixels per cell:

```bash
cargo run -- -c 4 -d 250x250 ./raw_images ./collage.jpg
```

#### Multi-Directory Mode

If you have a parent directory `./experiments` containing multiple subfolders (e.g., `./experiments/exp_1`, `./experiments/exp_2`), you can compile them all in one command. A collage will be created for each subdirectory and saved in `./output_collages`:

```bash
cargo run -- --multi-directory ./experiments ./output_collages
```

## License

This project is licensed under the GPLv3 License. See the `LICENSE` file for details.
