use ugly::image::Image;
use ugly::image::sprite::Sprite;
use ugly::operation::compute::Compute;
use ugly::operation::pixel::Pixel;
use ugly::operation::scanline::line::Line;
use ugly::operation::scanline::triangle::filled::overlapping::OverlappingTriangle;
use ugly::operation::stamp::Stamp;
use ugly::painter::Painter;
use ugly::strategy::{IntoApply, IntoOverwrite};
use ugly::view::cropped::Cropped;
use ugly::view::flipped::Flipped;
use ugly::view::rotated::Rotated;
use ugly::view::shifted::Shifted;

#[test]
fn pixels_are_manipulated() {
    let mut sprite = Sprite::<u8, 4, 4>::from_copies(0x08);
    let mut flipped = Flipped::horizontal(&mut sprite);
    let mut painter = Painter::new(&mut flipped);

    painter.draw(Pixel::new((1, 2), 0x80.overwrite()));
    painter.draw(Pixel::new((2, 3), (|v| v * 2).apply()));

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

    painter.draw(Line::new((-1, -1), (8, 8), (|v| v + 0x02).apply()));
    painter.draw(Line::new((8, -1), (-1, 8), 0xff.overwrite()));
    painter.draw(Line::new((2, 1), (5, 1), 0x80.overwrite()));
    painter.draw(Line::new((1, 2), (1, 5), 0x80.overwrite()));

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
fn triangle_is_applied() {
    const WIDTH: usize = 32;
    const HEIGHT: usize = 32;

    let (a, b, c) = ((0, 0), (31, 4), (6, 31));

    let mut filled_sprite = Sprite::<u8, WIDTH, HEIGHT>::from_copies(0x00);
    let mut painter = Painter::new(&mut filled_sprite);
    painter.draw(OverlappingTriangle::new([a, b, c], 0xff.overwrite()));

    let mut lines_sprite = Sprite::<u8, WIDTH, HEIGHT>::from_copies(0x00);
    let mut painter = Painter::new(&mut lines_sprite);
    painter.draw(Line::new(a, b, 0xff.overwrite()));
    painter.draw(Line::new(b, c, 0xff.overwrite()));
    painter.draw(Line::new(a, c, 0xff.overwrite()));

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let filled_sprite_pixel = filled_sprite.pixel((x as i32, y as i32)).unwrap();

            if filled_sprite_pixel == 0x00 {
                let lines_sprite_pixel = lines_sprite.pixel((x as i32, y as i32)).unwrap();
                if lines_sprite_pixel != 0x00 {
                    panic!();
                }
            } else {
                for x in x..=WIDTH {
                    if x == WIDTH {
                        panic!();
                    }
                    let lines_sprite_pixel = lines_sprite.pixel((x as i32, y as i32)).unwrap();
                    if lines_sprite_pixel == 0xff {
                        break;
                    }
                }
            }
        }
    }
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
