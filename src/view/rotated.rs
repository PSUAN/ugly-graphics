use crate::image::{Dimensions, Image};

#[derive(Clone, Copy, Debug)]
pub enum Rotation {
    Clockwise,
    CounterClockwise,
}

pub struct Rotated<T> {
    rotation: Rotation,
    target: T,
}

fn transform(rotation: Rotation, (width, height): (u32, u32), (x, y): (i32, i32)) -> (i32, i32) {
    match rotation {
        Rotation::Clockwise => (y, height as i32 - 1 - x),
        Rotation::CounterClockwise => (width as i32 - 1 - y, x),
    }
}

impl<T> Rotated<T> {
    pub fn new(target: T, rotation: Rotation) -> Self {
        Self { rotation, target }
    }

    pub fn clockwise(target: T) -> Self {
        let rotation = Rotation::Clockwise;
        Self { rotation, target }
    }

    pub fn counter_clockwise(target: T) -> Self {
        let rotation = Rotation::CounterClockwise;
        Self { rotation, target }
    }

    pub fn extract(self) -> T {
        self.target
    }
}

impl<T> Dimensions for Rotated<T>
where
    T: Dimensions,
{
    fn dimensions(&self) -> (u32, u32) {
        let (width, height) = self.target.dimensions();
        (height, width)
    }
}

impl<T, C> Image<C> for Rotated<T>
where
    T: Image<C> + Dimensions,
{
    fn pixel(&self, position: (i32, i32)) -> Option<C> {
        let position = transform(self.rotation, self.target.dimensions(), position);
        self.target.pixel(position)
    }
}

#[cfg(test)]
mod test {
    use crate::image::sprite::Sprite;

    use super::*;

    #[test]
    fn rotated_stores_sprite_ref_properly() {
        let sprite = Sprite::<u8, _, _>::from_raw([
            [0x00, 0x01, 0x02], //
            [0x10, 0x11, 0x12], //
        ]);
        let rotated = Rotated::clockwise(&sprite);

        assert_eq!(rotated.pixel((-1, 2)), None);
        assert_eq!(rotated.pixel((0, 0)), Some(0x10));
        assert_eq!(rotated.pixel((1, 1)), Some(0x01));
        assert_eq!(rotated.pixel((2, 2)), None);
    }

    #[test]
    fn rotated_stores_sprite_mut_properly() {
        let mut sprite = Sprite::<u8, _, _>::from_raw([
            [0x00, 0x01, 0x02], //
            [0x10, 0x11, 0x12], //
        ]);
        let rotated = Rotated::clockwise(&mut sprite);

        assert_eq!(rotated.pixel((-1, 2)), None);
        assert_eq!(rotated.pixel((0, 0)), Some(0x10));
        assert_eq!(rotated.pixel((1, 1)), Some(0x01));
        assert_eq!(rotated.pixel((2, 2)), None);
    }
}
