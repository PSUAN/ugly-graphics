use crate::image::{Dimensions, Image};

pub struct Mapped<T, F> {
    mapper: F,
    target: T,
}

impl<T, F> Mapped<T, F> {
    pub fn new(target: T, mapper: F) -> Self {
        Self { mapper, target }
    }

    pub fn extract(self) -> T {
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
