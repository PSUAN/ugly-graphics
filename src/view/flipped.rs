use crate::image::{Dimensions, Image, ImageMut};
use crate::strategy::Modify;

#[derive(Clone, Copy, Debug)]
pub enum Flip {
    Horizontal,
    Vertical,
}

pub struct Flipped<T> {
    direction: Flip,
    target: T,
}

fn transform(
    direction: Flip,
    (width, height): (u32, u32),
    (x, y): (u32, u32),
) -> Option<(u32, u32)> {
    match direction {
        Flip::Horizontal => Some((width.checked_sub(1 + x)?, y)),
        Flip::Vertical => Some((x, height.checked_sub(1 + y)?)),
    }
}

fn transform_scan(
    direction: Flip,
    (width, height): (u32, u32),
    (x, y): (u32, u32),
    total: u32,
) -> Option<((u32, u32), u32)> {
    match direction {
        Flip::Horizontal => {
            let (from, to) = (x, (x + total).checked_sub(1)?);
            let (from, to) = (width.checked_sub(1 + from)?, width.saturating_sub(1 + to));

            Some(((to, y), from - to + 1))
        }
        Flip::Vertical => Some(((x, height.checked_sub(1 + y)?), total)),
    }
}

impl<T> Flipped<T> {
    pub fn new(target: T, direction: Flip) -> Self {
        Self { direction, target }
    }

    pub fn horizontal(target: T) -> Self {
        let direction = Flip::Horizontal;
        Self { direction, target }
    }

    pub fn vertical(target: T) -> Self {
        let direction = Flip::Vertical;
        Self { direction, target }
    }

    pub fn extract(self) -> T {
        self.target
    }
}

impl<T> Dimensions for Flipped<T>
where
    T: Dimensions,
{
    fn dimensions(&self) -> (u32, u32) {
        self.target.dimensions()
    }
}

impl<T, P> Image<P> for Flipped<T>
where
    T: Image<P>,
{
    fn pixel(&self, position: (u32, u32)) -> Option<P> {
        let position = transform(self.direction, self.target.dimensions(), position)?;
        self.target.pixel(position)
    }
}

impl<T, P> ImageMut<P> for Flipped<T>
where
    T: ImageMut<P>,
{
    fn set_pixel(&mut self, position: (u32, u32), value: P) {
        if let Some(position) = transform(self.direction, self.target.dimensions(), position) {
            self.target.set_pixel(position, value);
        }
    }

    fn modify_pixel(&mut self, position: (u32, u32), function: Modify<P>) {
        if let Some(position) = transform(self.direction, self.target.dimensions(), position) {
            self.target.modify_pixel(position, function);
        }
    }

    fn set_horizontal_line(&mut self, position: (u32, u32), total: u32, value: P) {
        if let Some(((x, y), total)) =
            transform_scan(self.direction, self.target.dimensions(), position, total)
        {
            self.target.set_horizontal_line((x, y), total, value);
        }
    }

    fn modify_horizontal_line(&mut self, position: (u32, u32), total: u32, function: Modify<P>) {
        if let Some(((x, y), total)) =
            transform_scan(self.direction, self.target.dimensions(), position, total)
        {
            self.target.modify_horizontal_line((x, y), total, function);
        }
    }

    fn set(&mut self, value: P) {
        self.target.set(value);
    }

    fn modify(&mut self, function: Modify<P>) {
        self.target.modify(function);
    }
}

#[cfg(test)]
mod test {
    use crate::image::sprite::Sprite;

    use super::*;

    #[test]
    fn flipped_stores_sprite_ref_properly() {
        let sprite = Sprite::<u8, _, _>::from_raw([
            [0x00, 0x01, 0x02], //
            [0x10, 0x11, 0x12], //
        ]);
        let flipped = Flipped::horizontal(&sprite);

        assert_eq!(flipped.pixel((0, 0)), Some(0x02));
        assert_eq!(flipped.pixel((1, 1)), Some(0x11));
        assert_eq!(flipped.pixel((2, 2)), None);
    }

    #[test]
    fn flipped_stores_sprite_mut_properly() {
        let mut sprite = Sprite::<u8, _, _>::from_raw([
            [0x00, 0x01, 0x02], //
            [0x10, 0x11, 0x12], //
        ]);
        let flipped = Flipped::horizontal(&mut sprite);

        assert_eq!(flipped.pixel((0, 0)), Some(0x02));
        assert_eq!(flipped.pixel((1, 1)), Some(0x11));
        assert_eq!(flipped.pixel((2, 2)), None);
    }

    #[test]
    fn flipped_set_line_works_properly() {
        let mut sprite = Sprite::<u8, 4, 4>::from_copies(0x00);
        let mut flipped = Flipped::horizontal(&mut sprite);

        flipped.set_horizontal_line((0, 1), 2, 0x20);
        flipped.set_horizontal_line((2, 2), 4, 0x30);
        flipped.set_horizontal_line((1, 3), 2, 0x40);

        let expected = Sprite::from_raw([
            [0x00; 4],
            [0x00, 0x00, 0x20, 0x20],
            [0x30, 0x30, 0x00, 0x00],
            [0x00, 0x40, 0x40, 0x00],
        ]);

        assert_eq!(sprite, expected);
    }
}
