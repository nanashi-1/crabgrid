use std::path::Path;

use crabgrid::canvas::{Canvas, Columns, Dimensions, ImageCount};

fn main() {
    let directory_target = "./test_files";
    let directory = Path::new(directory_target);
    let mut image_list = vec![];
    for file in directory.read_dir().unwrap() {
        let file = file.unwrap();

        if file.file_name().to_string_lossy().starts_with("Batch") {
            image_list.push(file.path());
        }
    }

    image_list.sort();

    dbg!(&image_list);

    let mut canvas = Canvas::new(
        Columns(5),
        Dimensions {
            width: 300,
            height: 300,
        },
        ImageCount(image_list.len() as u32),
        crabgrid::canvas::Gaps(2),
    );

    for image_path in image_list {
        let image = image::open(image_path).unwrap();

        canvas.append_image(&image);
    }

    canvas.write_to_file("out.jpg").unwrap();
}
