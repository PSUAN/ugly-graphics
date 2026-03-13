use std::fs::File;
use std::{env, io};

use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageBuffer, ImageEncoder, Rgb};
use ugly::image::image_adapter::Adapter;
use ugly::operation::pixel::Pixel;
use ugly::operation::scanline::line::Line;
use ugly::operation::scanline::triangle::filled::overlapping::OverlappingTriangle;
use ugly::painter::Painter;
use ugly::strategy::IntoOverwrite;

fn main() -> io::Result<()> {
    let mut image = ImageBuffer::new(32, 32);
    let mut adapter = Adapter::new(&mut image);
    let mut painter = Painter::new(&mut adapter);

    painter.draw(Line::new(
        (0, 0),
        (31, 15),
        Rgb([0xff, 0x80, 0x00]).overwrite(),
    ));
    painter.draw(OverlappingTriangle::new(
        [(1, 1), (15, 13), (4, 31)],
        Rgb([0xff, 0x00, 0x00]).overwrite(),
    ));
    painter.draw(Pixel::new((1, 1), Rgb([0x00, 0x00, 0xff]).overwrite()));
    painter.draw(Pixel::new((15, 13), Rgb([0x00, 0x00, 0xff]).overwrite()));
    painter.draw(Pixel::new((4, 31), Rgb([0x00, 0x00, 0xff]).overwrite()));

    let mut path = env::current_exe()?;
    path.set_extension("png");
    let file = File::create(&path)?;

    let encoder = PngEncoder::new(file);
    encoder
        .write_image(
            &image,
            image.width(),
            image.height(),
            ExtendedColorType::Rgb8,
        )
        .map_err(io::Error::other)?;

    println!("Generated image at {}", path.display());

    Ok(())
}
