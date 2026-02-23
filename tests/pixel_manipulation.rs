use ugly::operation::pixel::Pixel;
use ugly::operation::stamp::Stamp;
use ugly::painter::Painter;
use ugly::sprite::Sprite;
use ugly::strategy::{IntoApply, IntoOverwrite};
use ugly::view::flipped::Flipped;
use ugly::view::rotated::Rotated;

#[test]
fn pixels_are_manipulated() {
    let mut sprite = Sprite::<u8, 4, 4>::from_copies(0x08);
    let mut flipped = Flipped::horizontal(&mut sprite);
    let mut painter = Painter::new(&mut flipped);

    painter.draw(Pixel::new((1, 2), 0x80.overwrite()));
    painter.draw(Pixel::new((2, 3), (|_, v| v * 2).apply()));

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

    painter.draw(Stamp::new((2, 2), &rotated, &|_, pixel, _, stamp| {
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
