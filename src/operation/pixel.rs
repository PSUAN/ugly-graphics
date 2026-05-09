//! [`Pixel`] applies a [`Strategy`] to one pixel at the given position.

use crate::painter::DrawRegion;
use crate::strategy::Strategy;

use super::Operation;

/// Apply [`Strategy`] at the desired position.
#[derive(Clone, Copy)]
pub struct Pixel<'a, P> {
    position: (i32, i32),
    value: Strategy<'a, P>,
}

impl<'a, P> Pixel<'a, P> {
    /// Create new instance to apply `value` at the `position`.
    pub fn new(position: (i32, i32), value: Strategy<'a, P>) -> Self {
        Self { position, value }
    }
}

impl<'a, P> Operation<P> for Pixel<'a, P>
where
    P: Clone,
{
    type Output = ();

    fn draw_on(self, painter: &mut DrawRegion<'_, '_, P>) -> Self::Output {
        painter.pixel(self.position, &self.value);
    }
}
