use crate::image::{Dimensions, Image};

pub enum Rotation {
    Clockwise,
    CounterClockwise,
}

pub struct Rotated<T> {
    rotation: Rotation,
    target: T,
}

impl<T> Rotated<T>
where
    T: Dimensions,
{
    fn transform(&self, (x, y): (i32, i32)) -> (i32, i32) {
        let (width, height) = self.target.dimensions();
        match self.rotation {
            Rotation::Clockwise => (y, height - 1 - x),
            Rotation::CounterClockwise => (width - 1 - y, x),
        }
    }
}

impl<T> Rotated<T> {
    pub fn new(rotation: Rotation, target: T) -> Self {
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
    fn dimensions(&self) -> (i32, i32) {
        let (width, height) = self.target.dimensions();
        (height, width)
    }
}

impl<T, C> Image<C> for Rotated<T>
where
    T: Image<C> + Dimensions,
{
    fn pixel(&self, position: (i32, i32)) -> Option<&C> {
        let position = self.transform(position);
        self.target.pixel(position)
    }
}

#[cfg(test)]
mod test {
    use crate::sprite::Sprite;

    use super::*;

    #[test]
    fn rotated_stores_sprite_ref_properly() {
        let sprite = Sprite::<u8, _, _>::from_raw([
            [0x00, 0x01, 0x02], //
            [0x10, 0x11, 0x12],
        ]);
        let rotated = Rotated::clockwise(&sprite);

        assert_eq!(rotated.pixel((0, 0)).copied(), Some(0x10));
        assert_eq!(rotated.pixel((1, 1)).copied(), Some(0x01));
        assert_eq!(rotated.pixel((2, 2)).copied(), None);
    }

    #[test]
    fn rotated_stores_sprite_mut_properly() {
        let mut sprite = Sprite::<u8, _, _>::from_raw([
            [0x00, 0x01, 0x02], //
            [0x10, 0x11, 0x12],
        ]);
        let rotated = Rotated::clockwise(&mut sprite);

        assert_eq!(rotated.pixel((0, 0)).copied(), Some(0x10));
        assert_eq!(rotated.pixel((1, 1)).copied(), Some(0x01));
        assert_eq!(rotated.pixel((2, 2)).copied(), None);
    }
}
