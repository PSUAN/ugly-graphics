use std::fs::File;
use std::{env, io};

use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageBuffer, ImageEncoder, Rgb};
use ugly_graphics::image::image_adapter::Adapter;
use ugly_graphics::operation::pixel::Pixel;
use ugly_graphics::operation::scanline::triangle::filled::overlapping::OverlappingTriangle;
use ugly_graphics::operation::scanline::triangle::outline::OutlineTriangle;
use ugly_graphics::painter::Painter;
use ugly_graphics::strategy::IntoApply;

fn main() -> io::Result<()> {
    let mut image = ImageBuffer::new(320, 320);
    let mut adapter = Adapter::new(&mut image);
    let mut painter = Painter::new(&mut adapter);

    let triangle = [(10, 210), (160, 300), (310, 20)];
    painter.draw(OverlappingTriangle::new(
        triangle,
        (|mut v: Rgb<u8>| {
            v.0[0] += 0x70;
            v
        })
        .apply(),
    ));
    painter.draw(OutlineTriangle::new(
        triangle,
        (|mut v: Rgb<u8>| {
            v.0[1] += 0x70;
            v
        })
        .apply(),
    ));

    for pixel in triangle {
        painter.draw(Pixel::new(
            pixel,
            (|mut v: Rgb<u8>| {
                v.0[2] += 0x70;
                v
            })
            .apply(),
        ));
    }

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
