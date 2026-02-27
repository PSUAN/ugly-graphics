use ugly::image::sprite::Sprite;
use ugly::operation::pixel::Pixel;
use ugly::operation::scanline::line::Line;
use ugly::operation::stamp::Stamp;
use ugly::painter::Painter;
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

#[test]
fn line_is_applied() {
    let mut sprite = Sprite::<u8, 8, 8>::from_copies(0x00);
    let mut painter = Painter::new(&mut sprite);

    painter.draw(Line::new(
        (-1, -1),
        (8, 8),
        (&|(x, y), _| x as u8 + y as u8).apply(),
    ));
    painter.draw(Line::new((8, -1), (-1, 8), 0xff.overwrite()));
    painter.draw(Line::new((2, 1), (5, 1), 0x80.overwrite()));
    painter.draw(Line::new((1, 2), (1, 5), 0x80.overwrite()));

    let expected = Sprite::from_raw([
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff],
        [0x00, 0x02, 0x80, 0x80, 0x80, 0x80, 0xff, 0x00],
        [0x00, 0x80, 0x04, 0x00, 0x00, 0xff, 0x00, 0x00],
        [0x00, 0x80, 0x00, 0x06, 0xff, 0x00, 0x00, 0x00],
        [0x00, 0x80, 0x00, 0xff, 0x08, 0x00, 0x00, 0x00],
        [0x00, 0x80, 0xff, 0x00, 0x00, 0x0a, 0x00, 0x00],
        [0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00],
        [0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0e],
    ]);
    assert_eq!(sprite, expected);
}
