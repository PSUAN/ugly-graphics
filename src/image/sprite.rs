//! A compile-time sized array-based pixel storage.
//!
//! ```rust
//! # use ugly_graphics::image::sprite::Sprite;
//! # use ugly_graphics::image::{Image as _, ImageMut as _};
//! # use ugly_graphics::strategy;
//! fn main() {
//!     let mut sprite = Sprite::<_, 4, 4>::from_copies(b' ');
//!     sprite.write_pixel((3, 3), &strategy::overwrite(b'!'));
//!     assert_eq!(sprite.pixel((3, 3)), Some(b'!'));
//! }
//! ```

use crate::image::{Dimensions, Image, ImageMut};
use crate::strategy::{Apply, Overwrite};

/// A compile-time sized array-based pixel storage.
///
/// Current implementation is backed by a two-dimensional array.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sprite<P, const W: usize, const H: usize> {
    data: [[P; W]; H],
}

impl<P, const W: usize, const H: usize> Sprite<P, W, H> {
    /// Create a new [`Sprite`] instance using [`Copy`] trait.
    pub fn from_copies(value: P) -> Self
    where
        P: Copy,
    {
        let data = [[value; W]; H];
        Self { data }
    }

    /// Create a new [`Sprite`] instance using raw `data`.
    pub fn from_raw(data: [[P; W]; H]) -> Self {
        Self { data }
    }

    /// Extract stored `data`.
    pub fn into_owned(self) -> [[P; W]; H] {
        self.data
    }

    /// Get stored `data` as a reference.
    pub fn data(&self) -> &[[P; W]; H] {
        &self.data
    }

    /// Get stored `data` as a mutable reference.
    pub fn data_mut(&mut self) -> &mut [[P; W]; H] {
        &mut self.data
    }
}

impl<P, const W: usize, const H: usize> Dimensions for Sprite<P, W, H> {
    fn dimensions(&self) -> (u32, u32) {
        (W as _, H as _)
    }
}

impl<P, const W: usize, const H: usize> Image for Sprite<P, W, H>
where
    P: Clone,
{
    type Pixel = P;

    fn pixel(&self, (x, y): (u32, u32)) -> Option<P> {
        let (x, y) = (usize::try_from(x).ok()?, usize::try_from(y).ok()?);
        self.data.get(y)?.get(x).cloned()
    }
}

impl<P, const W: usize, const H: usize> ImageMut<Overwrite<P>> for Sprite<P, W, H>
where
    P: Clone,
{
    fn write_pixel(&mut self, (x, y): (u32, u32), Overwrite(value): &Overwrite<P>) {
        let indices = (usize::try_from(x), usize::try_from(y));
        if let (Ok(x), Ok(y)) = indices
            && let Some(row) = self.data.get_mut(y)
            && let Some(pixel) = row.get_mut(x)
        {
            *pixel = value.clone();
        }
    }

    fn write_horizontal_line(
        &mut self,
        (x, y): (u32, u32),
        total: u32,
        Overwrite(value): &Overwrite<P>,
    ) {
        let indices = (usize::try_from(x), usize::try_from(y));
        let total = usize::try_from(total);
        if let (Ok(index_x), Ok(index_y)) = indices
            && let Ok(total) = total
            && let Some(row) = self.data.get_mut(index_y)
            && let Some(slice) = row.get_mut(index_x..(index_x + total).min(W as _))
        {
            slice.fill_with(|| value.clone());
        }
    }

    fn write(&mut self, Overwrite(value): &Overwrite<P>) {
        for row in self.data.iter_mut() {
            row.fill_with(|| value.clone());
        }
    }
}

impl<P, const W: usize, const H: usize> ImageMut<Apply<'_, P>> for Sprite<P, W, H>
where
    P: Clone,
{
    fn write_pixel(&mut self, (x, y): (u32, u32), Apply(value): &Apply<P>) {
        let indices = (usize::try_from(x), usize::try_from(y));
        if let (Ok(x), Ok(y)) = indices
            && let Some(row) = self.data.get_mut(y)
            && let Some(pixel) = row.get_mut(x)
        {
            *pixel = value(pixel.clone());
        }
    }

    fn write_horizontal_line(&mut self, (x, y): (u32, u32), total: u32, Apply(value): &Apply<P>) {
        let indices = (usize::try_from(x), usize::try_from(y));
        let total = usize::try_from(total);
        if let (Ok(index_x), Ok(index_y)) = indices
            && let Ok(total) = total
            && let Some(row) = self.data.get_mut(index_y)
            && let Some(slice) = row.get_mut(index_x..(index_x + total).min(W as _))
        {
            slice
                .iter_mut()
                .for_each(|pixel| *pixel = value(pixel.clone()));
        }
    }

    fn write(&mut self, Apply(value): &Apply<P>) {
        for row in self.data.iter_mut() {
            row.iter_mut()
                .for_each(|pixel| *pixel = value(pixel.clone()));
        }
    }
}

#[cfg(test)]
mod test {
    use crate::strategy;

    use super::*;

    #[test]
    fn horizontal_line_is_being_set_even_out_of_bounds() {
        let mut sprite = Sprite::<u8, 4, 3>::from_copies(0x00);
        sprite.write_horizontal_line((2, 1), 5, &strategy::overwrite(0xff));

        let expected = Sprite::from_raw([[0x00; 4], [0x00, 0x00, 0xff, 0xff], [0x00; 4]]);

        assert_eq!(sprite, expected);
    }

    #[test]
    fn horizontal_line_is_being_set_properly() {
        let mut sprite = Sprite::<u8, 6, 3>::from_copies(0x00);
        sprite.write_horizontal_line((1, 1), 3, &strategy::overwrite(0x80));

        let expected =
            Sprite::from_raw([[0x00; 6], [0x00, 0x80, 0x80, 0x80, 0x00, 0x00], [0x00; 6]]);

        assert_eq!(sprite, expected);
    }

    #[test]
    fn horizontal_line_is_being_modified_even_out_of_bounds() {
        let mut sprite = Sprite::<u8, 4, 3>::from_copies(0x00);
        sprite.write_horizontal_line((2, 1), 5, &strategy::apply(&|_| 0xff));

        let expected = Sprite::from_raw([[0x00; 4], [0x00, 0x00, 0xff, 0xff], [0x00; 4]]);

        assert_eq!(sprite, expected);
    }

    #[test]
    fn horizontal_line_is_being_modified_properly() {
        let mut sprite = Sprite::<u8, 6, 3>::from_copies(0x00);
        sprite.write_horizontal_line((1, 1), 3, &strategy::apply(&|_| 0x80));

        let expected =
            Sprite::from_raw([[0x00; 6], [0x00, 0x80, 0x80, 0x80, 0x00, 0x00], [0x00; 6]]);

        assert_eq!(sprite, expected);
    }
}
