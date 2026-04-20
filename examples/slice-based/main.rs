use ugly::image::slice_based::SliceBased;
use ugly::operation::scanline::rectangle::filled::FilledRectangle;
use ugly::operation::scanline::rectangle::outline::OutlineRectangle;
use ugly::operation::scanline::triangle::filled::overlapping::OverlappingTriangle;
use ugly::operation::scanline::triangle::outline::OutlineTriangle;
use ugly::painter::Painter;
use ugly::strategy::{IntoApply as _, IntoOverwrite as _};

const WIDTH: usize = 32;

fn main() {
    let slice = vec![' '; WIDTH * 32].into_boxed_slice();

    let mut slice_based = SliceBased::new(slice, WIDTH as u32).unwrap();

    let mut painter = Painter::new(&mut slice_based);

    painter.draw(OutlineRectangle::new(
        (1, 30),
        (30, 1),
        (|v| (v as u8 + 11) as char).apply(),
    ));

    painter.draw(FilledRectangle::new((3, 3), (28, 28), '-'.overwrite()));

    let triangle = [(2, 2), (30, 30), (16, 5)];
    painter.draw(OverlappingTriangle::new(triangle, '#'.overwrite()));
    painter.draw(OutlineTriangle::new(
        triangle,
        (|v| (v as u8 + 7) as char).apply(),
    ));
    let slice = slice_based.to_owned();

    for chunk in slice.chunks(WIDTH) {
        println!("{}", chunk.iter().collect::<String>());
    }
}
