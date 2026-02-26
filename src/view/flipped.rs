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
    pub fn new(target: T, direction: Flip) -> Self {
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
    fn pixel(&self, position: (i32, i32)) -> Option<C> {
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

    fn set_horizontal_line(&mut self, position: (i32, i32), total: u32, value: C) {
        let (x, y) = self.transformer()(position);
        let x = match self.direction {
            Flip::Horizontal => x - total as i32 + 1,
            Flip::Vertical => x,
        };
        self.target.set_horizontal_line((x, y), total, value);
    }

    fn modify_horizontal_line(
        &mut self,
        position: (i32, i32),
        total: u32,
        function: &dyn Fn((i32, i32), C) -> C,
    ) {
        let transformer = self.transformer();
        let (x, y) = transformer(position);
        let x = match self.direction {
            Flip::Horizontal => x - total as i32 + 1,
            Flip::Vertical => x,
        };
        self.target
            .modify_horizontal_line((x, y), total, &move |position, pixel| {
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
    use crate::image::sprite::Sprite;

    use super::*;

    #[test]
    fn flip_stores_sprite_ref_properly() {
        let sprite = Sprite::<u8, _, _>::from_raw([
            [0x00, 0x01, 0x02], //
            [0x10, 0x11, 0x12], //
        ]);
        let flipped = Flipped::horizontal(&sprite);

        assert_eq!(flipped.pixel((0, 0)), Some(0x02));
        assert_eq!(flipped.pixel((1, 1)), Some(0x11));
        assert_eq!(flipped.pixel((2, 2)), None);
    }

    #[test]
    fn flip_stores_sprite_mut_properly() {
        let mut sprite = Sprite::<u8, _, _>::from_raw([
            [0x00, 0x01, 0x02], //
            [0x10, 0x11, 0x12], //
        ]);
        let flipped = Flipped::horizontal(&mut sprite);

        assert_eq!(flipped.pixel((0, 0)), Some(0x02));
        assert_eq!(flipped.pixel((1, 1)), Some(0x11));
        assert_eq!(flipped.pixel((2, 2)), None);
    }

    #[test]
    fn flip_set_line_works_properly() {
        let mut sprite = Sprite::<u8, 4, 4>::from_copies(0x00);
        let mut flipped = Flipped::horizontal(&mut sprite);

        flipped.set_horizontal_line((-2, 1), 4, 0x20);
        flipped.set_horizontal_line((2, 2), 4, 0x30);
        flipped.set_horizontal_line((1, 3), 2, 0x40);

        let expected = Sprite::from_raw([
            [0x00; 4],
            [0x00, 0x00, 0x20, 0x20],
            [0x30, 0x30, 0x00, 0x00],
            [0x00, 0x40, 0x40, 0x00],
        ]);

        assert_eq!(sprite, expected);
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
                assert_eq!(sprite.pixel((x as _, y as _)), Some(expected));
            }
        }
    }
}
