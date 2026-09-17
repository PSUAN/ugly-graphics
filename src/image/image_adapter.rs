//! An adapter to [`ImageBuffer`].
//!
//! ```rust
//! # use ugly_graphics::image::image_adapter::Adapter;
//! # use ugly_graphics::image::ImageMut;
//! # use ugly_graphics::strategy;
//! # use image::{ImageBuffer, Rgb};
//! fn main() {
//!     let mut image = ImageBuffer::new(320, 320);
//!     let mut adapter = Adapter::new(&mut image);
//!     adapter.write_pixel((1, 1), &strategy::overwrite(Rgb([0xff, 0xff, 0xff])));
//! }
//! ```

pub use image;

use core::ops::{Deref, DerefMut};

use image::ImageBuffer;

use crate::strategy::{Apply, Overwrite};

use super::{Dimensions, Image, ImageMut};

/// An adapter over `T`.
///
/// Implementations are for the following `T`:
///
/// - `&ImageBuffer` - immutable [`Image`]-only operations;
/// - `&mut ImageBuffer` - mutable [`Image`] and [`ImageMut`] operations.
pub struct Adapter<T> {
    buffer: T,
}

impl<T> Adapter<T> {
    /// Create new [`Adapter`] instance.
    pub fn new(buffer: T) -> Self {
        Self { buffer }
    }
}

impl<P, C> Dimensions for Adapter<&ImageBuffer<P, C>>
where
    P: image::Pixel,
    C: Deref<Target = [P::Subpixel]>,
{
    fn dimensions(&self) -> (u32, u32) {
        self.buffer.dimensions()
    }
}

impl<P, C> Dimensions for Adapter<&mut ImageBuffer<P, C>>
where
    P: image::Pixel,
    C: Deref<Target = [P::Subpixel]>,
{
    fn dimensions(&self) -> (u32, u32) {
        self.buffer.dimensions()
    }
}

impl<P, C> Image for Adapter<&ImageBuffer<P, C>>
where
    P: image::Pixel,
    C: Deref<Target = [P::Subpixel]>,
{
    type Pixel = P;

    fn pixel(&self, (x, y): (u32, u32)) -> Option<Self::Pixel> {
        self.buffer.get_pixel_checked(x, y).cloned()
    }
}

impl<P, C> Image for Adapter<&mut ImageBuffer<P, C>>
where
    P: image::Pixel,
    C: Deref<Target = [P::Subpixel]>,
{
    type Pixel = P;

    fn pixel(&self, (x, y): (u32, u32)) -> Option<Self::Pixel> {
        self.buffer.get_pixel_checked(x, y).cloned()
    }
}

impl<P, C> ImageMut<Overwrite<P>> for Adapter<&mut ImageBuffer<P, C>>
where
    P: image::Pixel,
    C: DerefMut<Target = [P::Subpixel]>,
{
    fn write_pixel(&mut self, (x, y): (u32, u32), Overwrite(value): &Overwrite<P>) {
        if let Some(pixel) = self.buffer.get_pixel_mut_checked(x, y) {
            *pixel = *value;
        }
    }

    fn write_horizontal_line(&mut self, (x, y): (u32, u32), total: u32, value: &Overwrite<P>) {
        let (width, heignt) = self.dimensions();
        if y >= heignt {
            return;
        }
        for x in x..(x + total).min(width) {
            self.write_pixel((x, y), value);
        }
    }

    fn write(&mut self, Overwrite(value): &Overwrite<P>) {
        self.buffer.pixels_mut().for_each(|p| *p = *value);
    }
}

impl<P, C> ImageMut<Apply<'_, P>> for Adapter<&mut ImageBuffer<P, C>>
where
    P: image::Pixel,
    C: DerefMut<Target = [P::Subpixel]>,
{
    fn write_pixel(&mut self, (x, y): (u32, u32), Apply(value): &Apply<P>) {
        if let Some(pixel) = self.buffer.get_pixel_mut_checked(x, y) {
            *pixel = value(*pixel);
        }
    }

    fn write_horizontal_line(&mut self, (x, y): (u32, u32), total: u32, value: &Apply<P>) {
        let (width, heignt) = self.dimensions();
        if y >= heignt {
            return;
        }
        for x in x..(x + total).min(width) {
            self.write_pixel((x, y), value);
        }
    }

    fn write(&mut self, Apply(value): &Apply<P>) {
        self.buffer.pixels_mut().for_each(|p| *p = value(*p));
    }
}
