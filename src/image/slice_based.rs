//! A slice-based pixel storage.
//!
//! Uses any [`Deref<Target = [P]>`](core::ops::Deref) as pixel storage.
//!
//! ```rust
//! # use ugly_graphics::image::slice_based::SliceBased;
//! # use ugly_graphics::image::{Image as _, ImageMut as _};
//! fn main() {
//!     let data = vec![0; 32 * 16];
//!     let mut slice_based = SliceBased::new(data, 32).unwrap();
//!     slice_based.set_pixel((1, 1), 4);
//!     assert_eq!(slice_based.pixel((1, 1)), Some(4));
//! }
//! ```

use core::ops;

use crate::image::{Dimensions, Image, ImageMut};
use crate::strategy::Modify;

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

impl<T, P> ImageMut for SliceBased<T>
where
    T: ops::DerefMut<Target = [P]>,
    P: Clone,
{
    type Pixel = P;

    fn set_pixel(&mut self, (x, y): (u32, u32), value: Self::Pixel) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = (x + y * self.width) as usize;
        if let Some(pixel) = self.data.get_mut(index) {
            *pixel = value;
        }
    }

    fn modify_pixel(&mut self, (x, y): (u32, u32), function: Modify<Self::Pixel>) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = (x + y * self.width) as usize;
        if let Some(pixel) = self.data.get_mut(index) {
            *pixel = function(pixel.clone());
        }
    }

    fn set_horizontal_line(&mut self, (x, y): (u32, u32), total: u32, value: Self::Pixel) {
        if x >= self.width || y >= self.height {
            return;
        }
        let start = (x + y * self.width) as usize;
        let end = ((x + total).min(self.width) + y * self.width) as usize;
        if let Some(slice) = self.data.get_mut(start..end) {
            slice.fill_with(|| value.clone());
        }
    }

    fn modify_horizontal_line(
        &mut self,
        (x, y): (u32, u32),
        total: u32,
        function: Modify<Self::Pixel>,
    ) {
        if x >= self.width || y >= self.height {
            return;
        }
        let start = (x + y * self.width) as usize;
        let end = ((x + total).min(self.width) + y * self.width) as usize;
        if let Some(slice) = self.data.get_mut(start..end) {
            slice
                .iter_mut()
                .for_each(|pixel| *pixel = function(pixel.clone()));
        }
    }

    fn set(&mut self, value: Self::Pixel) {
        self.data.fill(value);
    }

    fn modify(&mut self, function: Modify<Self::Pixel>) {
        self.data
            .iter_mut()
            .for_each(|pixel| *pixel = function(pixel.clone()));
    }
}
