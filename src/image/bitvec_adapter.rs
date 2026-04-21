//! An adapter to store [`BitSlice`] data and use it as a `1BPP` pixel storage.
//!
//! ```rust
//! # use ugly::image::bitvec_adapter::Adapter;
//! # use bitvec::bitarr;
//! # const WIDTH: usize = 128;
//! # const HEIGHT: usize = 64;
//! #
//! fn main() {
//!    let mut data = bitarr![0; WIDTH * HEIGHT];
//!    let adapter = Adapter::new_mut(&mut data, WIDTH as _).unwrap();
//! }

pub use bitvec;

use bitvec::slice::BitSlice;

use crate::image::{Dimensions, Image, ImageMut};
use crate::strategy::Modify;

/// An adapter over `T`.
///
/// Implementations are for the following `T`:
///
/// - `&BitSLice` - immutable [`Image`]-only operations;
/// - `&mut BitSlice` - mutable [`Image`] and [`ImageMut`] operations.
pub struct Adapter<T> {
    data: T,
    width: u32,
    height: u32,
}

impl<'a> Adapter<&'a BitSlice> {
    /// Create new [`Adapter`] instance with immutable access.
    ///
    /// Returns `None` if `data`'s length is not a multiple of `width`.
    pub fn new(data: &'a BitSlice, width: u32) -> Option<Self> {
        let len = data.len() as u32;
        if !len.is_multiple_of(width) {
            return None;
        }
        let height = len / width;
        Some(Self {
            data,
            width,
            height,
        })
    }
}

impl<'a> Adapter<&'a mut BitSlice> {
    /// Create new [`Adapter`] instance with mutable access.
    ///
    /// Returns `None` if `data`'s length is not a multiple of `width`.
    pub fn new_mut(data: &'a mut BitSlice, width: u32) -> Option<Self> {
        let len = data.len() as u32;
        if !len.is_multiple_of(width) {
            return None;
        }
        let height = len / width;
        Some(Self {
            data,
            width,
            height,
        })
    }
}

impl<T> Dimensions for Adapter<T> {
    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl Image for Adapter<&BitSlice> {
    type Pixel = bool;

    fn pixel(&self, (x, y): (u32, u32)) -> Option<Self::Pixel> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = (x + y * self.width) as usize;
        self.data.get(index).as_deref().cloned()
    }
}

impl Image for Adapter<&mut BitSlice> {
    type Pixel = bool;

    fn pixel(&self, (x, y): (u32, u32)) -> Option<Self::Pixel> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = (x + y * self.width) as usize;
        self.data.get(index).as_deref().cloned()
    }
}

impl ImageMut for Adapter<&mut BitSlice> {
    type Pixel = bool;

    fn set_pixel(&mut self, (x, y): (u32, u32), value: Self::Pixel) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = (x + y * self.width) as usize;
        self.data.set(index, value);
    }

    fn modify_pixel(&mut self, (x, y): (u32, u32), function: Modify<Self::Pixel>) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = (x + y * self.width) as usize;
        if let Some(mut pixel) = self.data.get_mut(index) {
            *pixel = function(*pixel);
        }
    }

    fn set_horizontal_line(&mut self, (x, y): (u32, u32), total: u32, value: Self::Pixel) {
        if x >= self.width || y >= self.height {
            return;
        }
        let start = (x + y * self.width) as usize;
        let end = ((x + total).min(self.width) + y * self.width) as usize;
        if let Some(slice) = self.data.get_mut(start..end) {
            slice.fill(value);
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
                .for_each(|mut pixel| *pixel = function(*pixel));
        }
    }

    fn set(&mut self, value: Self::Pixel) {
        self.data.fill(value);
    }

    fn modify(&mut self, function: Modify<Self::Pixel>) {
        self.data
            .iter_mut()
            .for_each(|mut pixel| *pixel = function(*pixel));
    }
}
