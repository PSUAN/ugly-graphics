//! The [`Zipped`] view allows to zip two [`Image`s](`Image`) together for
//! read operations.

use crate::image::{Dimensions, Image};

/// The view that zips two [`Image`s](`Image`).
pub struct Zipped<A, B> {
    first: A,
    second: B,
}

impl<A, B> Zipped<A, B> {
    /// Create new instance provided two [`Image`s](`Image`).
    pub fn new(first: A, second: B) -> Self {
        Self { first, second }
    }

    /// Extract stored values.
    pub fn into_owned(self) -> (A, B) {
        (self.first, self.second)
    }
}

impl<A, B> Dimensions for Zipped<A, B>
where
    A: Dimensions,
    B: Dimensions,
{
    fn dimensions(&self) -> (u32, u32) {
        let (first_width, first_height) = self.first.dimensions();
        let (second_width, second_height) = self.second.dimensions();
        (
            first_width.min(second_width),
            first_height.min(second_height),
        )
    }
}

impl<A, B> Image for Zipped<A, B>
where
    A: Image,
    B: Image,
{
    type Pixel = (A::Pixel, B::Pixel);

    fn pixel(&self, position: (u32, u32)) -> Option<Self::Pixel> {
        self.first.pixel(position).zip(self.second.pixel(position))
    }
}

#[cfg(test)]
mod test {
    use crate::image::sprite::Sprite;

    use super::*;

    #[test]
    fn zipped_zips_images() {
        let sprite = Sprite::from_raw([
            [0x00, 0x01, 0x02, 0x03],
            [0x10, 0x11, 0x12, 0x13],
            [0x20, 0x21, 0x22, 0x23],
            [0x30, 0x31, 0x32, 0x33],
        ]);
        let mask = Sprite::from_raw([
            [true, false, true],
            [false, true, false],
            [true, false, true],
        ]);
        let zipped = Zipped::new(&sprite, &mask);

        assert_eq!(zipped.pixel((0, 0)), Some((0x00, true)));
        assert_eq!(zipped.pixel((1, 2)), Some((0x21, false)));
        assert_eq!(zipped.pixel((3, 1)), None);
    }
}
