use std::fs::File;
use std::{env, io};

use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageBuffer, ImageEncoder, Rgb};
use ugly::image::image_adapter::Adapter;
use ugly::operation::scanline::triangle::filled::overlapping::OverlappingTriangle;
use ugly::operation::scanline::triangle::outline::OutlineTriangle;
use ugly::painter::Painter;
use ugly::strategy::{IntoApply, IntoOverwrite};

fn main() -> io::Result<()> {
    let mut image = ImageBuffer::new(320, 320);
    let mut adapter = Adapter::new(&mut image);
    let mut painter = Painter::new(&mut adapter);

    let triangle = [(10, 10), (160, 130), (40, 310)];
    painter.draw(OverlappingTriangle::new(
        triangle,
        Rgb([0x80, 0x00, 0x00]).overwrite(),
    ));
    painter.draw(OutlineTriangle::new(
        triangle,
        (|mut v: Rgb<u8>| {
            v.0[1] += 0x40;
            v
        })
        .apply(),
    ));

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

    eprintln!("Generated image at {}", path.display());
    eprintln!("You can pass the output of this example as a path to some viewer...");
    eprintln!();
    println!("{}", path.display());

    Ok(())
}
