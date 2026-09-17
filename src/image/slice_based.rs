//! A slice-based pixel storage.
//!
//! Uses any [`Deref<Target = [P]>`](core::ops::Deref) as pixel storage.
//!
//! ```rust
//! # use ugly_graphics::image::slice_based::SliceBased;
//! # use ugly_graphics::image::{Image as _, ImageMut as _};
//! # use ugly_graphics::strategy;
//! fn main() {
//!     let data = vec![0; 32 * 16];
//!     let mut slice_based = SliceBased::new(data, 32).unwrap();
//!     slice_based.write_pixel((1, 1), &strategy::overwrite(4));
//!     assert_eq!(slice_based.pixel((1, 1)), Some(4));
//! }
//! ```

use core::ops;

use crate::image::{Dimensions, Image, ImageMut};
use crate::strategy::{Apply, Overwrite};

/// Slice-based storage for pixels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SliceBased<T> {
    data: T,
    width: u32,
    height: u32,
}

impl<T> SliceBased<T> {
    /// Try constructing new instance using provided `data` and `width`.
    ///
    /// Returns `None` if `data`'s length is not a multiple of `width`.
    pub fn new<P>(data: T, width: u32) -> Option<Self>
    where
        T: ops::Deref<Target = [P]>,
    {
        let len = data.deref().len() as u32;
        if !len.is_multiple_of(width) {
            return None;
        };
        let height = len / width;
        Some(Self {
            data,
            width,
            height,
        })
    }

    /// Extract stored `data`.
    pub fn to_owned(self) -> T {
        self.data
    }

    /// Get internal `data` as a reference.
    pub fn data<P>(&self) -> &[P]
    where
        T: ops::Deref<Target = [P]>,
    {
        self.data.deref()
    }

    /// Get internal `data` as a mutable reference.
    pub fn data_mut<P>(&mut self) -> &mut [P]
    where
        T: ops::DerefMut<Target = [P]>,
    {
        self.data.deref_mut()
    }
}

impl<T> Dimensions for SliceBased<T> {
    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl<T, P> Image for SliceBased<T>
where
    T: ops::Deref<Target = [P]>,
    P: Clone,
{
    type Pixel = P;

    fn pixel(&self, (x, y): (u32, u32)) -> Option<Self::Pixel> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = (x + y * self.width) as usize;
        self.data.get(index).cloned()
    }
}

impl<T, P> ImageMut<Overwrite<P>> for SliceBased<T>
where
    T: ops::DerefMut<Target = [P]>,
    P: Clone,
{
    fn write_pixel(&mut self, (x, y): (u32, u32), Overwrite(value): &Overwrite<P>) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = (x + y * self.width) as usize;
        if let Some(pixel) = self.data.get_mut(index) {
            *pixel = value.clone();
        }
    }

    fn write_horizontal_line(
        &mut self,
        (x, y): (u32, u32),
        total: u32,
        Overwrite(value): &Overwrite<P>,
    ) {
        if x >= self.width || y >= self.height {
            return;
        }
        let start = (x + y * self.width) as usize;
        let end = ((x + total).min(self.width) + y * self.width) as usize;
        if let Some(slice) = self.data.get_mut(start..end) {
            slice.fill_with(|| value.clone());
        }
    }

    fn write(&mut self, Overwrite(value): &Overwrite<P>) {
        self.data.fill(value.clone());
    }
}

impl<T, P> ImageMut<Apply<'_, P>> for SliceBased<T>
where
    T: ops::DerefMut<Target = [P]>,
    P: Clone,
{
    fn write_pixel(&mut self, (x, y): (u32, u32), Apply(value): &Apply<P>) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = (x + y * self.width) as usize;
        if let Some(pixel) = self.data.get_mut(index) {
            *pixel = value(pixel.clone());
        }
    }

    fn write_horizontal_line(&mut self, (x, y): (u32, u32), total: u32, Apply(value): &Apply<P>) {
        if x >= self.width || y >= self.height {
            return;
        }
        let start = (x + y * self.width) as usize;
        let end = ((x + total).min(self.width) + y * self.width) as usize;
        if let Some(slice) = self.data.get_mut(start..end) {
            slice.iter_mut().for_each(|p| *p = value(p.clone()));
        }
    }

    fn write(&mut self, Apply(value): &Apply<P>) {
        self.data.iter_mut().for_each(|p| *p = value(p.clone()));
    }
}

#[cfg(test)]
mod test {
    use crate::strategy;

    use super::*;

    #[test]
    fn raw_access() {
        let mut data = [0x00u8; 256];
        let data = &mut data as &mut [u8];
        let mut slice_based = super::SliceBased::new(data, 16).unwrap();

        slice_based.write(&strategy::overwrite(0x80));

        assert!(slice_based.data().iter().all(|v| *v == 0x80));
    }
}
