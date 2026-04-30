use ugly_graphics::image::slice_based::SliceBased;
use ugly_graphics::operation::scanline::rectangle::filled::FilledRectangle;
use ugly_graphics::operation::scanline::rectangle::outline::OutlineRectangle;
use ugly_graphics::operation::scanline::triangle::filled::overlapping::OverlappingTriangle;
use ugly_graphics::operation::scanline::triangle::outline::OutlineTriangle;
use ugly_graphics::painter::Painter;
use ugly_graphics::strategy::{apply, overwrite};

const WIDTH: usize = 32;
const HEIGHT: usize = 32;

fn main() {
    let slice = vec![' '; WIDTH * HEIGHT].into_boxed_slice();

    let mut slice_based = SliceBased::new(slice, WIDTH as u32).unwrap();

    let mut painter = Painter::new(&mut slice_based);

    painter.draw(OutlineRectangle::new(
        (1, 30),
        (30, 1),
        apply(&|v| (v as u8 + 11) as char),
    ));

    painter.draw(FilledRectangle::new((3, 3), (28, 28), overwrite('-')));

    let triangle = [(2, 2), (30, 30), (16, 5)];
    painter.draw(OverlappingTriangle::new(triangle, overwrite('#')));
    painter.draw(OutlineTriangle::new(
        triangle,
        apply(&|v| (v as u8 + 7) as char),
    ));
    let slice = slice_based.to_owned();

    for chunk in slice.chunks(WIDTH) {
        println!("{}", chunk.iter().collect::<String>());
    }
}
