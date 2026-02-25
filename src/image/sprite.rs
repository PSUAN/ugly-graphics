use crate::image::{Dimensions, Image, ImageMut};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sprite<P, const W: usize, const H: usize> {
    data: [[P; W]; H],
}

impl<P, const W: usize, const H: usize> Sprite<P, W, H> {
    pub fn from_copies(value: P) -> Self
    where
        P: Copy,
    {
        let data = [[value; W]; H];
        Self { data }
    }

    pub fn from_raw(data: [[P; W]; H]) -> Self {
        Self { data }
    }
}

impl<P, const W: usize, const H: usize> Dimensions for Sprite<P, W, H> {
    fn dimensions(&self) -> (i32, i32) {
        (W as _, H as _)
    }
}

impl<P, const W: usize, const H: usize> Image<P> for Sprite<P, W, H>
where
    P: Clone,
{
    fn pixel(&self, (x, y): (i32, i32)) -> Option<P> {
        let (x, y) = (usize::try_from(x).ok()?, usize::try_from(y).ok()?);
        self.data.get(y)?.get(x).cloned()
    }
}

impl<P, const W: usize, const H: usize> ImageMut<P> for Sprite<P, W, H>
where
    P: Clone,
{
    fn set_pixel(&mut self, (x, y): (i32, i32), value: P) {
        let indices = (usize::try_from(x), usize::try_from(y));
        if let (Ok(x), Ok(y)) = indices
            && let Some(row) = self.data.get_mut(y)
            && let Some(pixel) = row.get_mut(x)
        {
            *pixel = value;
        }
    }

    fn modify_pixel(&mut self, (x, y): (i32, i32), function: &dyn Fn((i32, i32), P) -> P) {
        let indices = (usize::try_from(x), usize::try_from(y));
        if let (Ok(index_x), Ok(index_y)) = indices
            && let Some(row) = self.data.get_mut(index_y)
            && let Some(pixel) = row.get_mut(index_x)
        {
            *pixel = function((x, y), pixel.clone());
        }
    }

    fn set_horizontal_line(&mut self, (x, y): (i32, i32), plus: u32, value: P) {
        let (x, plus) = if x < 0 {
            (0, plus - (-x) as u32)
        } else {
            (x, plus)
        };

        let indices = (usize::try_from(x), usize::try_from(y));
        let plus = usize::try_from(plus);
        if let (Ok(index_x), Ok(index_y)) = indices
            && let Ok(plus) = plus
            && let Some(row) = self.data.get_mut(index_y)
            && let Some(slice) = row.get_mut(index_x..(index_x + plus).min(W as _))
        {
            slice.fill_with(|| value.clone());
        }
    }

    fn modify_horizontal_line(
        &mut self,
        (x, y): (i32, i32),
        plus: u32,
        function: &dyn Fn((i32, i32), P) -> P,
    ) {
        let (x, plus) = if x < 0 {
            (0, plus - (-x) as u32)
        } else {
            (x, plus)
        };

        let indices = (usize::try_from(x), usize::try_from(y));
        let plus = usize::try_from(plus);
        if let (Ok(index_x), Ok(index_y)) = indices
            && let Ok(plus) = plus
            && let Some(row) = self.data.get_mut(index_y)
            && let Some(slice) = row.get_mut(index_x..(index_x + plus).min(W as _))
        {
            slice.iter_mut().enumerate().for_each(|(index, pixel)| {
                *pixel = function(((index + index_x) as _, y), pixel.clone());
            });
        }
    }

    fn set(&mut self, value: P) {
        for row in self.data.iter_mut() {
            row.fill_with(|| value.clone());
        }
    }

    fn modify(&mut self, function: &dyn Fn((i32, i32), P) -> P) {
        for (y, row) in self.data.iter_mut().enumerate() {
            let y = y as _;

            row.iter_mut()
                .enumerate()
                .for_each(|(index, pixel)| *pixel = function((index as _, y), pixel.clone()));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn horizontal_line_is_being_set_even_out_of_bounds() {
        let mut sprite = Sprite::<u8, 4, 3>::from_copies(0x00);
        sprite.set_horizontal_line((-3, 1), 5, 0xff);

        let expected = Sprite::from_raw([[0x00; 4], [0xff, 0xff, 0x00, 0x00], [0x00; 4]]);

        assert_eq!(sprite, expected);

        let mut sprite = Sprite::<u8, 4, 3>::from_copies(0x00);
        sprite.set_horizontal_line((2, 1), 5, 0xff);

        let expected = Sprite::from_raw([[0x00; 4], [0x00, 0x00, 0xff, 0xff], [0x00; 4]]);

        assert_eq!(sprite, expected);
    }
}
