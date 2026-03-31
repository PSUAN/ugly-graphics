use crate::strategy::Modify;

pub mod slice_based;
pub mod sprite;

#[cfg(feature = "image-adapter")]
pub mod image_adapter;

#[cfg(feature = "bitvec-adapter")]
pub mod bitvec_adapter;

pub trait Dimensions {
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

pub trait Image: Dimensions {
    type Pixel;

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

pub trait ImageMut: Dimensions {
    type Pixel;

    fn set_pixel(&mut self, position: (u32, u32), value: Self::Pixel);
    fn modify_pixel(&mut self, position: (u32, u32), function: Modify<Self::Pixel>);

    fn set_horizontal_line(&mut self, position: (u32, u32), total: u32, value: Self::Pixel);
    fn modify_horizontal_line(
        &mut self,
        position: (u32, u32),
        total: u32,
        function: Modify<Self::Pixel>,
    );

    fn set(&mut self, value: Self::Pixel);
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
