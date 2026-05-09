use std::fs::File;
use std::{env, io};

use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageBuffer, ImageEncoder as _, Rgb};
use ugly_graphics::image::image_adapter::Adapter;
use ugly_graphics::operation::scanline::triangle::filled::overlapping::OverlappingTriangle;
use ugly_graphics::painter::Painter;
use ugly_graphics::strategy::overwrite;

fn main() -> io::Result<()> {
    let mut image = ImageBuffer::new(320, 320);
    let mut adapter = Adapter::new(&mut image);
    let mut painter = Painter::new(&mut adapter);

    let triangle = OverlappingTriangle::new(
        [(10, 210), (160, 300), (310, 20)],
        overwrite(Rgb([0xff, 0x80, 0x00])),
    );
    painter.draw(triangle); // 5

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
