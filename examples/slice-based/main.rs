use ugly::image::slice_based::SliceBased;
use ugly::operation::scanline::triangle::filled::overlapping::OverlappingTriangle;
use ugly::operation::scanline::triangle::outline::OutlineTriangle;
use ugly::painter::Painter;
use ugly::strategy::{IntoApply as _, IntoOverwrite as _};

const WIDTH: usize = 32;

fn main() {
    let slice = vec![' '; WIDTH * 32].into_boxed_slice();

    let mut slice_based = SliceBased::new(slice, WIDTH as u32).unwrap();

    let mut painter = Painter::new(&mut slice_based);

    let triangle = [(1, 2), (30, 30), (16, 5)];
    painter.draw(OverlappingTriangle::new(triangle, '#'.overwrite()));
    painter.draw(OutlineTriangle::new(
        triangle,
        (|v| (v as u8 + 5) as char).apply(),
    ));
    let slice = slice_based.to_owned();

    for chunk in slice.chunks(WIDTH) {
        println!("{:?}", chunk);
    }
}
