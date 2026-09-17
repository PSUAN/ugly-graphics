//! [`Pixel`] applies a [`Strategy`] to one pixel at the given position.

use crate::image::ImageMut;
use crate::painter::DrawRegion;

use super::Operation;

/// Apply given pixel at the desired position.
#[derive(Clone, Copy)]
pub struct Pixel<P> {
    position: (i32, i32),
    value: P,
}

impl<P> Pixel<P> {
    /// Create new instance to apply `value` at the `position`.
    pub fn new(position: (i32, i32), value: P) -> Self {
        Self { position, value }
    }
}

impl<T, P> Operation<T> for Pixel<P>
where
    T: ImageMut<P>,
{
    type Output = ();

    fn draw_on(self, painter: &mut DrawRegion<'_, T>) -> Self::Output {
        painter.pixel(self.position, &self.value);
    }
}
