use bitvec::bitarr;
use ugly_graphics::image::bitvec_adapter::Adapter;
use ugly_graphics::operation::scanline::triangle::filled::overlapping::OverlappingTriangle;
use ugly_graphics::operation::scanline::triangle::outline::OutlineTriangle;
use ugly_graphics::painter::Painter;
use ugly_graphics::strategy::apply;

const WIDTH: usize = 128;
const HEIGHT: usize = 64;

fn main() {
    let mut data = bitarr![0; WIDTH * HEIGHT];
    let mut adapter = Adapter::new_mut(&mut data, WIDTH as _).unwrap();
    let mut painter = Painter::new(&mut adapter).with_offset((64, 32));

    painter.draw(OutlineTriangle::new(
        [(-32, -32), (32, -32), (-32, 32)],
        apply(&|v: bool| !v),
    ));

    painter.draw(OverlappingTriangle::new(
        [(-30, -30), (28, -30), (-30, 28)],
        apply(&|v: bool| !v),
    ));

    for line in data.chunks(128) {
        println!(
            "{}",
            line.iter()
                .map(|v| if *v { '#' } else { '.' })
                .collect::<String>()
        );
    }
}
