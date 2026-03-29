use std::fs::File;
use std::{env, io};

use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageBuffer, ImageEncoder, Rgb};
use ugly::image::image_adapter::Adapter;
use ugly::operation::scanline::triangle::filled::tiling::TilingTriangle;
use ugly::painter::Painter;
use ugly::strategy::IntoApply;

fn main() -> io::Result<()> {
    let mut image = ImageBuffer::new(320, 320);
    let mut adapter = Adapter::new(&mut image);
    let mut painter = Painter::new(&mut adapter);

    let vertices = [(10, 10), (160, 130), (40, 310), (310, 240), (300, 40)];
    let strategy = (|mut v: Rgb<u8>| {
        v.0[1] += 0x40;
        v
    })
    .apply();

    painter.draw(TilingTriangle::new(
        [vertices[0], vertices[1], vertices[2]],
        strategy,
    ));
    painter.draw(TilingTriangle::new(
        [vertices[1], vertices[2], vertices[3]],
        strategy,
    ));
    painter.draw(TilingTriangle::new(
        [vertices[1], vertices[3], vertices[4]],
        strategy,
    ));
    painter.draw(TilingTriangle::new(
        [vertices[0], vertices[1], vertices[4]],
        strategy,
    ));

    let vertices = [(10, 130), (310, 130), (130, 10), (130, 310)];
    let strategy = (|mut v: Rgb<u8>| {
        v.0[0] += 0x40;
        v
    })
    .apply();
    painter.draw(TilingTriangle::new(
        [vertices[0], vertices[1], vertices[2]],
        strategy,
    ));
    painter.draw(TilingTriangle::new(
        [vertices[0], vertices[1], vertices[3]],
        strategy,
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
