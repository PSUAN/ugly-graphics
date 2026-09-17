use ugly_graphics::image::ImageMutFull;
use ugly_graphics::image::slice_based::SliceBased;
use ugly_graphics::operation::scanline::triangle::filled::overlapping::OverlappingTriangle;
use ugly_graphics::painter::Painter;
use ugly_graphics::strategy;

fn main() {
    let slice = vec!['.'; 32 * 16].into_boxed_slice();
    let mut slice_based = SliceBased::new(slice, 32).unwrap();
    let sprite_reference = &mut slice_based as &mut dyn ImageMutFull<char>;
    let mut painter = Painter::new(sprite_reference);

    painter_user(&mut painter);

    let slice = slice_based.to_owned();
    for chunk in slice.chunks(32) {
        println!("{}", chunk.iter().collect::<String>());
    }
}

fn painter_user(painter: &mut Painter<&mut dyn ImageMutFull<char>>) {
    let overwrite_triangle =
        OverlappingTriangle::new([(0, 0), (31, 0), (31, 15)], strategy::overwrite('+'));
    painter.draw(overwrite_triangle);

    let apply_triangle =
        OverlappingTriangle::new([(0, 0), (0, 15), (31, 15)], strategy::apply(&|_| '='));
    painter.draw(apply_triangle);
}
