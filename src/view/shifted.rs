//! The [`Shifted`] view shifts (translates) the stored [`Image`] or
//! [`ImageMut`] by a positive amount, effectively reducing its size.

use crate::image::{Dimensions, Image, ImageMut};

/// The view to shift the coordinates.
pub struct Shifted<T> {
    shift: (u32, u32),
    target: T,
}

impl<T> Shifted<T> {
    /// Create new instance given the `shift` value.
    pub fn new(target: T, shift: (u32, u32)) -> Self {
        Self { shift, target }
    }

    /// Extract stored `target` value.
    pub fn into_owned(self) -> T {
        self.target
    }
}

impl<T> Dimensions for Shifted<T>
where
    T: Dimensions,
{
    fn dimensions(&self) -> (u32, u32) {
        self.target.dimensions()
    }
}

fn shift((shift_x, shift_y): (u32, u32), (x, y): (u32, u32)) -> (u32, u32) {
    (shift_x + x, shift_y + y)
}

impl<T> Image for Shifted<T>
where
    T: Image,
{
    type Pixel = T::Pixel;

    fn pixel(&self, position: (u32, u32)) -> Option<Self::Pixel> {
        let position = shift(self.shift, position);
        self.target.pixel(position)
    }
}

impl<T, W> ImageMut<W> for Shifted<T>
where
    T: ImageMut<W> + Dimensions,
{
    fn write_pixel(&mut self, position: (u32, u32), writer: &W) {
        let position = shift(self.shift, position);
        self.target.write_pixel(position, writer);
    }

    fn write_horizontal_line(&mut self, position: (u32, u32), total: u32, writer: &W) {
        let position = shift(self.shift, position);
        self.target.write_horizontal_line(position, total, writer);
    }

    fn write(&mut self, writer: &W) {
        let (width, height) = self.target.dimensions();
        let (shift_x, shift_y) = self.shift;
        let total = width - shift_x;

        for y in shift_y..height {
            self.target
                .write_horizontal_line((shift_x, y), total, writer);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::image::sprite::Sprite;
    use crate::strategy;

    use super::*;

    #[test]
    fn modify_pixel_works_properly() {
        let mut sprite = Sprite::<u8, 4, 4>::from_copies(0x01);
        let mut shifted = Shifted::new(&mut sprite, (1, 2));
        let function = &|v| v + 1;
        shifted.write_pixel((2, 0), &strategy::apply(function));
        shifted.write_pixel((0, 1), &strategy::apply(function));

        let expected = Sprite::from_raw([
            [0x01; 4],
            [0x01; 4],
            [0x01, 0x01, 0x01, 0x02],
            [0x01, 0x02, 0x01, 0x01],
        ]);
        assert_eq!(sprite, expected);
    }

    #[test]
    fn shifted_modify_works_properly() {
        let mut sprite = Sprite::<u8, 5, 6>::from_copies(0x01);
        let mut shifted = Shifted::new(&mut sprite, (2, 3));

        shifted.write(&strategy::apply(&|v| v + 1));

        let expected = Sprite::from_raw([
            [0x01; 5],
            [0x01; 5],
            [0x01; 5],
            [0x01, 0x01, 0x02, 0x02, 0x02],
            [0x01, 0x01, 0x02, 0x02, 0x02],
            [0x01, 0x01, 0x02, 0x02, 0x02],
        ]);

        assert_eq!(sprite, expected);
    }
}
