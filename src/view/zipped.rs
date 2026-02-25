use crate::image::{Dimensions, Image};

pub struct Zipped<A, B> {
    first: A,
    second: B,
}

impl<A, B> Zipped<A, B> {
    pub fn new(first: A, second: B) -> Self {
        Self { first, second }
    }
}

impl<A, B> Dimensions for Zipped<A, B>
where
    A: Dimensions,
    B: Dimensions,
{
    fn dimensions(&self) -> (i32, i32) {
        let (first_width, first_height) = self.first.dimensions();
        let (second_width, second_height) = self.second.dimensions();
        (
            first_width.min(second_width),
            first_height.min(second_height),
        )
    }
}

impl<A, B, PA, PB> Image<(PA, PB)> for Zipped<A, B>
where
    A: Image<PA>,
    B: Image<PB>,
{
    fn pixel(&self, position: (i32, i32)) -> Option<(PA, PB)> {
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
