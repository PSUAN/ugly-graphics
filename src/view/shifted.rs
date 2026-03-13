use crate::image::{Dimensions, Image, ImageMut};
use crate::strategy::Modify;

pub struct Shifted<T> {
    shift: (u32, u32),
    target: T,
}

impl<T> Shifted<T> {
    pub fn new(target: T, shift: (u32, u32)) -> Self {
        Shifted { shift, target }
    }

    pub fn extract(self) -> T {
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

impl<T, P> Image<P> for Shifted<T>
where
    T: Image<P>,
{
    fn pixel(&self, position: (u32, u32)) -> Option<P> {
        let position = shift(self.shift, position);
        self.target.pixel(position)
    }
}

impl<T, P> ImageMut<P> for Shifted<T>
where
    T: ImageMut<P>,
    P: Clone,
{
    fn set_pixel(&mut self, position: (u32, u32), value: P) {
        let position = shift(self.shift, position);
        self.target.set_pixel(position, value);
    }

    fn modify_pixel(&mut self, position: (u32, u32), function: Modify<P>) {
        let position = shift(self.shift, position);
        self.target.modify_pixel(position, function);
    }

    fn set_horizontal_line(&mut self, position: (u32, u32), total: u32, value: P) {
        let position = shift(self.shift, position);
        self.target.set_horizontal_line(position, total, value);
    }

    fn modify_horizontal_line(&mut self, position: (u32, u32), total: u32, function: Modify<P>) {
        let position = shift(self.shift, position);
        self.target
            .modify_horizontal_line(position, total, function);
    }

    fn set(&mut self, value: P) {
        let (width, height) = self.target.dimensions();
        let (shift_x, shift_y) = self.shift;
        let total = width - shift_x;

        for y in shift_y..height {
            self.target
                .set_horizontal_line((shift_x, y), total, value.clone());
        }
    }

    fn modify(&mut self, function: Modify<P>) {
        let (width, height) = self.target.dimensions();
        let (shift_x, shift_y) = self.shift;
        let total = width - shift_x;

        for y in shift_y..height {
            self.target
                .modify_horizontal_line((shift_x, y), total, function);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::image::sprite::Sprite;

    use super::*;

    #[test]
    fn modify_pixel_works_properly() {
        let mut sprite = Sprite::<u8, 4, 4>::from_copies(0x01);
        let mut shifted = Shifted::new(&mut sprite, (1, 2));
        let function = &|v| v + 1;
        shifted.modify_pixel((2, 0), function);
        shifted.modify_pixel((0, 1), function);

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

        shifted.modify(&|v| v + 1);

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
