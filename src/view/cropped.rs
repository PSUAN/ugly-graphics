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

fn crop_position(dimensions: (u32, u32), (x, y): (i32, i32)) -> Option<(i32, i32)> {
    let (cropped_width, cropped_height) = dimensions;
    if x >= cropped_width as _ || y >= cropped_height as _ {
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

impl<T, P> Image<P> for Cropped<T>
where
    T: Image<P>,
{
    fn pixel(&self, position: (i32, i32)) -> Option<P> {
        let position = crop_position(self.dimensions, position)?;
        self.target.pixel(position)
    }
}

impl<T, P> ImageMut<P> for Cropped<T>
where
    T: ImageMut<P>,
    P: Clone,
{
    fn set_pixel(&mut self, position: (i32, i32), value: P) {
        if let Some(position) = crop_position(self.dimensions, position) {
            self.target.set_pixel(position, value);
        }
    }

    fn modify_pixel(&mut self, position: (i32, i32), function: Modify<P>) {
        if let Some(position) = crop_position(self.dimensions, position) {
            self.target.modify_pixel(position, function);
        }
    }

    fn set_horizontal_line(&mut self, position: (i32, i32), plus: u32, value: P) {
        let cropped_width = self.dimensions.0;

        if let Some((x, y)) = crop_position(self.dimensions, position) {
            let (x, plus) = if x < 0 {
                (0, plus - (-x) as u32)
            } else {
                (x, plus)
            };
            let (x, plus) = if x as u32 + plus >= cropped_width {
                (x, cropped_width - x as u32)
            } else {
                (x, plus)
            };
            self.target.set_horizontal_line((x, y), plus, value);
        }
    }

    fn modify_horizontal_line(&mut self, position: (i32, i32), plus: u32, function: Modify<P>) {
        let cropped_width = self.dimensions.0;

        if let Some((x, y)) = crop_position(self.dimensions, position) {
            let (x, plus) = if x < 0 {
                (0, plus - (-x) as u32)
            } else {
                (x, plus)
            };
            let (x, plus) = if x as u32 + plus >= cropped_width {
                (x, cropped_width - x as u32)
            } else {
                (x, plus)
            };
            self.target.modify_horizontal_line((x, y), plus, function);
        }
    }

    fn set(&mut self, value: P) {
        let (cropped_width, cropped_height) = self.dimensions;

        for y in 0..cropped_height as i32 {
            self.target
                .set_horizontal_line((0, y), cropped_width, value.clone());
        }
    }

    fn modify(&mut self, function: Modify<P>) {
        let (cropped_width, cropped_height) = self.dimensions;

        for y in 0..cropped_height as i32 {
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
        let mut sprite = Sprite::<u8, 5, 4>::from_copies(0x00);
        let mut cropped = Cropped::new(&mut sprite, (4, 4));
        cropped.modify(&|(x, y), _| (x + y) as _);

        assert!(cropped.pixel((3, 3)).is_some());
        assert!(cropped.pixel((4, 4)).is_none());
        assert!(cropped.pixel((-1, 0)).is_none());

        cropped.set_pixel((3, 3), 0xff);
        cropped.set_pixel((4, 4), 0xff);
        cropped.set_horizontal_line((1, 1), 8, 0x80);
        cropped.set_horizontal_line((-1, 0), 3, 0x40);

        let expected = Sprite::from_raw([
            [0x40, 0x40, 0x02, 0x03, 0x00],
            [0x01, 0x80, 0x80, 0x80, 0x00],
            [0x02, 0x03, 0x04, 0x05, 0x00],
            [0x03, 0x04, 0x05, 0xff, 0x00],
        ]);
        assert_eq!(sprite, expected);
    }

    #[test]
    fn wide_set_in_cropped_works() {
        let mut sprite = Sprite::<u8, 5, 4>::from_copies(0x00);
        let mut cropped = Cropped::new(&mut sprite, (4, 4));
        cropped.set_horizontal_line((-2, 1), 16, 0x40);

        let expected = Sprite::from_raw([
            [0x00; 5],
            [0x40, 0x40, 0x40, 0x40, 0x00],
            [0x00; 5],
            [0x00; 5],
        ]);
        assert_eq!(sprite, expected);
    }
}
