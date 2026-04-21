use std::fs::File;
use std::{env, io};

use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageBuffer, ImageEncoder, Pixel, Rgba};
use ugly_graphics::image::ImageMut;
use ugly_graphics::image::image_adapter::Adapter;
use ugly_graphics::operation::pixel;
use ugly_graphics::operation::scanline::triangle::filled::tiling::TilingTriangle;
use ugly_graphics::painter::Painter;
use ugly_graphics::strategy::{IntoApply, IntoOverwrite};

fn strategy(addition: Rgba<u8>) -> impl Fn(Rgba<u8>) -> Rgba<u8> {
    move |mut v| {
        v.blend(&addition);
        v
    }
}

fn main() -> io::Result<()> {
    let mut image = ImageBuffer::new(320, 320);
    let mut adapter = Adapter::new(&mut image);
    adapter.set(Rgba([0x00, 0x00, 0x00, 0xff]));

    let mut painter = Painter::new(&mut adapter);

    let vertices = [(10, 10), (160, 135), (40, 310), (310, 240), (300, 40)];

    painter.draw(TilingTriangle::new(
        [vertices[0], vertices[1], vertices[2]],
        (strategy(Rgba([0x80, 0x00, 0x00, 0x80]))).apply(),
    ));
    painter.draw(TilingTriangle::new(
        [vertices[1], vertices[2], vertices[3]],
        (strategy(Rgba([0x00, 0x80, 0x00, 0x80]))).apply(),
    ));
    painter.draw(TilingTriangle::new(
        [vertices[1], vertices[3], vertices[4]],
        (strategy(Rgba([0x00, 0x00, 0x80, 0x80]))).apply(),
    ));
    painter.draw(TilingTriangle::new(
        [vertices[0], vertices[1], vertices[4]],
        (strategy(Rgba([0x40, 0x40, 0x40, 0x80]))).apply(),
    ));
    for vertex in vertices {
        painter.draw(pixel::Pixel::new(vertex, Rgba([0xff; 4]).overwrite()));
    }

    let vertices = [(10, 130), (310, 130), (130, 10), (130, 310)];
    painter.draw(TilingTriangle::new(
        [vertices[0], vertices[1], vertices[2]],
        (strategy(Rgba([0x80, 0x00, 0x00, 0x80]))).apply(),
    ));
    painter.draw(TilingTriangle::new(
        [vertices[0], vertices[1], vertices[3]],
        (strategy(Rgba([0x00, 0x80, 0x00, 0x80]))).apply(),
    ));
    for vertex in vertices {
        painter.draw(pixel::Pixel::new(vertex, Rgba([0xff; 4]).overwrite()));
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
            ExtendedColorType::Rgba8,
        )
        .map_err(io::Error::other)?;

    eprintln!("Generated image at {}", path.display());
    eprintln!("You can pass the output of this example as a path to some viewer...");
    eprintln!();
    println!("{}", path.display());

    Ok(())
}
