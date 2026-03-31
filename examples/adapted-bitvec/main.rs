use bitvec::bitarr;
use ugly::image::bitvec_adapter::Adapter;
use ugly::operation::scanline::triangle::outline::OutlineTriangle;
use ugly::painter::Painter;
use ugly::strategy::IntoOverwrite;

fn main() {
    let mut data = bitarr![0; 128 * 64];
    let mut adapter = Adapter::new_mut(&mut data, 128).unwrap();
    let mut painter = Painter::new(&mut adapter);

    painter.draw(OutlineTriangle::new(
        [(1, 1), (127, 16), (63, 63)],
        true.overwrite(),
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
