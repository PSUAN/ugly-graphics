use bitvec::bitarr;
use ugly_graphics::image::bitvec_adapter::Adapter;
use ugly_graphics::operation::scanline::triangle::outline::OutlineTriangle;
use ugly_graphics::painter::Painter;
use ugly_graphics::strategy::overwrite;

const WIDTH: usize = 128;
const HEIGHT: usize = 64;

fn main() {
    let mut data = bitarr![0; WIDTH * HEIGHT];
    let mut adapter = Adapter::new_mut(&mut data, WIDTH as _).unwrap();
    let mut painter = Painter::new(&mut adapter);

    painter.draw(OutlineTriangle::new(
        [(1, 1), (127, 16), (63, 63)],
        overwrite(true),
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
