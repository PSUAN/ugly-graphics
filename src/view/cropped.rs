use crate::image::{Dimensions, Image, ImageMut};
use crate::strategy::Modify;

pub struct Cropped<T> {
    dimensions: (u32, u32),
    target: T,
}

impl<T> Cropped<T> {
    pub fn new(target: T, dimensions: (u32, u32)) -> Self {
        Self { dimensions, target }
    }

    pub fn extract(self) -> T {
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

impl<T> ImageMut for Cropped<T>
where
    T: ImageMut,
    T::Pixel: Clone,
{
    type Pixel = T::Pixel;

    fn set_pixel(&mut self, position: (u32, u32), value: Self::Pixel) {
        if let Some(position) = crop_position(self.dimensions, position) {
            self.target.set_pixel(position, value);
        }
    }

    fn modify_pixel(&mut self, position: (u32, u32), function: Modify<Self::Pixel>) {
        if let Some(position) = crop_position(self.dimensions, position) {
            self.target.modify_pixel(position, function);
        }
    }

    fn set_horizontal_line(&mut self, position: (u32, u32), plus: u32, value: Self::Pixel) {
        let cropped_width = self.dimensions.0;

        if let Some((x, y)) = crop_position(self.dimensions, position) {
            let (x, plus) = if x + plus >= cropped_width {
                (x, cropped_width - x)
            } else {
                (x, plus)
            };
            self.target.set_horizontal_line((x, y), plus, value);
        }
    }

    fn modify_horizontal_line(
        &mut self,
        position: (u32, u32),
        plus: u32,
        function: Modify<Self::Pixel>,
    ) {
        let cropped_width = self.dimensions.0;

        if let Some((x, y)) = crop_position(self.dimensions, position) {
            let (x, plus) = if x + plus >= cropped_width {
                (x, cropped_width - x)
            } else {
                (x, plus)
            };
            self.target.modify_horizontal_line((x, y), plus, function);
        }
    }

    fn set(&mut self, value: Self::Pixel) {
        let (cropped_width, cropped_height) = self.dimensions;

        for y in 0..cropped_height {
            self.target
                .set_horizontal_line((0, y), cropped_width, value.clone());
        }
    }

    fn modify(&mut self, function: Modify<Self::Pixel>) {
        let (cropped_width, cropped_height) = self.dimensions;

        for y in 0..cropped_height {
            self.target
                .modify_horizontal_line((0, y), cropped_width, function);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::image::sprite::Sprite;

    use super::*;

    #[test]
    fn cropped_works() {
        let mut sprite = Sprite::<u8, 5, 4>::from_copies(0x01);
        let mut cropped = Cropped::new(&mut sprite, (4, 4));
        cropped.modify(&|v| v + 0x01);

        assert!(cropped.pixel((3, 3)).is_some());
        assert!(cropped.pixel((4, 4)).is_none());

        cropped.set_pixel((3, 3), 0xff);
        cropped.set_pixel((4, 4), 0xff);
        cropped.set_horizontal_line((1, 1), 8, 0x80);
        cropped.set_horizontal_line((0, 0), 2, 0x40);

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
        cropped.set_horizontal_line((0, 1), 16, 0x40);

        let expected = Sprite::from_raw([
            [0x00; 5],
            [0x40, 0x40, 0x40, 0x40, 0x00],
            [0x00; 5],
            [0x00; 5],
        ]);
        assert_eq!(sprite, expected);
    }
}
