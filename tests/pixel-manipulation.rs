use ugly_graphics::image::sprite::Sprite;
use ugly_graphics::operation::compute::Compute;
use ugly_graphics::operation::pixel::Pixel;
use ugly_graphics::operation::scanline::line::Line;
use ugly_graphics::operation::scanline::rectangle::filled::FilledRectangle;
use ugly_graphics::operation::scanline::rectangle::outline::OutlineRectangle;
use ugly_graphics::operation::stamp::Stamp;
use ugly_graphics::painter::Painter;
use ugly_graphics::strategy::{Strategy, apply, overwrite};
use ugly_graphics::view::cropped::Cropped;
use ugly_graphics::view::flipped::Flipped;
use ugly_graphics::view::rotated::Rotated;
use ugly_graphics::view::shifted::Shifted;

#[test]
fn pixels_are_manipulated() {
    let mut sprite = Sprite::<u8, 4, 4>::from_copies(0x08);
    let mut flipped = Flipped::horizontal(&mut sprite);
    let mut painter = Painter::new(&mut flipped);

    painter.draw(Pixel::new((1, 2), overwrite(0x80)));
    painter.draw(Pixel::new((2, 3), apply(&|v| v * 2)));

    let expected = Sprite::from_raw([
        [0x08, 0x08, 0x08, 0x08],
        [0x08, 0x08, 0x08, 0x08],
        [0x08, 0x08, 0x80, 0x08],
        [0x08, 0x10, 0x08, 0x08],
    ]);

    assert_eq!(sprite, expected);
}

#[test]
fn stamp_is_applied() {
    let mut sprite = Sprite::<u8, 6, 6>::from_copies(0x10);
    let mut painter = Painter::new(&mut sprite);

    let stamp = Sprite::from_raw([
        [true, true, false, false],
        [false, false, true, false],
        [false, true, false, false],
    ]);
    let rotated = Rotated::clockwise(&stamp);

    painter.draw(Stamp::new((2, 2), &rotated, &|pixel, stamp| {
        if stamp { pixel + 0x08 } else { pixel - 0x08 }
    }));

    let expected = Sprite::from_raw([
        [0x10, 0x10, 0x10, 0x10, 0x10, 0x10],
        [0x10, 0x10, 0x10, 0x10, 0x10, 0x10],
        [0x10, 0x10, 0x08, 0x08, 0x18, 0x10],
        [0x10, 0x10, 0x18, 0x08, 0x18, 0x10],
        [0x10, 0x10, 0x08, 0x18, 0x08, 0x10],
        [0x10, 0x10, 0x08, 0x08, 0x08, 0x10],
    ]);

    assert_eq!(sprite, expected);
}

#[test]
fn line_is_applied() {
    let mut sprite = Sprite::<u8, 8, 8>::from_copies(0x00);
    let mut painter = Painter::new(&mut sprite);

    painter.draw(Line::new((-1, -1), (8, 8), apply(&|v| v + 0x02)));
    painter.draw(Line::new((8, -1), (-1, 8), overwrite(0xff)));
    painter.draw(Line::new((2, 1), (5, 1), overwrite(0x80)));
    painter.draw(Line::new((1, 2), (1, 5), overwrite(0x80)));

    let expected = Sprite::from_raw([
        [0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff],
        [0x00, 0x02, 0x80, 0x80, 0x80, 0x80, 0xff, 0x00],
        [0x00, 0x80, 0x02, 0x00, 0x00, 0xff, 0x00, 0x00],
        [0x00, 0x80, 0x00, 0x02, 0xff, 0x00, 0x00, 0x00],
        [0x00, 0x80, 0x00, 0xff, 0x02, 0x00, 0x00, 0x00],
        [0x00, 0x80, 0xff, 0x00, 0x00, 0x02, 0x00, 0x00],
        [0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00],
        [0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02],
    ]);
    assert_eq!(sprite, expected);
}

#[test]
fn gradient() {
    let mut sprite = Sprite::<u8, 8, 8>::from_copies(0x00);
    let mut shifted = Shifted::new(&mut sprite, (1, 1));
    let mut cropped = Cropped::new(&mut shifted, (6, 6));
    let mut painter = Painter::new(&mut cropped);

    let gradient = Compute::new(&|(x, y), _| x as u8 + y as u8);
    painter.draw(gradient);

    let expected = Sprite::from_raw([
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        [0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x00],
        [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x00],
        [0x00, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00],
        [0x00, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x00],
        [0x00, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x00],
        [0x00, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x00],
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    ]);
    assert_eq!(sprite, expected);
}

#[test]
fn rectangles() {
    let mut sprite = Sprite::<u8, 16, 8>::from_copies(b' ');
    let mut painter = Painter::new(&mut sprite);

    let delta = b'#' - b' ';
    let apply = |v| v + delta;
    let rectangle = OutlineRectangle::new((14, 1), (1, 6), Strategy::Apply(&apply));
    painter.draw(rectangle);

    let rectangle = FilledRectangle::new((3, 3), (12, 4), Strategy::Overwrite(b'+'));
    painter.draw(rectangle);

    let raw = [
        b"                ",
        b" ############## ",
        b" #            # ",
        b" # ++++++++++ # ",
        b" # ++++++++++ # ",
        b" #            # ",
        b" ############## ",
        b"                ",
    ];
    let expected = Sprite::from_raw(raw.map(Clone::clone));
    assert_eq!(sprite, expected);
}
