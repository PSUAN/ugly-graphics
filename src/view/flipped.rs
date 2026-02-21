use crate::image::{Dimensions, Image, ImageMut};

#[derive(Clone, Copy, Debug)]
pub enum Flip {
    Horizontal,
    Vertical,
}

pub struct Flipped<T> {
    direction: Flip,
    target: T,
}

impl<T> Flipped<T>
where
    T: Dimensions,
{
    fn transformer(&self) -> impl Fn((i32, i32)) -> (i32, i32) + 'static {
        let (width, height) = self.target.dimensions();
        let direction = self.direction;
        move |(x, y)| match direction {
            Flip::Horizontal => (width - 1 - x, y),
            Flip::Vertical => (x, height - 1 - y),
        }
    }
}

impl<T> Flipped<T> {
    pub fn new(direction: Flip, target: T) -> Self {
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
    fn dimensions(&self) -> (i32, i32) {
        self.target.dimensions()
    }
}

impl<T, C> Image<C> for Flipped<T>
where
    T: Image<C> + Dimensions,
{
    fn pixel(&self, position: (i32, i32)) -> Option<&C> {
        let position = self.transformer()(position);
        self.target.pixel(position)
    }
}

impl<T, C> ImageMut<C> for Flipped<T>
where
    T: ImageMut<C> + Dimensions,
{
    fn set_pixel(&mut self, position: (i32, i32), value: C) {
        let position = self.transformer()(position);
        self.target.set_pixel(position, value);
    }

    fn modify_pixel(&mut self, position: (i32, i32), function: &dyn Fn((i32, i32), C) -> C) {
        let transformer = self.transformer();
        let position = transformer(position);
        self.target.modify_pixel(position, &move |position, pixel| {
            let position = transformer(position);
            function(position, pixel)
        });
    }

    fn set_horizontal_line(&mut self, position: (i32, i32), plus: u32, value: C) {
        let (x, y) = self.transformer()(position);
        let x = match self.direction {
            Flip::Horizontal => x - plus as i32,
            Flip::Vertical => x,
        };
        self.target.set_horizontal_line((x, y), plus, value);
    }

    fn modify_horizontal_line(
        &mut self,
        position: (i32, i32),
        plus: u32,
        function: &dyn Fn((i32, i32), C) -> C,
    ) {
        let transformer = self.transformer();
        let (x, y) = transformer(position);
        let x = match self.direction {
            Flip::Horizontal => x - plus as i32,
            Flip::Vertical => x,
        };
        self.target
            .modify_horizontal_line((x, y), plus, &move |position, pixel| {
                let position = transformer(position);
                function(position, pixel)
            });
    }

    fn set(&mut self, value: C) {
        self.target.set(value);
    }

    fn modify(&mut self, function: &dyn Fn((i32, i32), C) -> C) {
        let transformer = self.transformer();
        self.target.modify(&move |position, pixel| {
            let position = transformer(position);
            function(position, pixel)
        });
    }
}

#[cfg(test)]
mod test {
    use crate::sprite::Sprite;

    use super::*;

    #[test]
    fn flip_stores_sprite_ref_properly() {
        let sprite = Sprite::<u8, _, _>::from_raw([
            [0x00, 0x01, 0x02], //
            [0x10, 0x11, 0x12], //
        ]);
        let flipped = Flipped::horizontal(&sprite);

        assert_eq!(flipped.pixel((0, 0)).copied(), Some(0x02));
        assert_eq!(flipped.pixel((1, 1)).copied(), Some(0x11));
        assert_eq!(flipped.pixel((2, 2)).copied(), None);
    }

    #[test]
    fn flip_stores_sprite_mut_properly() {
        let mut sprite = Sprite::<u8, _, _>::from_raw([
            [0x00, 0x01, 0x02], //
            [0x10, 0x11, 0x12], //
        ]);
        let flipped = Flipped::horizontal(&mut sprite);

        assert_eq!(flipped.pixel((0, 0)).copied(), Some(0x02));
        assert_eq!(flipped.pixel((1, 1)).copied(), Some(0x11));
        assert_eq!(flipped.pixel((2, 2)).copied(), None);
    }

    #[test]
    fn modify_works() {
        const WIDTH: usize = 32;

        let mut sprite = Sprite::<u8, WIDTH, 32>::from_copies(0x00);
        let mut flipped = Flipped::horizontal(&mut sprite);

        flipped.modify(&|(x, y), _| (x + y) as _);

        let (width, height) = sprite.dimensions();
        for x in 0..width {
            for y in 0..height {
                let expected = WIDTH as u8 - x as u8 + y as u8 - 1;
                assert_eq!(sprite.pixel((x as _, y as _)).copied(), Some(expected));
            }
        }
    }
}
