//! Pixel storage abstractions.

use crate::strategy::{Apply, Overwrite};

pub mod slice_based;
pub mod sprite;

#[cfg(feature = "image-adapter")]
pub mod image_adapter;

#[cfg(feature = "bitvec-adapter")]
pub mod bitvec_adapter;

/// Something that has width and height.
pub trait Dimensions {
    /// Get width and height.
    fn dimensions(&self) -> (u32, u32);
}

impl<T> Dimensions for &T
where
    T: Dimensions + ?Sized,
{
    fn dimensions(&self) -> (u32, u32) {
        Dimensions::dimensions(*self)
    }
}

impl<T> Dimensions for &mut T
where
    T: Dimensions + ?Sized,
{
    fn dimensions(&self) -> (u32, u32) {
        Dimensions::dimensions(*self)
    }
}

/// A pixel container that may return pixel at given coordinates.
///
/// It is considered a good practice to return `Some` pixel for every position
/// inside dimensions, but that is not enforced.
pub trait Image: Dimensions {
    /// Stored pixel data.
    type Pixel;

    /// Try reading a pixel given its coordinates.
    fn pixel(&self, position: (u32, u32)) -> Option<Self::Pixel>;
}

impl<T> Image for &T
where
    T: Image + ?Sized,
{
    type Pixel = T::Pixel;

    fn pixel(&self, position: (u32, u32)) -> Option<Self::Pixel> {
        Image::pixel(*self, position)
    }
}

impl<T> Image for &mut T
where
    T: Image + ?Sized,
{
    type Pixel = T::Pixel;

    fn pixel(&self, position: (u32, u32)) -> Option<Self::Pixel> {
        Image::pixel(*self, position)
    }
}

/// A pixel container providing pixel modification operations.
pub trait ImageMut<W> {
    /// Write pixel at the given `position`.
    ///
    /// May fail silently if out of bounds or due to any other
    /// implementation-specific case.
    fn write_pixel(&mut self, position: (u32, u32), writer: &W);

    /// Write a `total` amount of pixels starting at the given `position`.
    fn write_horizontal_line(&mut self, position: (u32, u32), total: u32, writer: &W);

    /// Write to every pixel.
    fn write(&mut self, writer: &W);
}

impl<T, W> ImageMut<W> for &mut T
where
    T: ImageMut<W> + ?Sized,
{
    fn write_pixel(&mut self, position: (u32, u32), writer: &W) {
        ImageMut::write_pixel(*self, position, writer);
    }

    fn write_horizontal_line(&mut self, position: (u32, u32), total: u32, writer: &W) {
        ImageMut::write_horizontal_line(*self, position, total, writer);
    }

    fn write(&mut self, writer: &W) {
        ImageMut::write(*self, writer);
    }
}

/// An [`ImageMut`] that supports both [`Overwrite`] and [`Apply`] operations.
pub trait ImageMutFull<P>:
    ImageMut<Overwrite<P>> + for<'a> ImageMut<Apply<'a, P>> + Dimensions
{
}

impl<T, P> ImageMutFull<P> for T where
    T: ImageMut<Overwrite<P>> + for<'a> ImageMut<Apply<'a, P>> + Dimensions
{
}
