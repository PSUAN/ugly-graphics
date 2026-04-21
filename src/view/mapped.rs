//! The [`Mapped`] view provides `mapper` application to the pixels being read.

use crate::image::{Dimensions, Image};

/// The view that applies the `mapper` function to the pixels being read.
pub struct Mapped<T, F> {
    mapper: F,
    target: T,
}

impl<T, F> Mapped<T, F> {
    /// Create new instance using the provided `mapper`.
    pub fn new(target: T, mapper: F) -> Self {
        Self { mapper, target }
    }

    /// Extract stored `target` value.
    pub fn into_owned(self) -> T {
        self.target
    }
}

impl<T, F> Dimensions for Mapped<T, F>
where
    T: Dimensions,
{
    fn dimensions(&self) -> (u32, u32) {
        self.target.dimensions()
    }
}

impl<T, F, O> Image for Mapped<T, F>
where
    T: Image,
    F: Fn(T::Pixel) -> O,
{
    type Pixel = O;

    fn pixel(&self, position: (u32, u32)) -> Option<O> {
        Some((self.mapper)(self.target.pixel(position)?))
    }
}
