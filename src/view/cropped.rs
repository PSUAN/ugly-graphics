//! The [`Cropped`] view reduces the available dimensions of the [`Dimensions`]
//! target stored in it.

use crate::image::{Dimensions, Image, ImageMut};

/// The view to reduce dimensions.
///
/// Provides both mutable and immutable operations.
pub struct Cropped<T> {
    dimensions: (u32, u32),
    target: T,
}

impl<T> Cropped<T> {
    /// Create a new instance with dimensions to be less or equal to provided.
    pub fn new(target: T, dimensions: (u32, u32)) -> Self {
        Self { dimensions, target }
    }

    /// Extract stored `target` value.
    pub fn into_owned(self) -> T {
        self.target
    }
}

fn crop_position(dimensions: (u32, u32), (x, y): (u32, u32)) -> Option<(u32, u32)> {
    let (cropped_width, cropped_height) = dimensions;
    if x >= cropped_width || y >= cropped_height {
        return None;
    }
    Some((x, y))
}

impl<T> Dimensions for Cropped<T>
where
    T: Dimensions,
{
    fn dimensions(&self) -> (u32, u32) {
        let (target_width, target_height) = self.target.dimensions();
        let (cropped_width, cropped_height) = self.dimensions;
        (
            target_width.min(cropped_width),
            target_height.min(cropped_height),
        )
    }
}

impl<T> Image for Cropped<T>
where
    T: Image,
{
    type Pixel = T::Pixel;

    fn pixel(&self, position: (u32, u32)) -> Option<Self::Pixel> {
        let position = crop_position(self.dimensions, position)?;
        self.target.pixel(position)
    }
}

impl<T, W> ImageMut<W> for Cropped<T>
where
    T: ImageMut<W>,
{
    fn write_pixel(&mut self, position: (u32, u32), writer: &W) {
        if let Some(position) = crop_position(self.dimensions, position) {
            self.target.write_pixel(position, writer);
        }
    }

    fn write_horizontal_line(&mut self, position: (u32, u32), plus: u32, writer: &W) {
        let cropped_width = self.dimensions.0;

        if let Some((x, y)) = crop_position(self.dimensions, position) {
            let (x, plus) = if x + plus >= cropped_width {
                (x, cropped_width - x)
            } else {
                (x, plus)
            };
            self.target.write_horizontal_line((x, y), plus, writer);
        }
    }

    fn write(&mut self, writer: &W) {
        let (cropped_width, cropped_height) = self.dimensions;

        for y in 0..cropped_height {
            self.target
                .write_horizontal_line((0, y), cropped_width, writer);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::image::sprite::Sprite;
    use crate::strategy;

    use super::*;

    #[test]
    fn cropped_works() {
        let mut sprite = Sprite::<u8, 5, 4>::from_copies(0x01);
        let mut cropped = Cropped::new(&mut sprite, (4, 4));
        cropped.write(&strategy::apply(&|v| v + 0x01));

        assert!(cropped.pixel((3, 3)).is_some());
        assert!(cropped.pixel((4, 4)).is_none());

        cropped.write_pixel((3, 3), &strategy::overwrite(0xff));
        cropped.write_pixel((4, 4), &strategy::overwrite(0xff));
        cropped.write_horizontal_line((1, 1), 8, &strategy::overwrite(0x80));
        cropped.write_horizontal_line((0, 0), 2, &strategy::overwrite(0x40));

        let expected = Sprite::from_raw([
            [0x40, 0x40, 0x02, 0x02, 0x01],
            [0x02, 0x80, 0x80, 0x80, 0x01],
            [0x02, 0x02, 0x02, 0x02, 0x01],
            [0x02, 0x02, 0x02, 0xff, 0x01],
        ]);
        assert_eq!(sprite, expected);
    }

    #[test]
    fn wide_set_in_cropped_works() {
        let mut sprite = Sprite::<u8, 5, 4>::from_copies(0x00);
        let mut cropped = Cropped::new(&mut sprite, (4, 4));
        cropped.write_horizontal_line((0, 1), 16, &strategy::overwrite(0x40));

        let expected = Sprite::from_raw([
            [0x00; 5],
            [0x40, 0x40, 0x40, 0x40, 0x00],
            [0x00; 5],
            [0x00; 5],
        ]);
        assert_eq!(sprite, expected);
    }
}
