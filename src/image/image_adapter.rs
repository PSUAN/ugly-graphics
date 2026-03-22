use core::ops::{Deref, DerefMut};

use image::ImageBuffer;

use crate::strategy::Modify;

use super::{Dimensions, Image, ImageMut};

pub struct Adapter<T> {
    buffer: T,
}

impl<T> Adapter<T> {
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

impl<P, C> ImageMut for Adapter<&mut ImageBuffer<P, C>>
where
    P: image::Pixel,
    C: DerefMut<Target = [P::Subpixel]>,
{
    type Pixel = P;

    fn set_pixel(&mut self, (x, y): (u32, u32), value: Self::Pixel) {
        if let Some(pixel) = self.buffer.get_pixel_mut_checked(x, y) {
            *pixel = value;
        }
    }

    fn modify_pixel(&mut self, (x, y): (u32, u32), function: Modify<Self::Pixel>) {
        if let Some(pixel) = self.buffer.get_pixel_mut_checked(x, y) {
            *pixel = function(*pixel);
        }
    }

    fn set_horizontal_line(&mut self, (x, y): (u32, u32), total: u32, value: Self::Pixel) {
        let (width, heignt) = self.dimensions();
        if y >= heignt {
            return;
        }
        for x in x..(x + total).min(width) {
            self.set_pixel((x, y), value);
        }
    }

    fn modify_horizontal_line(
        &mut self,
        (x, y): (u32, u32),
        total: u32,
        function: Modify<Self::Pixel>,
    ) {
        let (width, heignt) = self.dimensions();
        if y >= heignt {
            return;
        }
        for x in x..(x + total).min(width) {
            self.modify_pixel((x, y), function);
        }
    }

    fn set(&mut self, value: Self::Pixel) {
        self.buffer.pixels_mut().for_each(|p| *p = value);
    }

    fn modify(&mut self, function: Modify<Self::Pixel>) {
        self.buffer.pixels_mut().for_each(|p| *p = function(*p));
    }
}
