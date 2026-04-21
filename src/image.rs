//! Pixel storage abstractions.

use crate::strategy::Modify;

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
    T: Dimensions,
{
    fn dimensions(&self) -> (u32, u32) {
        Dimensions::dimensions(*self)
    }
}

impl<T> Dimensions for &mut T
where
    T: Dimensions,
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
    T: Image,
{
    type Pixel = T::Pixel;

    fn pixel(&self, position: (u32, u32)) -> Option<Self::Pixel> {
        Image::pixel(*self, position)
    }
}

impl<T> Image for &mut T
where
    T: Image,
{
    type Pixel = T::Pixel;

    fn pixel(&self, position: (u32, u32)) -> Option<Self::Pixel> {
        Image::pixel(*self, position)
    }
}

/// A pixel container providing pixel modification operations.
pub trait ImageMut: Dimensions {
    /// Stored pixel data.
    type Pixel;

    /// Overwrite a pixel at the given `position`.
    ///
    /// May fail silently if out of bounds or due to any other
    /// implementation-specific case.
    fn set_pixel(&mut self, position: (u32, u32), value: Self::Pixel);

    /// Overwrite a pixel at the given `position` using the provided `function`
    /// to compute new value.
    ///
    /// May fail silently if out of bounds or due to any other
    /// implementation-specific case.
    fn modify_pixel(&mut self, position: (u32, u32), function: Modify<Self::Pixel>);

    /// Overwrite a `total` amount of pixels starting at the given `position`.
    fn set_horizontal_line(&mut self, position: (u32, u32), total: u32, value: Self::Pixel);

    /// Overwrite a `total` amount of pixels starting at the given `position`
    /// using the provided `function` to compute new values.
    fn modify_horizontal_line(
        &mut self,
        position: (u32, u32),
        total: u32,
        function: Modify<Self::Pixel>,
    );

    /// Overwrite all pixels with the given `value`.
    fn set(&mut self, value: Self::Pixel);

    /// Modify each pixel using the provided `function` to compute new values.
    fn modify(&mut self, function: Modify<Self::Pixel>);
}

impl<T> ImageMut for &mut T
where
    T: ImageMut,
{
    type Pixel = T::Pixel;

    fn set_pixel(&mut self, position: (u32, u32), value: Self::Pixel) {
        ImageMut::set_pixel(*self, position, value);
    }

    fn modify_pixel(&mut self, position: (u32, u32), function: Modify<Self::Pixel>) {
        ImageMut::modify_pixel(*self, position, function);
    }

    fn set_horizontal_line(&mut self, position: (u32, u32), plus: u32, value: Self::Pixel) {
        ImageMut::set_horizontal_line(*self, position, plus, value);
    }

    fn modify_horizontal_line(
        &mut self,
        position: (u32, u32),
        plus: u32,
        function: Modify<Self::Pixel>,
    ) {
        ImageMut::modify_horizontal_line(*self, position, plus, function);
    }

    fn set(&mut self, value: Self::Pixel) {
        ImageMut::set(*self, value);
    }

    fn modify(&mut self, function: Modify<Self::Pixel>) {
        ImageMut::modify(*self, function);
    }
}
