use std::fs::File;
use std::{env, io};

use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageBuffer, ImageEncoder, Rgb};
use ugly_graphics::image::image_adapter::Adapter;
use ugly_graphics::operation::scanline::line::Line;
use ugly_graphics::painter::Painter;
use ugly_graphics::strategy::overwrite;

fn main() -> io::Result<()> {
    let mut image = ImageBuffer::new(320, 320);
    let mut adapter = Adapter::new(&mut image);
    let mut painter = Painter::new(&mut adapter);

    for y in (0..320).step_by(16) {
        painter.draw(Line::new(
            (0, 0),
            (320, y),
            overwrite(Rgb([0xff, 0xff, 0xff])),
        ));
    }
    for x in (0..=320).step_by(16) {
        painter.draw(Line::new(
            (0, 0),
            (x, 320),
            overwrite(Rgb([0xff, 0xff, 0xff])),
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
